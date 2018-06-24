extern crate base64;
extern crate ring;

mod errors;
mod string_encryption_key_based_symmetric;

fn main() {
    // This main method simply runs all the examples to ensure that they compile and work.
    ::string_encryption_key_based_symmetric::main().unwrap();
}
