use dam::network::{FeatureId, Stacked};
use dam_card::device::{card, scaled_on_card};
use dam_console::train::{TrainRow, Training, trained_acts};
use spec::contexts::training::vocabulary::{ADAGRAD_FLOOR, INIT_SCALE, LEARNING_RATE};

fn card_here() -> Option<dam_card::device::Card> {
    let said = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let found = std::panic::catch_unwind(|| card().ok()).unwrap_or(None);
    std::panic::set_hook(said);
    found
}

#[test]
fn the_card_runs_a_kernel_and_gives_the_numbers_back() {
    let Some(card) = card_here() else { return };
    let xs = [1.0_f32, 2.0, 3.0, 4.0];
    let ys = [10.0_f32, 20.0, 30.0, 40.0];
    let got = scaled_on_card(&card, (&xs, &ys), 0.5).expect("the kernel compiles and runs");
    assert_eq!(got, vec![10.5, 21.0, 31.5, 42.0]);
}

fn numbers(net: &Stacked) -> Vec<f32> {
    net.table.values().flatten().chain(&net.slot_weights).chain(&net.fill).chain(&net.hidden_bias).chain(&net.out).chain(&net.out_bias).chain(&net.point).copied().collect()
}

#[test]
fn the_card_trains_the_numbers_the_processor_trains() {
    if card_here().is_none() {
        return;
    }
    let outputs: Vec<String> = ["go", "stop", "copy"].iter().map(|s| s.to_string()).collect();
    let rows: Vec<TrainRow> = (0..40u32)
        .map(|k| {
            let copies = k % 3 == 2;
            let items: Vec<Vec<FeatureId>> = (0..1 + k % 7).map(|s| [k % 5 + 1, s * 11 + k % 4, 900 + s].into_iter().map(FeatureId::from).collect()).collect();
            TrainRow {
                line: (k / 4) as usize,
                behavior: Some(outputs[(k % 3) as usize].clone()),
                pointed: if copies { vec![0] } else { Vec::new() },
                pointable: if copies { (0..items.len()).collect() } else { Vec::new() },
                depth: items.len(),
                stack: std::sync::Arc::new(items),
                ..TrainRow::default()
            }
        })
        .collect();
    let t = Training {
        item: 8,
        hidden: 32,
        epochs: 6,
        rate: LEARNING_RATE,
        floor: ADAGRAD_FLOOR,
        scale: INIT_SCALE,
        seed: 1,
        slices: 5,
        threads: 1,
    };
    let mut losses = (Vec::new(), Vec::new());
    let (on_cpu, _) = trained_acts(&rows, &outputs, (&t, (0, 2.0, 100, None), (None, false)), &mut |e| losses.0.push((e.loss, e.wrong))).unwrap();
    let (on_card, _) = trained_acts(&rows, &outputs, (&t, (0, 2.0, 100, None), (None, true)), &mut |e| losses.1.push((e.loss, e.wrong))).unwrap();
    assert!(!on_cpu.point.is_empty(), "the rows copy a token, so the network points");
    let (a, b) = (numbers(&on_cpu), numbers(&on_card));
    assert_eq!(a.len(), b.len());
    let worst = a.iter().zip(&b).map(|(x, y)| (x - y).abs()).fold(0.0_f32, f32::max);
    assert!(worst < 1e-3, "the card's weights differ from the processor's by {worst}");
    for ((cpu_loss, cpu_wrong), (card_loss, card_wrong)) in losses.0.iter().zip(&losses.1) {
        assert!((cpu_loss - card_loss).abs() < 1e-3, "losses {cpu_loss} and {card_loss}");
        assert!(card_wrong >= cpu_wrong, "the card counts each row in its own pass, before its batch moved the weights, so it counts no fewer wrong: {cpu_wrong} on the processor, {card_wrong} on the card");
    }
}
