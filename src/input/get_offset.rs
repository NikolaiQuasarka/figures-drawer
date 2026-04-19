pub fn get_offset() -> crate::drawing::basic::Point {
    use crate::{drawing::basic, input};

    let x_offset = input::input("Введите смещение по x:", "", |_| true);
    let y_offset = input::input("Введите смещение по y:", "", |_| true);

    basic::Point(x_offset, y_offset)
}
