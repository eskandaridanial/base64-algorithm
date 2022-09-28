pub fn to_binary_str(value : String) -> String {
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

pub fn to_decimal(binary_str : i32) -> i32{
    i32::from_str_radix(&binary_str.to_string(), 2).unwrap()
}
