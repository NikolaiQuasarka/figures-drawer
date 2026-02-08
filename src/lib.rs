pub mod drawning {
    use std::fmt::{Debug, Display};

    pub struct Size(u32, u32);
    pub struct Point(i32, i32);

    pub struct Grid {
        field: Box<[Box<[char]>]>,
        center: Point,
    }

    impl Grid {
        pub fn new(size: Size, center: Point) -> Self {
            let field = (0..size.1)
                .map(|_| vec![' '; size.0 as usize].into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice();
            Grid { field, center }
        }

        pub fn draw(&mut self, drawing: Grid, point: Point) {
            unimplemented!()
        }

        pub fn to_string(&self, grid: Grid) -> String {
            let string = self
                .field
                .iter()
                .map(|chars| chars.iter().collect::<String>())
                .collect();

            string
        }

        fn get_coordinates(&self) -> (Point, Point) {
            let height = self.field.len() as i32;
            let width = self.field.get(0).expect("Height zero is invalid!").len() as i32;

            // let width_half = (width / 2).try_into().expect("usize error");
            // let height_half = (height / 2).try_into().expect("usize error");
            let width_half = width / 2;
            let height_half = height / 2;

            let corner_lh = Point(self.center.0 - width_half, self.center.1 + height_half);
            let corner_rd = Point(self.center.0 + width_half, self.center.1 - height_half);

            (corner_lh, corner_rd)
        }

        fn is_char_displayed(&self, point: Point) -> bool {
            unimplemented!()
        }

        fn real_position_from_relative(relative_position: Point, real_point: Point) -> Point {
            Point(
                real_point.0 + relative_position.0,
                real_point.1 + relative_position.1,
            )
        }
    }

    pub mod figures {
        use crate::drawning::Grid;

        trait Drawn {
            fn create_drawing() -> Grid;
        }

        struct Rectangle {
            width: i32,
            height: i32,
        }

        impl Drawn for Rectangle {
            fn create_drawing() -> Grid {}
        }

        struct Circle {
            radius: i32,
        }

        struct Triangle {
            angle1: u8,
            angle2: u8,
        }
    }
}
