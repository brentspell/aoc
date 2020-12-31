const CARD_PUBLIC_KEY: u64 = 3469259;
const DOOR_PUBLIC_KEY: u64 = 13170438;

pub fn solve() {
    println!("part 1: {}", part1());
}

fn part1() -> u64 {
    derive_key(derive_loop(CARD_PUBLIC_KEY), DOOR_PUBLIC_KEY)
}

fn derive_loop(public_key: u64) -> u64 {
    let mut k = 1;
    let mut i = 0;
    while k != public_key {
        k = k * 7 % 20201227;
        i += 1;
    }
    i
}

fn derive_key(loop_size: u64, public_key: u64) -> u64 {
    let mut k = 1;
    for _ in 0..loop_size {
        k = k * public_key % 20201227;
    }
    k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(derive_loop(5764801), 8);
        assert_eq!(derive_loop(17807724), 11);
        assert_eq!(derive_key(8, 17807724), 14897079);
        assert_eq!(derive_key(11, 5764801), 14897079);
    }
}
