enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .into_iter()
            .map(|(string, command)| match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(times) => format!("{}{}", string, "bar".repeat(times)),
            })
            .collect()
    }
}

fn main() {
    // Example usage
    let input = vec![
        ("hello".to_string(), Command::Uppercase),
        (" all roads lead to rome! ".to_string(), Command::Trim),
        ("foo".to_string(), Command::Append(1)),
        ("bar".to_string(), Command::Append(5)),
    ];

    let output = my_module::transformer(input);

    println!("{:?}", output);
}

#[cfg(test)]
mod tests {
    use super::Command;
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
