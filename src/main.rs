mod calories_maximizer;

fn find_elf_with_max_calories() -> Result<calories_maximizer::ElfBag, String>{
    use std::io::{stdin};
    let mut maximizer: calories_maximizer::CaloriesMaximizer = calories_maximizer::CaloriesMaximizer::new();
    loop {
        let mut input_text: String = String::new();
        let end: Result<usize, std::io::Error> = stdin().read_line(&mut input_text);
        if !end.is_ok() || input_text.len() <= 0 {
            break;
        }
        let trimmed = input_text.trim();

        if trimmed.len() > 0 {
            let parse_result: Result<i32, core::num::ParseIntError> = trimmed.parse::<i32>();
            let calories = match parse_result {
                Ok(i) => i,
                Err(err) => return Err(err.to_string())
            };
            maximizer.add_calories(calories);
        } else {
            maximizer.go_to_next();
        }
    }
    Ok(maximizer.get_max())
}

fn main() {
    let result = find_elf_with_max_calories();
    match result {
        Ok(elf) => println!("Max Elf {} with calories: {}", elf.get_index(), elf.get_calories()),
        Err(e) => println!("ERROR! {}", e)
    }
}
