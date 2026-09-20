use dam::events::{STACK_SLOTS, Stack, heard_item, step_ids};
use dam::network::FeatureId;

#[test]
fn slots_hold_the_newest_events_up_to_the_cap() {
    let mut s = Stack::default();
    for i in 0..STACK_SLOTS + STACK_SLOTS {
        s.push(heard_item(&format!("w{i}")));
    }
    assert_eq!(s.depth(), STACK_SLOTS + STACK_SLOTS);
    assert_eq!(s.slots().count(), STACK_SLOTS);
    let newest = format!("w{}", STACK_SLOTS + STACK_SLOTS - 1);
    assert_eq!(s.slots().next().map(|e| e.item.text.to_string()), Some(newest));
}

#[test]
fn a_number_is_known_to_the_network_by_its_form_last_digit_and_digits_alone() {
    let (seven, twelve, seventeen) = (heard_item("7").ids, heard_item("12").ids, heard_item("17").ids);
    assert_ne!(seven, twelve);
    let shared = |a: &[FeatureId], b: &[FeatureId]| a.iter().filter(|x| b.contains(x)).count();
    assert_eq!(shared(&seven, &seventeen), shared(&seven, &twelve) + 1, "seven and seventeen share the last digit");
    assert_eq!(shared(&twelve, &seventeen), shared(&seven, &twelve) + 1, "twelve and seventeen share how many digits they have");
    assert_eq!(shared(&heard_item("cat").ids, &seven), 1, "a word and a number share only the kind");
    assert_ne!(heard_item("cat").ids, heard_item("dog").ids);
}

#[test]
fn a_number_pushed_after_another_carries_the_gap_and_whether_it_doubled() {
    let top = |words: &[&str]| {
        let mut s = Stack::default();
        for w in words {
            s.push(heard_item(w));
        }
        s.items().next().unwrap().item.ids.clone()
    };
    assert_eq!(top(&["cat", "4"]), heard_item("4").ids, "a number with none before it has no step");
    let counted = top(&["3", "cat", "4"]);
    let mut want = heard_item("4").ids;
    want.extend(step_ids(4.0, 3.0));
    assert_eq!(counted, want, "the step is taken from the newest number, past other words");
    assert_eq!(top(&["2", "3", "4"]), want, "only the newest number counts");
    assert_eq!(step_ids(4.0, 3.0), step_ids(8.0, 7.0), "every step of a count by one reads the same");
    assert_ne!(step_ids(4.0, 2.0), step_ids(6.0, 4.0), "a doubling is told from a gap of the same size");
}
