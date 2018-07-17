/// This example does the following:
/// 1. Generate a random key for symmetric encryption. Normally you would only do this once and then either store the
///    key on the only machine that needs access to it (in order to implement an encrypted data storage on that machine)
///    or distribute the key to another machine so that encrypted messages can be sent and decrypted.
/// 2. Use the key to encrypt some data. This corresponds to the saving or sending of data.
/// 3. Use the key to decrypt the data again. This corresponds to loading or receiving the data.
pub fn main() {
    use ring::aead;
    use ring::rand::{SecureRandom, SystemRandom};
    use std::io::prelude::*;
    use std::fs::File;
    use base64;

    let keyfile = "example.key";

    // GENERATE key and write it to the key file
    {
        let mut key: [u8; 32] = [0; 32];
        SystemRandom::new().fill(&mut key).unwrap();
        let mut keyfile = File::create(keyfile).unwrap();
        keyfile.write_all(&key).unwrap();
    }

    // The data to be encrypted
    let original_data = "Text that is going to be sent over an insecure channel and must be encrypted at all costs!".as_bytes();

    // The algorithm to be used
    let algorithm : &aead::Algorithm = &aead::CHACHA20_POLY1305;

    // GENERATE random nonce (number used once)
    let nonce = {
        use std::iter::repeat;
        let mut nonce: Vec<u8> = repeat(0).take(algorithm.nonce_len()).collect();
        SystemRandom::new().fill(&mut nonce).unwrap();
        nonce
    };

    // ENCRYPTION
    let ciphertext_base64 : String = {
        // LOAD key file for encryption
        let mut file = File::open(keyfile).unwrap();
        let mut key: [u8; 32] = [0; 32];
        file.read_exact(&mut key).unwrap();
        drop(file);

        // ENCRYPT the original data
        let cipher = aead::SealingKey::new(algorithm, &key).unwrap();
        let mut in_out = original_data.to_vec();
        in_out.resize(original_data.len() + algorithm.tag_len(), 0u8);
        let ciphertext_length = aead::seal_in_place(&cipher, &nonce, &[0; 0], &mut in_out, algorithm.tag_len()).unwrap();
        let ciphertext = &in_out[..ciphertext_length];

        // CONVERT raw bytes to Base64 representation
        base64::encode(ciphertext)
    };

    // DECRYPTION
    let decrypted_data : String = {
        // LOAD key file for decryption
        let mut file = File::open(keyfile).unwrap();
        let mut key: [u8; 32] = [0; 32];
        file.read_exact(&mut key).unwrap();
        drop(file);

        // DECRYPT the ciphertext
        let cipher = aead::OpeningKey::new(algorithm, &key).unwrap();
        let mut in_out = base64::decode(&ciphertext_base64).unwrap();
        let text_bytes = aead::open_in_place(&cipher, &nonce, &[0; 0], 0, &mut in_out).unwrap();
        String::from_utf8(text_bytes.to_vec()).unwrap()
    };

    assert!(decrypted_data.as_bytes() == original_data);
}