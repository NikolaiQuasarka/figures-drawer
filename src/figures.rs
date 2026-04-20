pub mod circle;
pub mod rectangle;
pub mod triangle;

use crate::drawing::matrix::Matrix;

pub trait Drawable {
    fn create_drawing(&self) -> Result<Matrix, ()>;
}
