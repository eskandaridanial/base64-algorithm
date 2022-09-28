#[path = "util/utils.rs"] mod utils;

fn main() {
    let value = String::from("ab");
    let binary_str = utils::to_binary_str(value);
    println!("{}" , binary_str);
}