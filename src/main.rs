use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() {
    loop {
        draw_rectangle();
    }
}

enum Language {
    Ru,
    En,
}

fn draw_rectangle() {
    //Choose language
    println!("Choose language: \nen -English \nru - русский");
    let lang = loop {
        let mut input = String::new();

        let read = io::stdin().read_line(&mut input);

        if let Err(_) = read {
            eprintln!("Input error. Try again");
            continue;
        };

        let input = input.trim();

        let lang = match input {
            "ru" => Language::Ru,
            "en" => Language::En,
            _ => {
                eprintln!("Invalid language name. Try again. Only \"en\" and \"ru\" available");
                continue;
            }
        };

        break lang;
    };

    let width = get_size(
        80,
        match lang {
            Language::Ru => "Введие ширину",
            Language::En => "Enter a width",
        },
        &lang,
    );
    let height = get_size(
        80,
        match lang {
            Language::Ru => "Введие высоту",
            Language::En => "Enter height",
        },
        &lang,
    );

    for i in 0..height {
        for n in 0..width {
            thread::sleep(Duration::from_millis(10));
            let mut char = ' ';
            if i == 0 || i == height - 1 {
                char = '-';
            } else if n == 0 || n == width - 1 {
                char = '|';
            }
            print!("{}", char);
            io::stdout().flush();
        }
        println!()
    }
    //Get width and height
}

fn get_size(max_value: u8, message: &str, lang: &Language) -> u8 {
    let size = loop {
        let mut input = String::new();

        println!("{}", message);

        let read = io::stdin().read_line(&mut input);

        if let Err(_) = read {
            eprint!(
                "{}",
                match lang {
                    Language::Ru => "Ошибка ввода.\nПовторите еше раз",
                    Language::En => "Inpt error.\nTry again",
                }
            );
            continue;
        };

        let input = input.trim();

        let size = match input.parse::<u8>() {
            Ok(size) => size,
            Err(_) => {
                eprintln!(
                    "{}",
                    match lang {
                        Language::Ru =>
                            "Вы ввели неверное значение. Значение должно быть между 1 и 80",
                        Language::En =>
                            "You enterd an invalid value. A value must be between 1 and 80",
                    }
                );
                continue;
            }
        };

        if size > max_value || size < 1 {
            eprintln!(
                "{}",
                match lang {
                    Language::Ru => "Значение должны быть между 1 и 80",
                    Language::En => "Value is supposed to be between 1 and 80",
                }
            );
            continue;
        }

        break size;
    };

    size
}
