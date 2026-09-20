use crate::numbers::zero;
use patterns::{because, source};
use serde::{Deserialize, Serialize};

pub struct Fnv;
source!(Fnv, "the FNV hash of Fowler, Noll and Vo in its form over a double machine word, each code unit mixed in before the multiply, \
     taken over the code units a feature is written in");

const FNV_BASIS: FeatureId = 14_695_981_039_346_656_037;
because!(FNV_BASIS, Fnv, "the offset basis the hash over a double machine word starts from");

const FNV_PRIME: FeatureId = 1_099_511_628_211;
because!(FNV_PRIME, Fnv, "the prime the hash over a double machine word multiplies by after each code unit");

pub struct StackFeatures;
source!(
    StackFeatures,
    "the features the first network over the stack reads: the word, where it stands in the vocabulary, whether a question is read, the \
     behavior and the word before it, the waiting pointer and the focus, how deep the stack is, the newest groups one by one and whether \
     the word names a group on the stack, with the word crossed with the behavior before it, the pointer and that last test"
);

pub type FeatureId = u64;
because!(FeatureId, StackFeatures, "the place of a feature in the hashed feature space, a whole double machine word: the user wants the \
     stack the network reads as exact as it can be, since the stack decides every step on the permanent memory, and a narrower place gave \
     two words the stack holds, as japan and earth, the same features, so the network could not tell them apart");

pub const FEATURE_BITS: u32 = FeatureId::BITS;
because!(FEATURE_BITS, StackFeatures, "the width in bits of the hashed feature space: the whole hash, none of it cut away, so no two \
     features the stack holds share a place in practice");

struct Folded(FeatureId);
because!(Folded, "a hash folded over a text while the text is written, piece by piece, so no string is made for it and the hash is the one \
     the whole text folds to");

impl std::fmt::Write for Folded {
    fn write_str(&mut self, piece: &str) -> std::fmt::Result {
        self.0 = piece.encode_utf16().fold(self.0, |h, u| (h ^ FeatureId::from(u)).wrapping_mul(FNV_PRIME));
        Ok(())
    }
}

pub fn slot_written(text: std::fmt::Arguments) -> FeatureId {
    let mut folded = Folded(FNV_BASIS);
    std::fmt::Write::write_fmt(&mut folded, text).ok();
    folded.0
}
because!(slot_written, "the place of a feature in the hashed feature space from its text as it is written, without making the string");

pub fn slot(text: &str) -> FeatureId {
    slot_written(format_args!("{text}"))
}
because!(slot, "the place of a feature in the hashed feature space");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackShape {
    pub slots: usize,
    pub item: usize,
    pub hidden: usize,
    pub items: usize,
    #[serde(default)]
    pub pointer: bool,
}
because!(
    StackShape,
    "the shape of a network over the events as its classes file names it: how many slots it reads, the width of an event's embedding, how \
     many hidden units, how many feature places have an embedding in the weights, and whether it points at the slots it copies a word \
     from, none for a file written before it could"
);

#[derive(Clone, Debug)]
pub struct Stacked {
    pub slots: usize,
    pub item: usize,
    pub hidden: usize,
    pub table: std::collections::BTreeMap<FeatureId, Vec<f32>>,
    pub slot_weights: Vec<f32>,
    pub fill: Vec<f32>,
    pub hidden_bias: Vec<f32>,
    pub out: Vec<f32>,
    pub out_bias: Vec<f32>,
    pub point: Vec<f32>,
}
because!(
    Stacked,
    CopyPointer,
    "the user's network over the events: each slot's event is the sum of one shared embedding per feature place, a place without one \
     adding nothing; each slot's embedding goes through that slot's own block of weights into one hidden layer of rectified units, with a \
     weight for the slot being filled, so the slots are read side by side in order with the current event first and an empty slot is \
     nothing but its flag at zero, which costs nothing and is never read; the outputs, the behaviors with the settle and carry heads, are \
     a weighted sum of the hidden units; a network that copies words has one matrix shared by every slot from the hidden units to an \
     embedding, empty otherwise; stored little endian as the embeddings in rising order of their place, each led by its place, then the \
     slot blocks, the fill weights, the hidden biases, the output weights, the output biases and the pointing matrix"
);

