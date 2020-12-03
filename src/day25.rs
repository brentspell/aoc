const CODE_ROW: u64 = 2947;
const CODE_COL: u64 = 3029;

const CODE_START: u64 = 20151125;
const CODE_MUL: u64 = 252533;
const CODE_MOD: u64 = 33554393;

pub fn solve() {
    // part 1
    let exp = position(CODE_ROW, CODE_COL);
    let result = CODE_START * expmod(CODE_MUL, exp, CODE_MOD) % CODE_MOD;
    println!("part 1: {}", result);
}

fn position(row: u64, col: u64) -> u64 {
    (1..row + col).into_iter().sum::<u64>() - row
}

fn expmod(x: u64, e: u64, m: u64) -> u64 {
    let mut x = x;
    let mut e = e;
    let mut y = 1;
    while e > 0 {
        if e % 2 == 0 {
            x = (x * x) % m;
            e /= 2;
        } else {
            y = (x * y) % m;
            e -= 1;
        }
    }
    y
}
