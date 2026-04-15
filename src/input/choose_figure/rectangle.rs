use crate::figures::rectangle;
use std::io;

pub fn get_rectangle() -> rectangle::Rectangle {
    let width = get_size("Введите ширину:");
    let height = get_size("Введите высоту:");

    rectangle::Rectangle::from(width, height).expect("Не удалось создать прямоугольник")
}

fn get_size(prompt: &str) -> u32 {
    let size = loop {
        println!("{}", prompt);

        let mut size = String::new();

        if let Err(_) = io::stdin().read_line(&mut size) {
            eprintln!("Ошибка ввода. Попробуте еще раз.");
            continue;
        }

        let size = match size.parse::<u32>() {
            Ok(size) => {
                if size < 1 {
                    eprintln!("Размер должен быть не ниже 1. Нечетные числа не поддерживаются.");
                    continue;
                }

                size
            }
            Err(_) => {
                eprintln!("Некорректное число. Попробуйте еще раз.");
                continue;
            }
        };

        break size;
        // let size = size.parse::<u32>().is_ok_and(|size| size > 0);
    };

    size
}
