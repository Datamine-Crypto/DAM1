use crate::layout::{
    ARCHITECTURE, CONFIG_JSON, FILL_PART, FORMAT_KEY, FORMAT_VERSION, HIDDEN_BIAS_PART, ITEMS_PART, NETWORK_PART, OUTPUT_BIAS_PART, OUTPUT_WEIGHTS_PART, POINT_PART, SLOT_WEIGHTS_PART, STATE_FILE, TABLE_PART, WEIGHTS_FILE,
    tensor_name, voter_owner,
};
use dam::network::{FEATURE_BITS, FeatureId, NetworkClasses, StackShape, Stacked};
use dam::state::state_mind;
use dam::word::WordStep;
use patterns::because;
use safetensors::tensor::TensorView;
use safetensors::{Dtype, SafeTensors, serialize};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Build {
    pub commit: String,
    pub network: String,
    pub state: String,
}
because!(Build, "where a model came from: the commit of the repository it was exported at and the network and state files it was made of, so a published model is traced to the English it was taught from");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reading {
    pub steps: usize,
}
because!(
    Reading,
    "the limit a reading runs under, as the spec the tool was built with sets it: the most steps the network may take at one word before the reading goes on to the next, so a loader reads as the console and the chat page read"
);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkSource {
    pub weights: Vec<u8>,
    pub classes: String,
}
because!(
    NetworkSource,
    "a word network as the two files the console, the page and the tests load it from: the bytes of its weights file, one network or the several that read by vote one after another, and the text of the classes file beside it, kept as read so a model is written from them, loaded back as them and compared with them"
);

impl NetworkSource {
    pub fn named(&self) -> Result<NetworkClasses, String> {
        serde_json::from_str(&self.classes).map_err(text_of)
    }

