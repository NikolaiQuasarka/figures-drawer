pub fn get_offset() -> crate::drawing::basic::Point {
    use crate::input;

    input::input(
        "Введите смещение в формате: x y\nПо умолчанию: 0 0",
        "",
        |_| true,
    )
}
