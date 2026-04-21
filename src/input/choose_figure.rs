mod circle;
mod rectangle;

use std::io;

use crate::{drawing::basic, figures::Drawable, input};

fn from(str: &str) -> Result<Box<dyn Drawable>, ()> {
    let drawing: Box<dyn Drawable> = match str {
        "Circle" => Box::new(circle::get_circle()),
        "Rectangle" => Box::new(rectangle::get_rectangle()),
        "Triangle" => todo!(),
        _ => return Err(()),
    };

    Ok(drawing)
}

const LIST: [&str; 3] = ["Circle", "Rectangle", "Triangle"];

fn get_drawing() -> Box<dyn Drawable> {
    println!(
        "Введите название рисунка:{}",
        LIST.iter()
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

        let drawing = match from(&drawing) {
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

pub fn get_drawings() -> Vec<(Box<dyn Drawable>, basic::Point)> {
    let mut drawings = vec![];

    //Нужно получить минимум 1 рисунок
    let first_drawing = get_drawing();

    let first_drawing_offset = input::get_offset::get_offset();

    drawings.push((first_drawing, first_drawing_offset));

    loop {
        let answer: String = input::input(
            "Хотите нарисовать больше рисунков?\nВарианты ответа:\nД[а]\nН[ет]",
            "Таког ответа нет",
            |input| input == "Д" || input == "Н",
        );

        match answer.as_str() {
            "Д" => {
                let new_drawing = get_drawing();

                let new_drawing_offset = input::get_offset::get_offset();

                drawings.push((new_drawing, new_drawing_offset));
            }
            "Н" => break,
            _ => unreachable!(),
        }
    }

    drawings
}
