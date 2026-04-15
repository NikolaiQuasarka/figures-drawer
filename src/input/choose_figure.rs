mod rectangle;

use crate::figures::Drawable;

pub enum FigureInput {
    Circle,
    Rectangle,
    Triangle,
}

impl FigureInput {
    fn from(str: &str) -> Result<Box<dyn Drawable>, ()> {
        let draw: Box<dyn Drawable> = Box::new(match str {
            "Circle" => todo!(),
            "Rectangle" => rectangle::get_rectangle(),
            "Triangle" => todo!(),
            _ => return Err(()),
        });

        Ok(draw)
    }

    pub fn list() -> &'static [&'static str; 3] {
        &["Circle", "Rectangle", "Triangle"]
    }
}
