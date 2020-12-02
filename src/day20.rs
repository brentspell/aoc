const PRESENTS: i32 = 33100000;

pub fn solve() {
    // robin's theorem lower bound
    let mut result = 5041;
    loop {
        let n = result as f32;
        let bound = (0.57721_f32).exp() * n * n.ln().ln() + 0.6483 * n / n.ln().ln();
        if bound as i32 * 10 >= PRESENTS {
            break;
        }
        result += 1;
    }

    // part 1
    while factsum(result) * 10 < PRESENTS {
        result += 1;
    }
    println!("part 1: {}", result);

    // part 2
    while factsum2(result) * 11 < PRESENTS {
        result += 1;
    }
    println!("part 1: {}", result);
}

// https://www.math.upenn.edu/~deturck/m170/wk3/lecture/sumdiv.html
fn factsum(x: i32) -> i32 {
    let mut x = x;
    let mut sum = 1;

    // first find all divisors that are powers of two
    let i = 2;
    let mut p = 1;
    let mut q = 1;
    while x % i == 0 {
        x /= i;
        q *= i;
        p += q;
    }
    sum *= p;

    // then we can skip all multiples of two to find the prime divisors
    let max = (x as f32).sqrt() as i32 + 1;
    for i in (3..max).step_by(2) {
        p = 1;
        q = 1;
        while x % i == 0 {
            x /= i;
            q *= i;
            p += q;
        }
        sum *= p
    }

    // if anything remains, it is a prime divisor
    if x > 2 {
        sum *= 1 + x
    }

    sum
}

// brute force enumerate divisors
// ignore any elves that stop before reaching this house
fn factsum2(x: i32) -> i32 {
    (1..(x as f32).sqrt() as i32 + 1)
        .into_iter()
        .filter(|i| x % i == 0)
        .flat_map(|i| if i != x / i { vec![i, x / i] } else { vec![i] })
        .filter(|i| i * 50 >= x)
        .sum()
}
