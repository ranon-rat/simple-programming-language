use crate::types::Types;
use std::io;
pub fn get_input() -> Types {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }

    println!("{}", input.trim());
    let i = input.trim().to_string();
    if i.len() == 0 {
        return Types::String("".to_string());
    }
    let first_char = i.as_bytes()[0] as char;
    match &first_char {
        '0'..'9' => return Types::Number(i.to_string().parse::<f64>().unwrap()),
        _ => return Types::String(i.to_string()),
    }
}
