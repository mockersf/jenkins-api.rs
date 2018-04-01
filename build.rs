extern crate skeptic;

fn main() {
    println!("my build.rs");
    // generates doc tests for `README.md`.
    skeptic::generate_doc_tests(&["README.md"]);
}
