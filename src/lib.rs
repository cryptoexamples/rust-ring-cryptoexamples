extern crate base64;
extern crate ring;

mod string_encryption_key_based_symmetric;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // This test simply simply runs all the examples to ensure that they compile and work.
        ::string_encryption_key_based_symmetric::main();
    }
}