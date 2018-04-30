pub mod from_serde;

#[macro_use]
mod tagged_enum_or_default;

pub fn to_snake_case(string: &str) -> String {
    let mut snake = String::new();
    for ch in string.chars() {
        if ch.is_uppercase() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}
