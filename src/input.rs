pub mod choose_figure;
pub mod get_grid;

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

pub fn input<T, F>(prompt: &str, err_message: &str, check: F) -> Result<T, InputError>
where
    T: FromStr,
    T::Err: Into<Box<dyn Error>>,
    F: Fn(&T) -> bool,
{
    println!("{}", prompt);

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).map_err(|e| {
        eprintln!("{}: {}", err_message, e);
        InputError::BuffError
    })?;

    let target_value = user_input.trim().parse::<T>().map_err(|_| {
        eprintln!("Ошибка парсинга");
        InputError::ParseError
    })?;

    if !check(&target_value) {
        return Err(InputError::CheckError);
    }

    Ok(target_value)
}
