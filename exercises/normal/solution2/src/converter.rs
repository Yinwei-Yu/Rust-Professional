pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    let parts:Vec<&str>=num_str.split('(').collect();
    let number=parts[0];
    let from_base=parts[1].trim_end_matches(')').parse::<u32>().unwrap();
    let num_in_decimal=u32::from_str_radix(number, from_base).unwrap();

    let mut result=String::new();
    let mut num=num_in_decimal;
    while num>0 {
        result.push(std::char::from_digit(num%to_base,to_base).unwrap());
        num/=to_base;
    }

    result.chars().rev().collect()
}