extern crate roggen;

fn main() {
    use roggen::template::index::index;

    println!("Index: {}", index().into_string());
}