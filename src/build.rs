fn main() {
    for i in std::env::vars() {
        println!("aaaaa {}={}", i.0, i.1);
    }
}