    pub fn voters(&self) -> Result<(NetworkClasses, Vec<Stacked>), String> {
        let named = self.named()?;
        if named.bits != FEATURE_BITS {
            return Err(format!("the network places its features in {} bits, and the engine reads {FEATURE_BITS}", named.bits));
        }
        if named.half {
            return Err("the weights are written half as wide, and a model is made from the full numbers".to_string());
        }
        if let Some(class) = named.classes.iter().find(|class| WordStep::of_class(class).is_none()) {
            return Err(format!("{class} is no step of the word network"));
        }
        let nets = Stacked::read_voters(&self.weights, named.stacked, named.classes.len(), named.voters.max(1))?;
        Ok((named, nets))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkShape {
    pub bits: u32,
    pub slots: usize,
    pub item: usize,
    pub hidden: usize,
    pub items: usize,
    pub pointer: bool,
    pub voters: usize,
    pub classes: Vec<String>,
}
because!(
    NetworkShape,
    "the network as a model's config names it: the width of the feature space its events' features are placed in, how many slots it reads, the width of an event's embedding, how many hidden units, how many feature places have an embedding, whether it points at the word a step copies, how many networks of this one shape read by vote, and its step classes in output order"
);

impl NetworkShape {
    pub fn stacked(&self) -> StackShape {
        StackShape { slots: self.slots, item: self.item, hidden: self.hidden, items: self.items, pointer: self.pointer }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub architecture: String,
    pub format_version: u32,
    pub reading: Reading,
    pub network: NetworkShape,
    pub build: Build,
}
because!(Config, "a model's config file: what the folder is and in which version of the layout, the limits a reading runs under, the network's feature width, shape and step classes, and where the model came from");

#[derive(Deserialize)]
struct Version {
    architecture: String,
    format_version: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    pub network: NetworkSource,
    pub state: Vec<u8>,
    pub reading: Reading,
    pub build: Build,
}
because!(Model, "a word network ready to read with: its files as written, the permanent state every chat starts from as its bytes, the limits a reading runs under and where it came from");

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tensor {
    pub name: String,
    pub dtype: Dtype,
    pub shape: Vec<usize>,
    pub bytes: Vec<u8>,
}
because!(Tensor, "one named tensor of a weights file: its name, its type, its shape and its little-endian bytes, so a tensor made from the source files and one read from a model folder compare byte for byte");

fn text_of(e: impl std::fmt::Display) -> String {
    e.to_string()
}

fn put(dir: &Path, name: &str, bytes: &[u8]) -> Result<(), String> {
    std::fs::write(dir.join(name), bytes).map_err(|e| format!("{name}: {e}"))
}

fn got(dir: &Path, name: &str) -> Result<Vec<u8>, String> {
    std::fs::read(dir.join(name)).map_err(|e| format!("{name}: {e}"))
}

fn json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    serde_json::to_vec_pretty(value).map_err(text_of)
}

fn json_from<T: DeserializeOwned>(dir: &Path, name: &str) -> Result<T, String> {
    serde_json::from_slice(&got(dir, name)?).map_err(|e| format!("{name}: {e}"))
}

fn float_bytes<'a>(xs: impl IntoIterator<Item = &'a f32>) -> Vec<u8> {
    xs.into_iter().flat_map(|x| x.to_le_bytes()).collect()
}

pub fn network_shape(named: &NetworkClasses, nets: &[Stacked]) -> NetworkShape {
    let s = named.stacked;
    NetworkShape { bits: named.bits, slots: s.slots, item: s.item, hidden: s.hidden, items: s.items, pointer: s.pointer, voters: nets.len(), classes: named.classes.clone() }
}
because!(network_shape, "the shape a model's config names for the networks the engine loaded from one weights file, how many they are, and their step classes in output order");

fn voter_tensor_shapes(shape: StackShape, outputs: usize, voter: usize) -> Vec<(String, Dtype, Vec<usize>)> {
    let owner = voter_owner(NETWORK_PART, voter);
    let named = |part: &str| tensor_name(&owner, part);
    let mut all = vec![
        (named(ITEMS_PART), Dtype::U64, vec![shape.items]),
        (named(TABLE_PART), Dtype::F32, vec![shape.items, shape.item]),
        (named(SLOT_WEIGHTS_PART), Dtype::F32, vec![shape.slots, shape.hidden, shape.item]),
        (named(FILL_PART), Dtype::F32, vec![shape.slots, shape.hidden]),
        (named(HIDDEN_BIAS_PART), Dtype::F32, vec![shape.hidden]),
        (named(OUTPUT_WEIGHTS_PART), Dtype::F32, vec![outputs, shape.hidden]),
        (named(OUTPUT_BIAS_PART), Dtype::F32, vec![outputs]),
    ];
    if shape.pointer {
        all.push((named(POINT_PART), Dtype::F32, vec![shape.hidden, shape.item]));
    }
    all
}
pub fn network_tensor_shapes(shape: StackShape, outputs: usize, voters: usize) -> Vec<(String, Dtype, Vec<usize>)> {
    (0..voters).flat_map(|voter| voter_tensor_shapes(shape, outputs, voter)).collect()
}
because!(
    network_tensor_shapes,
    "the name, type and exact shape of every tensor of a word network, voter by voter in the order the weights file holds them and within a voter in the order it writes their numbers, the pointer's weights last and only for a network that points, so the export, the loader and the check agree on one list"
);

fn voter_tensors(net: &Stacked, outputs: usize, voter: usize) -> Vec<Tensor> {
    let parts: Vec<Vec<u8>> = vec![
        net.table.keys().flat_map(|id| id.to_le_bytes()).collect(),
        float_bytes(net.table.values().flatten()),
        float_bytes(&net.slot_weights),
        float_bytes(&net.fill),
        float_bytes(&net.hidden_bias),
        float_bytes(&net.out),
        float_bytes(&net.out_bias),
        float_bytes(&net.point),
    ];
    voter_tensor_shapes(net.shape(), outputs, voter).into_iter().zip(parts).map(|((name, dtype, shape), bytes)| Tensor { name, dtype, shape, bytes }).collect()
}

pub fn network_tensors(nets: &[Stacked], outputs: usize) -> Vec<Tensor> {
    nets.iter().enumerate().flat_map(|(voter, net)| voter_tensors(net, outputs, voter)).collect()
}
because!(network_tensors, "the networks of one weights file, as the engine loaded them, as the tensors a model holds, voter by voter, each with its places, its item table, its slot blocks, fill weights, hidden biases, output weights, output biases and the pointer's weights");

pub fn written(model: &Model, dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let (named, nets) = model.network.voters()?;
    state_mind(&model.state)?;
    let tensors = network_tensors(&nets, named.classes.len());
    let views = tensors.iter().map(|t| TensorView::new(t.dtype, t.shape.clone(), &t.bytes).map(|view| (t.name.clone(), view))).collect::<Result<Vec<_>, _>>().map_err(text_of)?;
    let about = HashMap::from([(FORMAT_KEY.to_string(), ARCHITECTURE.to_string())]);
    put(dir, WEIGHTS_FILE, &serialize(views, Some(about)).map_err(text_of)?)?;
    let config = Config { architecture: ARCHITECTURE.to_string(), format_version: FORMAT_VERSION, reading: model.reading, network: network_shape(&named, &nets), build: model.build.clone() };
    put(dir, CONFIG_JSON, &json_bytes(&config)?)?;
    put(dir, STATE_FILE, &model.state)
}
because!(
    written,
    "a model written as a folder in the hub layout: the network loaded from its files through the engine's loader, which refuses a feature width, a length, an order of places or a class it cannot read, its numbers as little-endian tensors, the config with its shape and step classes as JSON a person can read, and the state as the bytes the engine loads, refused when the engine cannot load them"
);

fn shaped(name: &str, shape: &[usize], wanted: &[usize]) -> Result<(), String> {
    if shape == wanted { Ok(()) } else { Err(format!("{name}: shape {shape:?}, where the config makes {wanted:?}")) }
}

fn data_of<'a>(safe: &SafeTensors<'a>, name: &str, dtype: Dtype, wanted: &[usize]) -> Result<&'a [u8], String> {
    let view = safe.tensor(name).map_err(|e| format!("{WEIGHTS_FILE}: {e}"))?;
    if view.dtype() != dtype {
        return Err(format!("{name}: stored as {:?}, the network reads {dtype:?}", view.dtype()));
    }
    shaped(name, view.shape(), wanted)?;
    Ok(view.data())
}

