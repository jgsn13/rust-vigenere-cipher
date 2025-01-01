fn vigenere_encrypt(plain_text: &String, alphabet: &String, key: &String) -> String {
    let upper_plain_text = plain_text.to_uppercase();
    let upper_key = key.to_uppercase();
    let mut cipher_text = String::from("");
    let mut key_index: usize = 0;

    for character in upper_plain_text.chars() {
        let xi = alphabet.find(character).unwrap();
        let ki = alphabet
            .find(upper_key.chars().nth(key_index).unwrap())
            .unwrap();
        let index = (xi + ki) % alphabet.len();

        cipher_text.push(alphabet.chars().nth(index).unwrap());

        key_index = (key_index + 1) % upper_key.len();
    }

    cipher_text
}

fn vigenere_decrypt(cipher_text: &String, alphabet: &String, key: &String) -> String {
    let upper_cipher_text = cipher_text.to_uppercase();
    let upper_key = key.to_uppercase();
    let alphabet_len = alphabet.len();
    let mut plain_text = String::from("");
    let mut key_index: usize = 0;

    for character in upper_cipher_text.chars() {
        let xi = alphabet.find(character).unwrap();
        let ki = alphabet
            .find(upper_key.chars().nth(key_index).unwrap())
            .unwrap();
        let index = (xi.checked_sub(ki).unwrap_or(alphabet_len - ki + xi)) % alphabet_len;

        plain_text.push(alphabet.chars().nth(index).unwrap());

        key_index = (key_index + 1) % upper_key.len();
    }

    plain_text
}

fn main() {
    let alphabet = String::from(" ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("SECRET");
    let plain_text = String::from("MY NAME IS JOAQUIM");
    println!("Alphabet: {alphabet}");
    println!("Key: {key}");
    println!("Plain text: {plain_text}\n");

    let encrypted_text = vigenere_encrypt(&plain_text, &alphabet, &key);
    let decrypted_text = vigenere_decrypt(&encrypted_text, &alphabet, &key);
    println!("Encrypted text: {encrypted_text}");
    println!("Decrypted text: {decrypted_text}");
}
