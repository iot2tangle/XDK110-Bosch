//!
//! To Generate a new Random KEY
//!
extern crate rand;
use rand::Rng;

///
/// Generates a new random String of 81 Chars of A..Z and 9
///
pub fn new() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghilmnopqrstuvwz123456789";
    const KEY_LEN: usize = 64;
    let mut rng = rand::thread_rng();

    let key: String = (0..KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    key
}
