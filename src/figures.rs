use crate::{
    drawing::{Grid, Point, Size},
    utils::u32_to_i32,
};

pub trait Drawable {
    fn create_drawing(&self) -> Grid;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Drawable for Rectangle {
    fn create_drawing(&self) -> Grid {
        let width = self.width;
        let height = self.height;

        let mut grid = Grid::new(Size(width, height), Point(0, 0));

        for y in 0..height {
            for x in 0..width {
                let mut char = ' ';
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    char = '*';
                }
                if let Err(num) = grid.draw(&Point(u32_to_i32(x), u32_to_i32(y)), char) {
                    eprint!("{num}");
                    unreachable!()
                }
            }
        }

        grid
    }
}

#[cfg(test)]
mod rectangle_test {
    use super::*;

    #[test]
    fn draw_rectangle() {
        let rectangle_string = r#"*****
            *   *
            *   *
            *   *
            *****"#;

        let rectangle = Rectangle {
            width: 5,
            height: 5,
        };

        assert_eq!(rectangle_string, rectangle.create_drawing().to_string())
    }
}

struct Circle {
    radius: i32,
}

struct Triangle {
    angle1: u8,
    angle2: u8,
}
