pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let mut split = num_str.split('(');
    let num_str = split.next().unwrap();
    let base_from = split
        .next()
        .map(|s| s.trim_end_matches(')'))
        .unwrap_or("10");
    let base_from: u32 = base_from.parse().unwrap();
    let mut num = u32::from_str_radix(num_str, base_from).unwrap();
    let mut res = String::new();
    while num > 0 {
        let digit = num % to_base;
        res.push(char::from_digit(digit, to_base).unwrap());
        num /= to_base;
    }
    res.chars().rev().collect()
}
