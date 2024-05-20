fn main() {
    // Assumed ciphertext part in hex
    let hex_ciphertext = "35f";
    // Convert hex to bytes
    let ciphertext_bytes = hex::decode(hex_ciphertext).unwrap();

    // Assumed plaintext
    let plaintext = "The";
    // Convert plaintext to bytes
    let plaintext_bytes = plaintext.as_bytes();

    // Calculate the key bytes
    let key_bytes: Vec<u8> = ciphertext_bytes
        .iter()
        .zip(plaintext_bytes.iter())
        .map(|(&c, &p)| c ^ p)
        .collect();

    // Print the key bytes in hex format
    let key_hex: Vec<String> = key_bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    println!("Key bytes in hex: {}", key_hex.join(""));
}
