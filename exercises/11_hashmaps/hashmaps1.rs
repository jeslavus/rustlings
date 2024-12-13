use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // Создаем HashMap
    let mut basket = HashMap::new();

    // Добавляем два банана
    basket.insert(String::from("banana"), 2);

    // Добавляем другие фрукты
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 1);

    basket
}

fn main() {
    // Опционально: можно проверить результат
    let basket = fruit_basket();
    println!("{:?}", basket);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
