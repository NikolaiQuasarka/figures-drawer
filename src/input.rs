pub mod choose_figure;
pub mod get_grid;

use std::{error::Error, io, str::FromStr};

#[derive(Debug)]
struct CheckError;

impl Error for CheckError {}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Значение не прошло проверку")
    }
}

pub fn input<T, F>(prompt: &str, check: F) -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    T::Err: Into<Box<dyn Error>>,
    F: Fn(&T) -> bool,
{
    println!("{}", prompt);

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).map_err(|e| {
        eprintln!("Ошибка ввода: {}", e);
        e
    })?;

    let target_value = user_input.trim().parse::<T>().map_err(|e| {
        eprintln!("Ошибка парсинга");
        e.into()
    })?;

    if !check(&target_value) {
        return Err(Box::new(CheckError));
    }

    Ok(target_value)
}