fn only_named(safe: &SafeTensors, shape: &NetworkShape) -> Result<(), String> {
    let named: Vec<String> = network_tensor_shapes(shape.stacked(), shape.classes.len(), shape.voters).into_iter().map(|(name, _, _)| name).collect();
    match safe.names().into_iter().find(|name| !named.iter().any(|n| n == *name)) {
        Some(name) => Err(format!("{WEIGHTS_FILE}: holds {name}, a tensor this layout does not name")),
        None => Ok(()),
    }
}

fn network_source_of(safe: &SafeTensors, shape: &NetworkShape) -> Result<NetworkSource, String> {
    let mut weights = Vec::new();
    for voter in 0..shape.voters {
        let parts = voter_tensor_shapes(shape.stacked(), shape.classes.len(), voter).iter().map(|(name, dtype, wanted)| data_of(safe, name, *dtype, wanted)).collect::<Result<Vec<&[u8]>, String>>()?;
        let [items, table, dense @ ..] = parts.as_slice() else { continue };
        let row = (shape.item * size_of::<f32>()).max(size_of::<f32>());
        for (place, embedding) in items.chunks_exact(size_of::<FeatureId>()).zip(table.chunks_exact(row)) {
            weights.extend_from_slice(place);
            weights.extend_from_slice(embedding);
        }
        for part in dense {
            weights.extend_from_slice(part);
        }
    }
    let named = NetworkClasses { bits: shape.bits, half: false, classes: shape.classes.clone(), stacked: shape.stacked(), voters: shape.voters };
    Ok(NetworkSource { weights, classes: serde_json::to_string(&named).map_err(text_of)? })
}
because!(
    network_source_of,
    "a network's two files made again from a model folder: the weights file's bytes put back in the engine's order, voter after voter, each place before its embedding and then the dense parts, and the classes file from the config's shape and step classes, so the engine's own loader reads a model as it reads the files the console trained"
);

pub fn loaded(dir: &Path) -> Result<Model, String> {
    let version: Version = json_from(dir, CONFIG_JSON)?;
    if version.architecture != ARCHITECTURE {
        return Err(format!("{CONFIG_JSON}: holds {}, and this loader reads {ARCHITECTURE}", version.architecture));
    }
    if version.format_version != FORMAT_VERSION {
        return Err(format!("{CONFIG_JSON}: layout version {}, and this loader reads version {FORMAT_VERSION}", version.format_version));
    }
    let config: Config = json_from(dir, CONFIG_JSON)?;
    let bytes = got(dir, WEIGHTS_FILE)?;
    let safe = SafeTensors::deserialize(&bytes).map_err(|e| format!("{WEIGHTS_FILE}: {e}"))?;
    only_named(&safe, &config.network)?;
    let network = network_source_of(&safe, &config.network)?;
    network.voters()?;
    let state = got(dir, STATE_FILE)?;
    state_mind(&state).map_err(|e| format!("{STATE_FILE}: {e}"))?;
    Ok(Model { network, state, reading: config.reading, build: config.build })
}
because!(
    loaded,
    "a model folder read back: refused when it is another architecture or another version of the layout, when the weights file holds a tensor the layout does not name, lacks one, or has one of another type or shape, when the engine's loader refuses the network made from them, or when the engine cannot load the state, and otherwise the network's files, the state's bytes, the reading limits and the build as the folder holds them"
);
