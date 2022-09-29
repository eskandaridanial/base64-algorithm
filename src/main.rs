#[path = "util/utils.rs"] mod utils;

fn main() {
    let value = String::from("abc");
    println!("{}" , utils::encode(value));
}