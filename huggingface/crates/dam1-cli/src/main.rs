#![deny(non_snake_case)]
#![deny(unreachable_patterns)]
#![forbid(unknown_lints)]

use dam_console::commands::{Console, Shaping};
use dam_console::files::{args, exit_code};
use dam_console::train::Training;
use dam1_hub::commands::run;
use spec::contexts::training::vocabulary::{ADAGRAD_FLOOR, HIDDEN_UNITS, INIT_SCALE, ITEM_WIDTH, LEARNING_RATE, LOOP_ENOUGH, LOOP_EPOCHS, LOOP_PATIENCE, LOOP_SETTLE, TRAIN_SEED, TRAIN_SLICES, TRAIN_THREADS, WORD_STEPS};
use std::process::ExitCode;

fn main() -> ExitCode {
    let console = Console {
        training: Training { item: ITEM_WIDTH, hidden: HIDDEN_UNITS, epochs: LOOP_EPOCHS, rate: LEARNING_RATE, floor: ADAGRAD_FLOOR, scale: INIT_SCALE, seed: TRAIN_SEED, slices: TRAIN_SLICES, threads: TRAIN_THREADS },
        shaping: Shaping {
            steps: WORD_STEPS,
            hidden: HIDDEN_UNITS,
            settle: LOOP_SETTLE,
            enough: LOOP_ENOUGH,
            patience: LOOP_PATIENCE,
            epochs: LOOP_EPOCHS,
        },
    };
    exit_code(run(&args(), &console))
}
