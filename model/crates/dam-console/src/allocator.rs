use patterns::{because, source};

pub struct Mimalloc;
source!(
    Mimalloc,
    "the mimalloc allocator from Microsoft Research, which gives every thread a heap of its own so threads allocating at once do not wait on one shared lock"
);

#[global_allocator]
pub static ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;
because!(
    ALLOCATOR,
    Mimalloc,
    "the allocator every console command allocates through: reading clones whole readings for every behavior of every word, on every thread at once when the facts are seeded or the quiz is scored, and on the system heap the threads wait on one another for every allocation"
);
