use dam::cursor::{CursorMind, cursor_told};
use dam::network::{FEATURE_BITS, FeatureId, NetworkClasses, Stacked};
use dam::state::state_bytes;
use dam1::check::{network_differences, source_tensors, state_differences, tensor_differences};
use dam1::layout::{CONFIG_JSON, STATE_FILE, WEIGHTS_FILE};
use dam1::model::{Build, Model, NetworkSource, Reading, loaded, written};
use safetensors::tensor::TensorView;
use safetensors::{Dtype, SafeTensors, serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const SLOTS: usize = 3;
const ITEM: usize = 2;
const HIDDEN: usize = 4;

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("dam1-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn classes() -> Vec<String> {
    ["{continue}", "{grab}", "{drop @}", "{step parent}", "{get owner}"].iter().map(|c| c.to_string()).collect()
}

fn counted(n: usize, from: f32) -> Vec<f32> {
    (0..n).map(|i| from + i as f32 / 8.0).collect()
}

fn network(pointer: bool) -> NetworkSource {
    voting(pointer, 1)
}

fn voting(pointer: bool, voters: usize) -> NetworkSource {
    let k = classes().len();
    let mut table: BTreeMap<FeatureId, Vec<f32>> = BTreeMap::new();
    table.insert(0, vec![0.5, -0.25]);
    table.insert(7, vec![1.0, f32::NEG_INFINITY]);
    table.insert(1 << 40, vec![-0.0, 2.0]);
    table.insert(FeatureId::MAX, vec![f32::NAN, 3.0]);
    let mut slot_weights = counted(SLOTS * HIDDEN * ITEM, 1.0);
    slot_weights[1] = -0.0;
    let net = Stacked {
        slots: SLOTS,
        item: ITEM,
        hidden: HIDDEN,
        table,
        slot_weights,
        fill: counted(SLOTS * HIDDEN, 5.0),
        hidden_bias: counted(HIDDEN, 2.0),
        out: counted(k * HIDDEN, 20.0),
        out_bias: counted(k, 3.0),
        point: if pointer { counted(HIDDEN * ITEM, 9.0) } else { Vec::new() },
    };
    let named = NetworkClasses { bits: FEATURE_BITS, half: false, classes: classes(), stacked: net.shape(), voters };
    let weights = (0..voters).flat_map(|voter| Stacked { hidden_bias: counted(HIDDEN, 2.0 + voter as f32), ..net.clone() }.bytes()).collect();
    NetworkSource { weights, classes: serde_json::to_string(&named).unwrap() }
}

fn state() -> Vec<u8> {
    let mut mind = CursorMind::default();
    assert!(cursor_told(&mut mind, "tom {owns} car", false));
    state_bytes(&mind).unwrap()
}

fn model() -> Model {
    Model { network: network(true), state: state(), reading: Reading { steps: 50 }, build: Build { commit: "abc".to_string(), network: "all.bin".to_string(), state: "data/seeds".to_string() } }
}

fn edit_config(dir: &Path, change: impl FnOnce(&mut serde_json::Value)) {
    let file = dir.join(CONFIG_JSON);
    let mut config: serde_json::Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    change(&mut config);
    std::fs::write(&file, serde_json::to_vec_pretty(&config).unwrap()).unwrap();
}

type Held = (String, Dtype, Vec<usize>, Vec<u8>);

fn edit_weights(dir: &Path, change: impl FnOnce(&mut Vec<Held>)) {
    let file = dir.join(WEIGHTS_FILE);
    let bytes = std::fs::read(&file).unwrap();
    let safe = SafeTensors::deserialize(&bytes).unwrap();
    let mut held: Vec<Held> = safe.tensors().into_iter().map(|(name, v)| (name, v.dtype(), v.shape().to_vec(), v.data().to_vec())).collect();
    change(&mut held);
    let (_, metadata) = SafeTensors::read_metadata(&bytes).unwrap();
    let views: Vec<(String, TensorView)> = held.iter().map(|t| (t.0.clone(), TensorView::new(t.1, t.2.clone(), &t.3).unwrap())).collect();
    std::fs::write(&file, serialize(views, metadata.metadata().clone()).unwrap()).unwrap();
}

#[test]
fn a_model_folder_loads_back_bit_for_bit() {
    for pointer in [true, false] {
        let dir = folder(if pointer { "whole-pointer" } else { "whole-plain" });
        let made = Model { network: network(pointer), ..model() };
        written(&made, &dir).unwrap();
        let back = loaded(&dir).unwrap();
        assert_eq!(back.network.weights, made.network.weights, "the weights file the tensors make is the source's, every not-a-number and negative zero kept");
        assert_eq!(back.state, made.state);
        assert_eq!(back.reading, made.reading);
        assert_eq!(back.build, made.build);
        assert!(network_differences(&made.network, &back.network).unwrap().is_empty());
        assert!(state_differences(&made.state, &back.state).is_empty());
    }
}

#[test]
fn the_weights_file_is_plain_safetensors() {
    let dir = folder("plain");
    written(&model(), &dir).unwrap();
    let bytes = std::fs::read(dir.join(WEIGHTS_FILE)).unwrap();
    let safe = SafeTensors::deserialize(&bytes).unwrap();
    let items = safe.tensor("network.0.items").unwrap();
    assert_eq!(items.dtype(), Dtype::U64);
    assert_eq!(items.shape(), &[4]);
    assert_eq!(u64::from_le_bytes(items.data()[24..32].try_into().unwrap()), u64::MAX);
    let table = safe.tensor("network.0.item_table").unwrap();
    assert_eq!(table.shape(), &[4, ITEM]);
    assert!(f32::from_le_bytes(table.data()[3 * ITEM * 4..3 * ITEM * 4 + 4].try_into().unwrap()).is_nan());
    let k = classes().len();
    let shapes: [(&str, Vec<usize>); 6] = [
        ("network.0.slot_weights", vec![SLOTS, HIDDEN, ITEM]),
        ("network.0.fill_weights", vec![SLOTS, HIDDEN]),
        ("network.0.hidden_bias", vec![HIDDEN]),
        ("network.0.output_weights", vec![k, HIDDEN]),
        ("network.0.output_bias", vec![k]),
        ("network.0.point_weights", vec![HIDDEN, ITEM]),
    ];
    for (name, shape) in shapes {
        let t = safe.tensor(name).unwrap();
        assert_eq!(t.dtype(), Dtype::F32, "{name}");
        assert_eq!(t.shape(), shape.as_slice(), "{name}");
    }
    assert_eq!(safe.tensor("network.0.slot_weights").unwrap().data()[4..8], (-0.0_f32).to_le_bytes());
    assert_eq!(safe.len(), 8);
    let (_, metadata) = SafeTensors::read_metadata(&bytes).unwrap();
    assert_eq!(metadata.metadata().as_ref().unwrap().get("format").map(String::as_str), Some("dam word network"));
    let config: serde_json::Value = serde_json::from_slice(&std::fs::read(dir.join(CONFIG_JSON)).unwrap()).unwrap();
    assert_eq!(config["format_version"], 5);
    assert_eq!(config["network"]["voters"], 1);
    assert_eq!(config["network"]["slots"], SLOTS);
    assert_eq!(config["network"]["item"], ITEM);
    assert_eq!(config["network"]["hidden"], HIDDEN);
    assert_eq!(config["network"]["items"], 4);
    assert_eq!(config["network"]["pointer"], true);
    assert_eq!(config["network"]["classes"].as_array().unwrap().len(), k);
    assert_eq!(config["reading"]["steps"], 50);
}

#[test]
fn a_model_reads_a_turn_the_same_after_loading() {
    let dir = folder("talk");
    let made = model();
    written(&made, &dir).unwrap();
    let back = loaded(&dir).unwrap();
    let said = |m: &Model| {
        dam_page::load(&m.network.weights, &m.network.classes).unwrap();
        dam_page::state(&m.state).unwrap();
        dam_page::read("who has the car?", m.reading.steps).unwrap()
    };
    assert_eq!(said(&made), said(&back), "the engine reads with the loaded model as with its source");
}

#[test]
fn the_check_compares_every_tensor_with_the_source_files() {
    let dir = folder("check");
    let made = model();
    written(&made, &dir).unwrap();
    let tensors = source_tensors(&made.network).unwrap();
    assert!(tensor_differences(&std::fs::read(dir.join(WEIGHTS_FILE)).unwrap(), &tensors).unwrap().is_empty());
    edit_weights(&dir, |held| held.iter_mut().find(|t| t.0 == "network.0.hidden_bias").unwrap().3[0] ^= 1);
    let found = tensor_differences(&std::fs::read(dir.join(WEIGHTS_FILE)).unwrap(), &tensors).unwrap();
    assert_eq!(found, vec!["network.0.hidden_bias: its bytes differ from the source".to_string()]);
}

#[test]
fn a_changed_state_is_found_and_a_broken_one_refused() {
    let dir = folder("state");
    let made = model();
    written(&made, &dir).unwrap();
    assert_eq!(state_differences(&made.state, &state_bytes(&CursorMind::default()).unwrap()).len(), 1);
    std::fs::write(dir.join(STATE_FILE), b"not a state").unwrap();
    assert!(loaded(&dir).unwrap_err().contains(STATE_FILE));
}

#[test]
fn another_architecture_or_layout_version_is_refused() {
    let dir = folder("version");
    written(&model(), &dir).unwrap();
    edit_config(&dir, |c| c["format_version"] = 4.into());
    assert!(loaded(&dir).unwrap_err().contains("version 4"));
    edit_config(&dir, |c| {
        c["format_version"] = 5.into();
        c["architecture"] = "dam reader".into();
    });
    assert!(loaded(&dir).unwrap_err().contains("dam reader"));
}

#[test]
fn a_class_that_is_no_step_is_refused() {
    let dir = folder("class");
    written(&model(), &dir).unwrap();
    edit_config(&dir, |c| c["network"]["classes"][1] = "{addNumber @}".into());
    assert!(loaded(&dir).unwrap_err().contains("{addNumber @}"));
}

#[test]
fn a_network_shape_that_does_not_fit_its_tensors_is_refused() {
    let dir = folder("shape");
    written(&model(), &dir).unwrap();
    edit_config(&dir, |c| c["network"]["hidden"] = (HIDDEN + 1).into());
    assert!(loaded(&dir).unwrap_err().contains("shape"));
}

#[test]
fn places_out_of_rising_order_are_refused() {
    let dir = folder("order");
    written(&model(), &dir).unwrap();
    edit_weights(&dir, |held| {
        let items = &mut held.iter_mut().find(|t| t.0 == "network.0.items").unwrap().3;
        let (first, rest) = items.split_at_mut(8);
        first.swap_with_slice(&mut rest[..8]);
    });
    assert!(loaded(&dir).unwrap_err().contains("rising order"));
}

#[test]
fn a_tensor_the_layout_does_not_name_is_refused() {
    let dir = folder("extra");
    written(&model(), &dir).unwrap();
    edit_weights(&dir, |held| held.push(("network.extra".to_string(), Dtype::F32, vec![1], vec![0; 4])));
    assert!(loaded(&dir).unwrap_err().contains("network.extra"));
}

#[test]
fn a_network_over_another_feature_width_is_refused() {
    let dir = folder("bits");
    written(&model(), &dir).unwrap();
    edit_config(&dir, |c| c["network"]["bits"] = 20.into());
    assert!(loaded(&dir).unwrap_err().contains("20 bits"));
}

#[test]
fn networks_that_read_by_vote_load_back_one_after_another() {
    let dir = folder("voters");
    let made = Model { network: voting(true, 3), ..model() };
    written(&made, &dir).unwrap();
    let bytes = std::fs::read(dir.join(WEIGHTS_FILE)).unwrap();
    let safe = SafeTensors::deserialize(&bytes).unwrap();
    assert_eq!(safe.len(), 24, "eight tensors for each of the three voters");
    let bias = |voter: usize| safe.tensor(&format!("network.{voter}.hidden_bias")).unwrap().data()[..4].to_vec();
    assert_eq!(bias(0), 2.0_f32.to_le_bytes());
    assert_eq!(bias(2), 4.0_f32.to_le_bytes());
    let config: serde_json::Value = serde_json::from_slice(&std::fs::read(dir.join(CONFIG_JSON)).unwrap()).unwrap();
    assert_eq!(config["network"]["voters"], 3);
    let back = loaded(&dir).unwrap();
    assert_eq!(back.network.weights, made.network.weights, "the voters stand in the weights file in the order they were written");
    assert_eq!(back.network.voters().unwrap().1.len(), 3);
    assert!(network_differences(&made.network, &back.network).unwrap().is_empty());
    assert_eq!(network_differences(&network(true), &back.network).unwrap().iter().filter(|found| found.contains("read by vote")).count(), 1);
}

#[test]
fn weights_written_half_as_wide_are_refused() {
    let mut named: NetworkClasses = serde_json::from_str(&network(true).classes).unwrap();
    named.half = true;
    let source = NetworkSource { classes: serde_json::to_string(&named).unwrap(), ..network(true) };
    assert!(source.voters().err().unwrap().contains("half as wide"));
}
