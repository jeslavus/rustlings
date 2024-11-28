fn trim_me(input: &str) -> &str {
    input.trim() // Убираем пробелы с начала и конца строки
}

fn compose_me(input: &str) -> String {
    format!("{} world!", input) // Добавляем " world!" к строке
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons") // Заменяем "cars" на "balloons"
}

fn main() {
    // Пример использования функций:
    let trimmed = trim_me("   Hello!   ");
    let composed = compose_me("Hello");
    let replaced = replace_me("I think cars are cool");
    
    println!("Trimmed: {}", trimmed);
    println!("Composed: {}", composed);
    println!("Replaced: {}", replaced);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
