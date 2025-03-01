pub fn goldbach_conjecture() -> String {
    let mut num = 9; // 从第一个奇合数开始
    let mut results = Vec::with_capacity(2);
    
    'main: loop {
        // 跳过非奇合数
        while !is_odd_composite(num) {
            num += 2;
        }
        
        // 检查哥德巴赫猜想
        let max_k = (((num - 2) as f64) / 2.0).sqrt() as u64;
        for k in 1..=max_k {
            let p = num - 2 * k * k;
            if is_prime(p) {
                num += 2;  // 找到有效分解，检查下一个奇合数
                continue 'main;
            }
        }
        
        // 找到反例
        results.push(num);
        if results.len() == 2 {
            break;
        }
        
        num += 2;  // 继续搜索下一个奇合数
    }
    
    format!("{},{}", results[0], results[1])
}

// 奇合数判断函数
fn is_odd_composite(n: u64) -> bool {
    n % 2 == 1 && n > 1 && !is_prime(n)
}

// 素数判断
fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}