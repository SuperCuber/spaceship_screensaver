pub(crate) const SCREEN_SIZE: (u32, u32) = (1920, 1080);

// Amount
pub(crate) const LINES_PER_FRAME: usize = 3;
pub(crate) const CENTER_SIZE: f32 = 200.0;

const TMP: f32 = 0.002;
pub(crate) const MIN_GROW_FACTOR: f32 = TMP * 0.1;
pub(crate) const MAX_GROW_FACTOR: f32 = TMP * 1.0;
pub(crate) const SHORT_LINE_BIAS: f32 = 3.0;

// Look
pub(crate) const GROWTH_TO_TAIL_LEN_RATIO: f32 = 40.0 / TMP;
pub(crate) const USE_CLEAR_COLORS: bool = false;
