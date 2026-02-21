use crate::drawing::{
    basic::{Point, Size},
    matrix::Matrix,
};

#[derive(Debug)]
pub struct Grid {
    field: Matrix<char>,
    center: Point,
}

impl Grid {
    pub fn new(size: Size, center: Point) -> Result<Self, ()> {
        let Ok(matrix) = Matrix::from(size) else {
            return Err(());
        };

        Ok(Grid {
            field: matrix,
            center,
        })
    }
}
