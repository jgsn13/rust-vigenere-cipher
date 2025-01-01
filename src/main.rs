struct VigenereCipher {
    alphabet: String,
    key: String,
}

impl VigenereCipher {
    fn new(alphabet: String, key: String) -> Self {
        Self { alphabet, key }
    }

    fn encrypt(&self, plain_text: &String) -> String {
        let upper_plain_text = plain_text.to_uppercase();
        let upper_key = self.key.to_uppercase();
        let mut cipher_text = String::from("");
        let mut key_index: usize = 0;

        for character in upper_plain_text.chars() {
            let xi = self.alphabet.find(character).unwrap();
            let ki = self.alphabet
                .find(upper_key.chars().nth(key_index).unwrap())
                .unwrap();
            let index = (xi + ki) % self.alphabet.len();

            cipher_text.push(self.alphabet.chars().nth(index).unwrap());

            key_index = (key_index + 1) % upper_key.len();
        }

        cipher_text
    }

    fn decrypt(&self, cipher_text: &String) -> String {
        let upper_cipher_text = cipher_text.to_uppercase();
        let upper_key = self.key.to_uppercase();
        let alphabet_len = self.alphabet.len();
        let mut plain_text = String::from("");
        let mut key_index: usize = 0;

        for character in upper_cipher_text.chars() {
            let xi = self.alphabet.find(character).unwrap();
            let ki = self.alphabet
                .find(upper_key.chars().nth(key_index).unwrap())
                .unwrap();
            let index = (xi.checked_sub(ki).unwrap_or(alphabet_len - ki + xi)) % alphabet_len;

            plain_text.push(self.alphabet.chars().nth(index).unwrap());

            key_index = (key_index + 1) % upper_key.len();
        }

        plain_text
    }
}


fn main() {
    let alphabet = String::from(" ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let key = String::from("SECRET");
    println!("Alphabet: {alphabet}");
    println!("Key: {key}");

    let vigenere = VigenereCipher::new(alphabet, key);

    let plain_text = String::from("MY NAME IS JOAQUIM");
    println!("Plain text: {plain_text}\n");

    let encrypted_text = vigenere.encrypt(&plain_text);
    let decrypted_text = vigenere.decrypt(&encrypted_text);
    println!("Encrypted text: {encrypted_text}");
    println!("Decrypted text: {decrypted_text}");
}
