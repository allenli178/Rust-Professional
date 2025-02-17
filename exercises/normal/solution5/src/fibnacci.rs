pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 1;
    while a <= threshold {
        if a % 2 == 1 {
            sum += a;
        }
        let temp = a;
        a = b;
        b = temp + b;
    }
    sum
}
