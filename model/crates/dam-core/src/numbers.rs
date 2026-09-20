use patterns::{because, none, unit};

pub fn zero() -> f32 {
    none() as f32
}
because!(
    zero,
    "zero as a single precision number, the library's none narrowed, since a float literal in a pattern is a value from nowhere; beside \
     one, which narrows the library's unit the same way"
);

pub fn one() -> f32 {
    unit() as f32
}
because!(
    one,
    "one as a single precision number, the library's unit narrowed, for the reason zero gives; beside zero, which narrows the library's \
     none the same way"
);
