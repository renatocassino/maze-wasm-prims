pub const BLOCK_SIZE: f64 = 8.0;
pub const COLS: usize = 50;
pub const ROWS: usize = 50;

#[derive(Copy, Clone)]
pub enum DIRECTION {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
