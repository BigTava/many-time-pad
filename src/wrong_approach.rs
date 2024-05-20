use std::io::prelude::*;
use std::fs::File;

mod constants;
mod utils;

fn main() {
    let word = "bitcoin";
    let word_len = word.len();
    let total_len = 356;

    let mut results = Vec::new();

    for i in 0..=total_len - word_len {
        let mut result = String::new();
        result.push_str(&"0".repeat(i));
        result.push_str(word);
        result.push_str(&"0".repeat(total_len - i - word_len));
        results.push(result.clone());
    }

    let ciphertexts_hex = constants::cipher_lines();

    // Calculate all possible codes
    let mut xor_results = Vec::new();
    for result in results.clone() {
        let bytes_result: Vec<u8> = result.bytes().collect();

        for j in 0..ciphertexts_hex.len() {
            let bytes_line = utils::hex_to_bytes(&ciphertexts_hex[j]);
            let xor_result = utils::xor_bytes(&bytes_result, &bytes_line);
            xor_results.push(utils::bytes_to_hex(&xor_result).clone());
        }
    }

    // Calculate all possible real messages
    let mut file = File::create("xor_results.txt").expect("Unable to create file");
    let mut pos = 0;
    for xor_result in xor_results {
        let bytes_result: Vec<u8> = xor_result.bytes().collect();

        for result in results.clone() {
            let bytes_result: Vec<u8> = result.bytes().collect();
            let xor_result = utils::xor_bytes(&bytes_result, &bytes_result);

            while pos < xor_result.len() {
                let bytes_written = file.write(&xor_result[pos..]).expect("Unable to write file");
                pos += bytes_written;
            }

            writeln!(file, "XOR: {}", utils::bytes_to_hex(&xor_result)).expect(
                "Unable to write to file"
            );
        }
    }
}
