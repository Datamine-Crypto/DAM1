use patterns::{because, source};

pub struct HubLayout;
source!(
    HubLayout,
    "the files a model repository on the Hugging Face hub holds for the word network and the state every chat starts from, as FORMAT.md beside these crates writes them down"
);

pub struct NetworkFiles;
source!(
    NetworkFiles,
    "the files the console reads a word network from: the weights file, one network or the several that read by vote one after another, each with the embeddings in rising order of their place each led by its place and then the slot blocks, fill weights, hidden biases, output weights, output biases and the pointer's weights, all little endian, and beside it a JSON file with the feature width, the step classes in output order and the shape"
);

pub const FORMAT_VERSION: u32 = 5;
because!(
    FORMAT_VERSION,
    HubLayout,
    "the version of the layout a folder was written in, raised by any change an older loader would misread, so a loader refuses a folder it cannot read rather than read its numbers wrongly; the fourth holds the cursor network alone, its step classes and the permanent state, the reader of the earlier versions being gone from the engine"
);

pub const ARCHITECTURE: &str = "dam word network";
because!(
    ARCHITECTURE,
    HubLayout,
    "what a model folder holds, written in its config and in the weights file's own metadata, so a tool that opens either can tell this model from a tensor network it would try to run"
);

pub const CONFIG_JSON: &str = "config.json";
because!(
    CONFIG_JSON,
    HubLayout,
    "the file that says what the folder is: the layout version, the limits a reading runs under, the network's feature width, shape and step classes, and the build, under the name the hub reads a model's settings from"
);

pub const WEIGHTS_FILE: &str = "model.safetensors";
because!(
    WEIGHTS_FILE,
    HubLayout,
    "every number of the network as named tensors in safetensors, the hub's own format, which any language reads without running code from the file, under the name the hub counts as the model's weights"
);

pub const STATE_FILE: &str = "state.bin";
because!(
    STATE_FILE,
    HubLayout,
    "the permanent state every chat starts from, the tree the seeds files make, as the bytes the console writes and the chat page loads, since telling every seed again at each load is slow"
);

pub const HUB_FILES: &[&str] = &[CONFIG_JSON, WEIGHTS_FILE, STATE_FILE];
because!(HUB_FILES, HubLayout, "every file a loader needs, so fetching a model from the hub fetches these and nothing else, whatever else the repository carries");

pub const FORMAT_KEY: &str = "format";
because!(FORMAT_KEY, HubLayout, "the key the weights file's metadata names its architecture under");

pub const NETWORK_PART: &str = "network";
because!(NETWORK_PART, HubLayout, "the first word of the name of every tensor the networks are kept in");

pub const ITEMS_PART: &str = "items";
because!(ITEMS_PART, HubLayout, "the feature places the network has an embedding for, in rising order, each once, one per row of its item table");

pub const TABLE_PART: &str = "item_table";
because!(TABLE_PART, HubLayout, "the network's embedding of each feature place, one row per place in the order of its items, one column per unit of an event's embedding");

pub const SLOT_WEIGHTS_PART: &str = "slot_weights";
because!(SLOT_WEIGHTS_PART, HubLayout, "each slot's own block of weights from an event's embedding into the hidden units, by slot, hidden unit and embedding unit");

pub const FILL_PART: &str = "fill_weights";
because!(FILL_PART, HubLayout, "the weight each hidden unit adds when a slot is filled, by slot and hidden unit");

pub const HIDDEN_BIAS_PART: &str = "hidden_bias";
because!(HIDDEN_BIAS_PART, HubLayout, "what every hidden unit starts from before any slot adds to it, one number per unit");

pub const OUTPUT_WEIGHTS_PART: &str = "output_weights";
because!(OUTPUT_WEIGHTS_PART, HubLayout, "the weights from the hidden units into each output, by output in the config's class order and hidden unit");

pub const OUTPUT_BIAS_PART: &str = "output_bias";
because!(OUTPUT_BIAS_PART, HubLayout, "the bias of each output, in the config's class order");

pub const POINT_PART: &str = "point_weights";
because!(POINT_PART, HubLayout, "the weights the pointer scores a slot's embedding with against the hidden units, by hidden unit and embedding unit, so a step that copies picks the slot it copies from");

pub const CLASSES_END: &str = ".json";
because!(CLASSES_END, NetworkFiles, "the ending added to a network's weights file to name the file listing its step classes and its shape");

pub fn tensor_name(owner: &str, part: &str) -> String {
    format!("{owner}.{part}")
}
because!(tensor_name, "a tensor's name in the weights file: what holds it and which of its parts it is, so a reader in any language finds a part by name");

pub fn voter_owner(owner: &str, voter: usize) -> String {
    format!("{owner}.{voter}")
}
because!(voter_owner, "what holds the tensors of one of the networks that read by vote: the network part and then the number of the voter, counted from nothing in the order the weights file holds them, so a reader in any language finds every part of every voter by name");
