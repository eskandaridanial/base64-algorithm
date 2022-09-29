fn to_binary_str(value : String) -> String {
    let mut binary_string= String::new();
    let chars: Vec<char> = value.chars().collect();
    for char in chars {
        let decimal = char as i32;
        binary_string.push_str(format!("{:0>8}", to_binary(decimal)).as_str());
    }
    binary_string
}

fn to_binary(decimal: i32) -> i32 {
    if decimal == 0 {
        return decimal;
    }
    decimal % 2 + 10 * to_binary(decimal / 2)
}

fn to_decimal(binary_str : String) -> usize{
    usize::from_str_radix(&binary_str, 2).unwrap()
}

pub fn encode(value : String) -> String {
    let base64chars = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let binary_str = to_binary_str(value);
    let mut index = 0;
    let mut single_byte = String::new();
    let mut base64str = String::new();
    while index != binary_str.len() {
        if index + 6 > binary_str.len() {
            single_byte = binary_str[index..binary_str.len()].to_string();
            index = binary_str.len();
        } else {
            single_byte = binary_str[index.. index + 6].to_string();
            index += 6;
        }
        let decimal = to_decimal(single_byte);
        base64str.push_str(&*(base64chars.as_bytes()[decimal] as char).to_string());
    }
    base64str
}