pub struct CopyPointer;
source!(
    CopyPointer,
    "the user's plan to copy a reply word by pointing and not by the slot it stands in, after copy classes by slot turned tom into cats \
     when one slot was off: one copy output, and each slot that may be copied scored by the hidden units through one shared matrix against \
     the slot's own embedding, a softmax over the slots present, as pointer networks choose among their inputs (Vinyals, Fortunato and \
     Jaitly)"
);

pub struct Forward {
    pub embedded: Vec<f32>,
    pub filled: usize,
    pub pre: Vec<f32>,
    pub hidden: Vec<f32>,
    pub out: Vec<f32>,
}
because!(Forward, "one pass through a network over the events, kept whole for a trainer: every filled slot's embedding in order, how many \
     slots were filled, the hidden units before and after rectifying, and the outputs");

pub struct Passed {
    pub embedded: Vec<f32>,
    pub filled: usize,
    pub pre: Vec<f32>,
    pub hidden: Vec<f32>,
}
because!(Passed, "a pass through a network over the events up to its hidden units, for a trainer that computes only the outputs a row is \
     taught at: every filled slot's embedding in order, how many slots were filled, and the hidden units before and after rectifying");

impl Stacked {
    pub fn forward<'a>(&self, slots: impl Iterator<Item = &'a [FeatureId]>) -> Forward {
        let Passed { embedded, filled, pre, hidden } = self.passed(slots);
        let out = (0..self.out_bias.len()).map(|c| self.output_at(&hidden, c)).collect();
        Forward { embedded, filled, pre, hidden, out }
    }

    pub fn output_at(&self, hidden: &[f32], c: usize) -> f32 {
        self.out_bias[c] + self.out[c * self.hidden..c * self.hidden + self.hidden].iter().zip(hidden).map(|(w, h)| w * h).sum::<f32>()
    }

    pub fn passed<'a>(&self, slots: impl Iterator<Item = &'a [FeatureId]>) -> Passed {
        let block = self.hidden * self.item;
        let mut pre = self.hidden_bias.clone();
        let mut embedded: Vec<f32> = Vec::with_capacity(self.slots * self.item);
        let mut filled = 0;
        for (s, ids) in slots.take(self.slots).enumerate() {
            embedded.resize(embedded.len() + self.item, zero());
            let e = &mut embedded[s * self.item..];
            for v in ids.iter().filter_map(|id| self.table.get(id)) {
                for (x, w) in e.iter_mut().zip(v) {
                    *x += w;
                }
            }
            let e = &embedded[s * self.item..];
            let weights = &self.slot_weights[s * block..s * block + block];
            let flags = &self.fill[s * self.hidden..s * self.hidden + self.hidden];
            for ((z, row), flag) in pre.iter_mut().zip(weights.chunks_exact(self.item)).zip(flags) {
                *z += row.iter().zip(e).map(|(w, x)| w * x).sum::<f32>() + flag;
            }
            filled = s + 1;
        }
        let hidden: Vec<f32> = pre.iter().map(|z| z.max(zero())).collect();
        Passed { embedded, filled, pre, hidden }
    }

    pub fn pointing(&self, passed: &Passed, among: &[usize]) -> Vec<f32> {
        among
            .iter()
            .map(|&s| {
                let e = passed.embedded.get(s * self.item..(s + 1) * self.item).unwrap_or_default();
                self.point.chunks_exact(self.item).zip(&passed.hidden).map(|(row, h)| h * row.iter().zip(e).map(|(w, x)| w * x).sum::<f32>()).sum()
            })
            .collect()
    }

    pub fn shape(&self) -> StackShape {
        StackShape { slots: self.slots, item: self.item, hidden: self.hidden, items: self.table.len(), pointer: !self.point.is_empty() }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        for (id, v) in &self.table {
            out.extend(id.to_le_bytes());
            out.extend(v.iter().flat_map(|w| w.to_le_bytes()));
        }
        for w in self.slot_weights.iter().chain(&self.fill).chain(&self.hidden_bias).chain(&self.out).chain(&self.out_bias).chain(&self.point) {
            out.extend(w.to_le_bytes());
        }
        out
    }

    pub fn read_voters(weights: &[u8], shape: StackShape, outputs: usize, voters: usize) -> Result<Vec<Self>, String> {
        Self::read_voters_wide(weights, shape, outputs, voters, false)
    }

    pub fn read_voters_wide(weights: &[u8], shape: StackShape, outputs: usize, voters: usize, half: bool) -> Result<Vec<Self>, String> {
        if voters == 0 || weights.len() % voters != 0 {
            return Err(format!("the weights are {} bytes, which {voters} networks of one shape do not share evenly", weights.len()));
        }
        weights.chunks_exact(weights.len() / voters).map(|one| Self::read_wide(one, shape.clone(), outputs, half)).collect()
    }

    pub fn read(weights: &[u8], shape: StackShape, outputs: usize) -> Result<Self, String> {
        Self::read_wide(weights, shape, outputs, false)
    }

    pub fn read_wide(weights: &[u8], shape: StackShape, outputs: usize, half: bool) -> Result<Self, String> {
        if shape.slots == 0 || shape.item == 0 || shape.hidden == 0 {
            return Err("a network over the events needs slots, an embedding and hidden units".to_string());
        }
        let width = if half { std::mem::size_of::<u16>() } else { std::mem::size_of::<f32>() };
        let number = |c: &[u8]| if half { of_half(u16::from_le_bytes(bytes_of(c))) } else { f32::from_le_bytes(bytes_of(c)) };
        let record = std::mem::size_of::<FeatureId>() + shape.item * width;
        let block = shape.hidden * shape.item;
        let pointing = if shape.pointer { block } else { 0 };
        let dense = shape.slots * block + shape.slots * shape.hidden + shape.hidden + outputs * shape.hidden + outputs + pointing;
        let wanted = shape.items * record + dense * width;
        if weights.len() != wanted {
            return Err(format!("the network's weights are {} bytes, and its shape takes {wanted}", weights.len()));
        }
        let (head, rest) = weights.split_at(shape.items * record);
        let mut table = std::collections::BTreeMap::new();
        for r in head.chunks_exact(record) {
            let (place, floats) = r.split_at(std::mem::size_of::<FeatureId>());
            let id = FeatureId::from_le_bytes(bytes_of(place));
            if table.last_key_value().is_some_and(|(&last, _)| id <= last) {
                return Err(format!("the network's embeddings are not in rising order of their place at {id}"));
            }
            table.insert(id, floats.chunks_exact(width).map(&number).collect());
        }
        let floats: Vec<f32> = rest.chunks_exact(width).map(&number).collect();
        let (slot_weights, floats) = floats.split_at(shape.slots * block);
        let (fill, floats) = floats.split_at(shape.slots * shape.hidden);
        let (hidden_bias, floats) = floats.split_at(shape.hidden);
        let (out, floats) = floats.split_at(outputs * shape.hidden);
        let (out_bias, point) = floats.split_at(outputs);
        Ok(Self {
            slots: shape.slots,
            item: shape.item,
            hidden: shape.hidden,
            table,
            slot_weights: slot_weights.to_vec(),
            fill: fill.to_vec(),
            hidden_bias: hidden_bias.to_vec(),
            out: out.to_vec(),
            out_bias: out_bias.to_vec(),
            point: point.to_vec(),
        })
    }

    pub fn numbers(&self) -> usize {
        self.table.len() * self.item + self.slot_weights.len() + self.fill.len() + self.hidden_bias.len() + self.out.len() + self.out_bias.len() + self.point.len()
    }
}

