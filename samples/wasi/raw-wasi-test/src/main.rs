fn main() {
    let thing = match std::env::var("THING") {
        Ok(s) => s,
        Err(_) => "world".to_owned(),
    };
    println!("Hello, {thing}!");
}
