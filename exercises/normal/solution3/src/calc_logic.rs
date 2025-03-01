pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    if n > 365 {
        return 1.0;
    } else {
        let mut diffrent: f64 = 1.0;
        for i in 0..n {
            diffrent *=(365-i) as f64 /365.0;
        }
        return 1.0-diffrent;
    }
}