pub struct HalfWeights;
source!(
    HalfWeights,
    "the half width number format of the institute of electrical and electronics engineers, in which a number is written in two bytes as a sign, five bits of exponent above a bias of fifteen \
     and ten bits of fraction, which the page reads the weights in: every weight of a trained network rounded to it and back scored \
     the same on every lesson, learned and held out"
);

const HALF_SIGN_BIT: u32 = 15;
because!(HALF_SIGN_BIT, HalfWeights, "the place of the sign in a weight written half as wide, as that format sets it");

const HALF_FRACTION: u32 = 10;
because!(HALF_FRACTION, HalfWeights, "how many of a half wide weight's bits are its fraction, the ones between it and the sign being its exponent");

const HALF_BIAS: u32 = 15;
because!(HALF_BIAS, HalfWeights, "what a half wide weight's exponent is written above, so an exponent of that value is one");

const HALF_SMALLEST: f32 = 5.960_464_5e-8;
because!(HALF_SMALLEST, HalfWeights, "the step between the half wide weights below the smallest one with an exponent, by which such a weight is read");

const SINGLE_SIGN_BIT: u32 = 31;
because!(SINGLE_SIGN_BIT, HalfWeights, "the place of the sign in a weight of the width the network works in");

const SINGLE_FRACTION: u32 = 23;
because!(SINGLE_FRACTION, HalfWeights, "how many of a weight's bits are its fraction at the width the network works in");

