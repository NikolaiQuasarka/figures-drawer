use crate::utils::{i32_to_u32, i32_to_usize};

pub struct Size(pub u32, pub u32);

#[derive(Debug)]
pub struct Point(pub i32, pub i32);

#[derive(Debug)]
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

    pub fn draw(&mut self, point: &Point, char: char) -> Result<(), i32> {
        let point_y_as_usize = i32_to_usize(point.1);
        let Some(row) = self.field.get_mut(point_y_as_usize) else {
            return Err(0);
        };

        let Some(cell) = row.get_mut(i32_to_usize(point.0)) else {
            return Err(1);
        };

        *cell = char;

        Ok(())
    }

    ///Get cell relatively by center of matrix
    pub fn get_cell(&self, point: Point) -> Option<char> {
        if !self.is_char_displayed(point) {
            return None;
        }
        //блять Коля эта какаета хуйня, что ты написал. Крч, у тебя есть виртуальное поле
        // а есть настощяшее, корое матрица. Тебе надо функция, котороая преобразует велечины из вирутальных
        // единиц в единицы для матрицы. И вообще апи говно
        // self.field
    }

    pub fn get_size(&self) -> Size {
        let width = self.field.len() as u32;
        let height = self.field.len() as u32;
        Size(width, height)
    }
    pub fn to_string(&self) -> String {
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
        let (lt_corner, rb_corner) = self.get_coordinates();

        if point.0 >= lt_corner.0
            && point.0 <= rb_corner.0
            && point.1 <= lt_corner.1
            && point.1 >= rb_corner.1
        {
            return true;
        }
        false
    }

    fn real_position_from_relative(relative_position: Point, real_point: Point) -> Point {
        Point(
            real_point.0 + relative_position.0,
            real_point.1 + relative_position.1,
        )
    }
}

#[cfg(test)]
mod drawing_test {
    use super::*;

    // #[test]
    // fn grid_creation() {
    //     let grid = Grid::new(Size(49, 30), Point(0, 0));
    //     let manul_grid = Grid {
    //         field: Box::new([Box::new([' '; 49]); 30])
    //             .map(|row| row.map(|cell| cell).iter().collect())
    //             .iter()
    //             .collect(),
    //         center: Point(0, 0),
    //     };
    //     assert_eq!(manul_grid, grid);
    // }
    #[test]
    fn draw_zero() {
        let mut grid = Grid::new(Size(5, 5), Point(0, 0));
        grid.draw(&Point(0, 0), '+').unwrap();

        assert_eq!('+', grid.field[0][0]);
    }
    fn draw_below_zero() {
        let mut grid = Grid::new(Size(5, 5), Point(0, 0));
        grid.draw(&Point(0, 2), '+').unwrap();

        assert_eq!('+', grid.field[0][2]);
    }
    fn draw_under_zero() {
        let mut grid = Grid::new(Size(5, 5), Point(0, 0));
        grid.draw(&Point(0, -2), '+').unwrap();

        // assert_eq!('+', grid.field[0][-2]);
    }
}
