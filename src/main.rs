use std::fs::File;

mod constants;
mod utils;

fn main() {
    let word = "bitcoin";
    let word_len = word.len();
    let total_len = 512;

    let mut results = Vec::new();

    for i in 0..=total_len - word_len {
        let mut result = String::new();
        result.push_str(&"0".repeat(i));
        result.push_str(word);
        result.push_str(&"0".repeat(total_len - i - word_len));
        results.push(result.clone());
    }

    let ciphertexts_hex = constants::cipher_lines();

    let mut file = File::create("xor_results.txt").expect("Unable to create file");
    for result in results {
        writeln!(file, "------------").expect("Unable to write to file");
        let bytes_result: Vec<u8> = result.bytes().collect();

        for j in 0..ciphertexts_hex.len() {
            let bytes_line = utils::hex_to_bytes(&ciphertexts_hex[j]);

            let xor_result = utils::xor_bytes(&bytes_result, &bytes_line);
            writeln!(file, "XOR: {}", utils::bytes_to_hex(&xor_result)).expect(
                "Unable to write to file"
            );
        }
    }
}
