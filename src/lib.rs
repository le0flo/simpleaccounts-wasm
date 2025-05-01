use hashcash::Token;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_stamp(seed: &str, bits: u32) -> String {
    let stamp = Token::new(String::from(seed), bits);
    return stamp.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_test() {
        let result = Token::from_str(calculate_stamp("test@localhost", 10).as_str()).unwrap();
        match result.check() {
            Ok(_) => (),
            Err(_) => assert_eq!(1, 0),
        };
    }
}