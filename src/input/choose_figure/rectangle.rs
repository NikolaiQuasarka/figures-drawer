use crate::{figures::rectangle, input};
<<<<<<< HEAD
use std::io;
=======
>>>>>>> d6daa2a (Заменил старые инпуты на новый метод)

pub fn get_rectangle() -> rectangle::Rectangle {
    let width = input::input(
        "Введите ширину:",
        "Ширина должна быть больше 0 и целым числом",
        |&width| width > 0,
    );
    let height = input::input(
        "Введите высоту:",
        "Ширина должна быть больше 0 и целым числом",
        |&height| height > 0,
    );

    rectangle::Rectangle::from(width, height).expect("Не удалось создать прямоугольник")
}
