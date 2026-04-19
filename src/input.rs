pub mod choose_figure;
pub mod get_grid;
pub mod get_offset;

use std::{error::Error, io, str::FromStr};

#[derive(Debug)]
pub enum InputError {
    ParseError,
    BuffError,
    CheckError,
}

impl Error for InputError {}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::ParseError => write!(f, "Ошибка парсинга"),
            InputError::BuffError => write!(f, "Ошибка чтения ввода"),
            InputError::CheckError => write!(f, "Значение не прощло проверку"),
        }
    }
}

pub fn input<T, F>(prompt: &str, check_fail_message: &str, check: F) -> T
where
    T: FromStr,
    T::Err: Into<Box<dyn Error>>,
    F: Fn(&T) -> bool,
{
    let target_value = loop {
        println!("{}", prompt);

        let mut user_input = String::new();

        if let Err(_) = io::stdin().read_line(&mut user_input) {
            eprintln!("Ошибка чтения ввода");
            continue;
        }
        let target_value = match user_input.trim().parse::<T>() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("Ошибка парсинга");
                continue;
            }
        };

        if !check(&target_value) {
            eprintln!("{}", check_fail_message);

            continue;
        }

        break target_value;
    };

    target_value
}
