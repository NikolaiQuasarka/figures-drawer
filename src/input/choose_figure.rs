mod rectangle;

use std::io;

use crate::figures::Drawable;

pub enum DrawInput {
    Circle,
    Rectangle,
    Triangle,
}

impl DrawInput {
    fn from(str: &str) -> Result<Box<dyn Drawable>, ()> {
        let drawing: Box<dyn Drawable> = Box::new(match str {
            "Circle" => todo!(),
            "Rectangle" => rectangle::get_rectangle(),
            "Triangle" => todo!(),
            _ => return Err(()),
        });

        Ok(drawing)
    }

    pub fn list() -> &'static [&'static str; 3] {
        &["Circle", "Rectangle", "Triangle"]
    }
}

pub fn get_drawing() -> Box<dyn Drawable> {
    println!(
        "Введите название рисунка:{}",
        DrawInput::list()
            .iter()
            .enumerate()
            .map(|(i, draw)| { format!("\n{i}. {draw}") })
            .collect::<String>()
    );

    let drawing = loop {
        let mut draw = String::new();

        if let Err(_) = io::stdin().read_line(&mut draw) {
            eprintln!("Ошибка ввода. Повторите попытку.");

            continue;
        }

        let drawing = draw.trim();

        let drawing = match DrawInput::from(&drawing) {
            Ok(drawing) => drawing,
            Err(_) => {
                eprintln!("Такого рисунка не существует");
                continue;
            }
        };

        break drawing;
    };

    draw
}
