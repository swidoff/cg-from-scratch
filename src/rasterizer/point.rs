pub struct Point {
    pub x: i64,
    pub y: i64,
    pub h: f64,
}

impl Point {
    pub fn new(x: i64, y: i64, h: f64) -> Point {
        Point { x, y, h }
    }
}
