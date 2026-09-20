use cudarc::driver::{CudaContext, CudaFunction, CudaStream, LaunchConfig, PushKernelArg};
use patterns::{because, source};
use std::sync::Arc;

pub struct CardRuntime;
source!(
    CardRuntime,
    "NVIDIA's CUDA driver and its runtime compiler, reached through the cudarc crate: a context on the first card, kernels compiled from their text when a run first uses the card, and a stream that copies numbers to the card, launches a kernel over them and copies them back"
);

pub struct Card {
    pub ctx: Arc<CudaContext>,
    pub stream: Arc<CudaStream>,
}
because!(Card, CardRuntime, "the first card of the machine with its default stream, made once for a run and used for every kernel of it");

pub fn err(e: impl std::fmt::Debug) -> String {
    format!("{e:?}")
}
because!(err, CardRuntime, "a driver or compiler error as text, so a run that cannot use the card says why and stays on the processor");

pub fn card() -> Result<Card, String> {
    let ctx = CudaContext::new(0).map_err(err)?;
    unsafe { ctx.disable_event_tracking() };
    let stream = ctx.default_stream();
    Ok(Card { ctx, stream })
}
because!(card, CardRuntime, "the first card of the machine and its default stream, or why there is none; the card is worked by one stream alone, so the driver keeps no record of which stream last wrote each group of numbers");

pub fn compiled<const N: usize>(card: &Card, text: &str, entries: [&str; N]) -> Result<[CudaFunction; N], String> {
    let ptx = cudarc::nvrtc::compile_ptx(text).map_err(err)?;
    let module = card.ctx.load_module(ptx).map_err(err)?;
    let loaded = entries.iter().map(|entry| module.load_function(entry).map_err(err)).collect::<Result<Vec<CudaFunction>, String>>()?;
    loaded.try_into().map_err(|_| "the kernel text gave fewer functions than it was asked for".to_string())
}
because!(compiled, CardRuntime, "kernels compiled on the card from one text and the functions it names, ready to launch");

const SCALED_TEXT: &str = r#"
extern "C" __global__ void scaled(const float *xs, float *ys, float a, int n) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) {
        ys[i] = a * xs[i] + ys[i];
    }
}
"#;
because!(SCALED_TEXT, CardRuntime, "the smallest kernel that proves the card works end to end: each number of one list added to a scaled number of another, one thread for each number");

pub fn scaled_on_card(card: &Card, (xs, ys): (&[f32], &[f32]), a: f32) -> Result<Vec<f32>, String> {
    let [func] = compiled(card, SCALED_TEXT, ["scaled"])?;
    let on_card = card.stream.clone_htod(xs).map_err(err)?;
    let mut added = card.stream.clone_htod(ys).map_err(err)?;
    let count = i32::try_from(xs.len().min(ys.len())).map_err(err)?;
    let mut launch = card.stream.launch_builder(&func);
    launch.arg(&on_card).arg(&mut added).arg(&a).arg(&count);
    unsafe { launch.launch(LaunchConfig::for_num_elems(count.unsigned_abs())) }.map_err(err)?;
    card.stream.clone_dtoh(&added).map_err(err)
}
because!(
    scaled_on_card,
    CardRuntime,
    "one list added to a scaled list on the card and copied back, the check that the driver, the runtime compiler, copying and launching all work on this machine before any network is taught there"
);