const SINGLE_BIAS: u32 = 127;
because!(SINGLE_BIAS, HalfWeights, "what a weight's exponent is written above at the width the network works in");

fn of_half(bits: u16) -> f32 {
    let bits = u32::from(bits);
    let sign = bits >> HALF_SIGN_BIT;
    let ones = |wide: u32| (1 << wide) - 1;
    let exponent = (bits >> HALF_FRACTION) & ones(HALF_SIGN_BIT - HALF_FRACTION);
    let fraction = bits & ones(HALF_FRACTION);
    let signed = sign << SINGLE_SIGN_BIT;
    if exponent == ones(HALF_SIGN_BIT - HALF_FRACTION) {
        return if sign == 0 { f32::INFINITY } else { f32::NEG_INFINITY };
    }
    if exponent == 0 {
        let small = fraction as f32 * HALF_SMALLEST;
        return if sign == 0 { small } else { -small };
    }
    f32::from_bits(signed | ((exponent + SINGLE_BIAS - HALF_BIAS) << SINGLE_FRACTION) | (fraction << (SINGLE_FRACTION - HALF_FRACTION)))
}
because!(of_half, HalfWeights, "one weight read from the two bytes a half wide file writes it in: its sign kept, its exponent written above the wider bias and its fraction shifted up, with the weights below the smallest exponent read by the smallest step and a full exponent read as beyond any weight");

fn bytes_of<const WIDTH: usize>(c: &[u8]) -> [u8; WIDTH] {
    <[u8; WIDTH]>::try_from(c).unwrap_or([0; WIDTH])
}
because!(bytes_of, "the bytes of one little endian number as wide as the number read from them, a single precision weight or a feature \
     place, all nothing when the piece is not that wide");

#[derive(Deserialize, Serialize)]
pub struct NetworkClasses {
    pub bits: u32,
    #[serde(default)]
    pub half: bool,
    pub classes: Vec<String>,
    pub stacked: StackShape,
    #[serde(default)]
    pub voters: usize,
}
because!(
    NetworkClasses,
    "the file beside a network's weights: the width of the hashed feature space its events' features are placed in, the outputs in order, \
     the behaviors with the settle and carry heads, the later positions and the predicted input items, and the shape, whether the weights are written half as wide, of the network over \
     the events; a list of names rather than a count, so an output of another kind can be added later"
);

