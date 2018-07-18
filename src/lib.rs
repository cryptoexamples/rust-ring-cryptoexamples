extern crate base64;
extern crate ring;

mod string_encryption_key_based_symmetric;

#[cfg(test)]
mod tests {
    #[test]
    fn test_tring_encryption_key_based_symmetric() {
        // This test runs all the examples to ensure that they compile and work.
        ::string_encryption_key_based_symmetric::main();
        // TODO check program output contains a line stating that the encryption and decryption worked
    }
}
