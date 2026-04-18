mod rectangle;

use std::io;

use crate::{figures::Drawable, input};

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

fn get_drawing() -> Box<dyn Drawable> {
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

    drawing
}

pub fn get_drawings() -> Vec<Box<dyn Drawable>> {
    let mut drawings = vec![];

    //Нужно получить минимум 1 рисунок
    let first_draw = get_drawing();

    drawings.push(first_draw);

    loop {
        let answer: String = input::input(
            "Хотите нарисовать больше рисунков?\nВарианты ответа:\nД[а]\nН[ет]",
            "Таког ответа нет",
            |input| input == "Д" || input == "Н",
        );

        match answer.as_str() {
            "Д" => {
                let new_drawing = get_drawing();

                drawings.push(new_drawing);
            }
            "Н" => break,
            _ => unreachable!(),
        }
    }

    drawings
}
