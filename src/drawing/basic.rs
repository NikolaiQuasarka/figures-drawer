#[derive(PartialEq, Debug)]
pub struct Size(pub u32, pub u32);

#[derive(Debug, Clone)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn default() -> Point {
        Point(0, 0)
    }
}
