use crate::model::{NetworkSource, Tensor, network_tensors};
use patterns::because;
use safetensors::SafeTensors;

pub fn network_differences(expected: &NetworkSource, got: &NetworkSource) -> Result<Vec<String>, String> {
    let (wanted, _) = expected.voters()?;
    let (held, _) = got.voters()?;
    let mut found = Vec::new();
    if wanted.bits != held.bits {
        found.push(format!("the feature width is {} in the model and {} in the source", held.bits, wanted.bits));
    }
    if wanted.classes != held.classes {
        found.push("the step classes differ from the source's".to_string());
    }
    if wanted.stacked != held.stacked {
        found.push(format!("the shape is {:?} in the model and {:?} in the source", held.stacked, wanted.stacked));
    }
    if wanted.voters.max(1) != held.voters.max(1) {
        found.push(format!("{} networks read by vote in the model and {} in the source", held.voters.max(1), wanted.voters.max(1)));
    }
    if expected.weights != got.weights {
        found.push("the weights file the model's tensors make differs from the source's".to_string());
    }
    Ok(found)
}
because!(
    network_differences,
    "every way a network loaded from a model folder differs from the network it came from: its feature width, its step classes in order, its shape, how many networks read by vote, and the weights file its tensors make, byte for byte; empty when they are the same network"
);

pub fn source_tensors(network: &NetworkSource) -> Result<Vec<Tensor>, String> {
    let (named, nets) = network.voters()?;
    Ok(network_tensors(&nets, named.classes.len()))
}
because!(source_tensors, "the tensors a network's source files make, through the engine's own loader, so a weights file is compared with what its source says and not with what the export wrote");

pub fn tensor_differences(weights: &[u8], expected: &[Tensor]) -> Result<Vec<String>, String> {
    let safe = SafeTensors::deserialize(weights).map_err(|e| e.to_string())?;
    let mut found = Vec::new();
    for t in expected {
        match safe.tensor(&t.name) {
            Err(_) => found.push(format!("{}: missing from the model", t.name)),
            Ok(view) if view.dtype() != t.dtype || view.shape() != t.shape.as_slice() => {
                found.push(format!("{}: {:?} {:?} in the model, {:?} {:?} in the source", t.name, view.dtype(), view.shape(), t.dtype, t.shape));
            }
            Ok(view) if view.data() != t.bytes.as_slice() => found.push(format!("{}: its bytes differ from the source", t.name)),
            Ok(_) => {}
        }
    }
    for name in safe.names() {
        if !expected.iter().any(|t| t.name == *name) {
            found.push(format!("{name}: in the model, not in the source"));
        }
    }
    Ok(found)
}
because!(
    tensor_differences,
    "every way a weights file differs from the tensors its source makes: a tensor missing, of another type or shape, with one byte changed, or held without being in the source; empty when every tensor is the same byte for byte"
);

pub fn state_differences(expected: &[u8], got: &[u8]) -> Vec<String> {
    if expected == got { Vec::new() } else { vec![format!("the state is {} bytes in the model and {} in the source, or its bytes differ", got.len(), expected.len())] }
}
because!(state_differences, "whether the permanent state a model folder holds is, byte for byte, the state its source file holds, said as one difference when it is not");
