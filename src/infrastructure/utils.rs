use rand::Rng;
use std::iter;

pub const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

pub fn generate_salt(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(length).collect()
}
