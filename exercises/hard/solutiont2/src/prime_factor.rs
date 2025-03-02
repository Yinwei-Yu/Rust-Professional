pub fn find_max_prime_factor(mut number: u128) -> u128 {
    if number <= 1 {
        return number;
    }
    let mut max_factor = 1;

    // 试除小质数
    let mut i = 2;
    while i <= 1000 && i * i <= number {
        if number % i == 0 {
            max_factor = max_factor.max(i);
            while number % i == 0 {
                number /= i;
            }
        }
        i += 1;
    }

    // 处理剩余部分
    if number > 1 {
        if is_prime(number) {
            max_factor = max_factor.max(number);
        } else {
            let mut factors = vec![number];
            while let Some(n) = factors.pop() {
                if n == 1 {
                    continue;
                }
                if is_prime(n) {
                    max_factor = max_factor.max(n);
                    continue;
                }
                let d = pollards_rho(n);
                if d == n {
                    continue;
                }
                factors.push(d);
                factors.push(n / d);
            }
        }
    }

    max_factor
}

// 辅助函数：快速幂取模
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

// 辅助函数：最大公约数
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Miller-Rabin素数测试
fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    // 使用确定的基集合，适用于n < 2^64
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in &bases {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

// Pollard's Rho算法分解因数
fn pollards_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }
    let mut c = 1;
    loop {
        let f = |x: u128| (mod_pow(x, 2, n) + c) % n;
        let mut x = 2;
        let mut y = 2;
        let mut d = 1;
        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd(x.abs_diff(y), n);
        }
        if d != n {
            return d;
        }
        c += 1;
        if c > 100 {
            return n;
        }
    }
}