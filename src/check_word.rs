pub fn check_word() {
    let given_hex: &str = "111";
    let bytes_given = hex_to_bytes(given_hex);

    for word in constants::common_3_letter_words() {
        println!("--------------");
        let bytes_word: Vec<u8> = word.bytes().take(bytes_given.len()).collect();
        println!("bytes_word={:?}", bytes_word);
        let xor_result = xor_bytes(&bytes_word, &bytes_given);
        println!("XOR result between {word} and {given_hex}: {}", bytes_to_hex(&xor_result));
    }
}
