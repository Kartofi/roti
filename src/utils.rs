use std::time::{ SystemTime, UNIX_EPOCH };
use rand::Rng;

const CHARSET: &str = "abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const ID_LENGTH: u64 = 20;

pub fn get_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
pub fn random_num() -> u64 {
    let mut rng = rand::rng();
    rng.random()
}
pub fn random_char() -> char {
    let mut rng = rand::rng();
    CHARSET.as_bytes()[rng.random_range(0..=CHARSET.len() - 1)] as char
}
pub fn get_id(length: u64) -> String {
    let mut output = String::new();
    for i in 0..length {
        output.push(random_char());
    }

    output
}
pub fn get_id_default() -> String {
    get_id(ID_LENGTH)
}
