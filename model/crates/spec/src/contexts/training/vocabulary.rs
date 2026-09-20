use patterns::provisional;

pub const ITEM_WIDTH: usize = 16;
provisional!(
    ITEM_WIDTH,
    "how many numbers one event's embedding has in a network over the events; the user's first value, wide enough to tell the few hundred feature places of a lesson apart; a sweep of lesson accuracy against the width would settle it"
);

pub const LEARNING_RATE: f32 = 0.05;
provisional!(
    LEARNING_RATE,
    "the step AdaGrad scales every update by; a common starting rate for AdaGrad on small networks, and the loss curves of two seeds at nearby rates would settle it"
);

pub const ADAGRAD_FLOOR: f32 = 1e-8;
provisional!(
    ADAGRAD_FLOOR,
    "what is added under AdaGrad's division so a weight that has seen no gradient takes a finite first step; the customary value, small enough never to matter once a gradient has been seen"
);

pub const INIT_SCALE: f32 = 0.1;
provisional!(
    INIT_SCALE,
    "the widest a starting weight or embedding number is drawn, evenly on both sides of nothing, so the rectified units start neither dead nor saturated; a sweep of the first epochs' loss would settle it"
);

pub const TRAIN_SLICES: usize = 12;
provisional!(
    TRAIN_SLICES,
    "how many slices of rows a training step works at once against the weights at the start of the step, before they are applied one after another in their order; as many as the threads a run may use, so no thread waits; a sweep of epoch time and lesson accuracy against the count would settle it"
);

pub const TRAIN_THREADS: usize = 24;
provisional!(
    TRAIN_THREADS,
    "the most threads a training run works a step's slices on, fewer when the machine has fewer; every logical core of the machine the grades are trained on, since the user wants all of it at work and one run trains at a time; timing an epoch against the count would settle it"
);

pub const TRAIN_SEED: u64 = 1;
provisional!(
    TRAIN_SEED,
    "the seed a training run draws its starting weights and its row order from when a run does not name its own; any seed serves, and a lesson is also trained and reported on a second seed so a result is not the luck of the draw"
);

pub const LOOP_SETTLE: usize = 0;
provisional!(
    LOOP_SETTLE,
    "how many more epochs a loop training run goes on once enough learned rows are taken; none, since the user wants a run stopped as soon as it is good enough; on the seven shapes files settling well past the first right epoch took two more held out lines, which a release run may want again"
);

pub const LOOP_ENOUGH: f32 = 0.9;
provisional!(
    LOOP_ENOUGH,
    "the share of learned lines, each right only when every row of it is taken as taught, at which a loop training run stops: the user's rule that training stops at ninety in a hundred and only a release run is carried to every row; a sweep of held out lines against the share would settle whether a lower one teaches as well"
);

pub const LOOP_PATIENCE: usize = 20;
provisional!(
    LOOP_PATIENCE,
    "how many epochs a loop training run goes on without fewer learned rows taken wrongly before it stops: long enough that a slow run still finds its next low, and short enough that rows taught two actions on one stack, which can never all be right, do not keep a run going to the last epoch; a sweep of held out lines and epochs against the count would settle it"
);

pub const LOOP_EPOCHS: usize = 100;
provisional!(
    LOOP_EPOCHS,
    "the most epochs a loop training run makes: the user's rule that a run not good enough by then stops so its data is looked at, and only data found good is given more epochs; the epochs the curriculum's files took would settle a better cap"
);

pub const WORD_STEPS: usize = 50;
provisional!(
    WORD_STEPS,
    "the most steps the network takes at one word before it must continue, the user's hard rule: a word that needs more steps than this means the reading is not efficient enough, the stack lacks a feature, or the sentence is wrong, so the reading stops there and reports the input as not finished; the most steps the teacher takes at any word of the curriculum would settle how low it may go"
);

pub const HIDDEN_UNITS: usize = 512;
provisional!(
    HIDDEN_UNITS,
    "how many hidden units a network has: double the width at which the grade of chat turns asking who has more or fewer stopped short of its last rows through every extension run, since at this width it fit every row, the user holding that hidden units cost little"
);
