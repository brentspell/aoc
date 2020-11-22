struct Password {
    value: Vec<u8>,
}

pub fn solve() {
    let data = std::fs::read_to_string("data/day11.txt").unwrap();

    // part 1
    let result = Password::new(&data).find(|s| is_valid(s)).unwrap();
    println!("part 1: {}", result);

    // part 2
    let result = Password::new(&result).find(|s| is_valid(s)).unwrap();
    println!("part 2: {}", result);
}

impl Password {
    fn new(value: &str) -> Password {
        Password {
            value: String::from(value).into_bytes(),
        }
    }
}

impl Iterator for Password {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        for i in (0..self.value.len()).rev() {
            if self.value[i] == b'z' {
                // wrap and continue to carry
                self.value[i] = b'a';
            } else {
                // increment, skip i, o, l, and stop carrying
                self.value[i] = match self.value[i] {
                    b'h' => b'j',
                    b'n' => b'p',
                    b'k' => b'm',
                    b => b + 1,
                };
                break;
            }
        }

        Some(String::from_utf8(self.value.clone()).unwrap())
    }
}

fn is_valid(password: &str) -> bool {
    let bytes = password.as_bytes();

    // must contain at least one 3-straight
    let straight = bytes
        .iter()
        .zip(&bytes[1..])
        .zip(&bytes[2..])
        .any(|((&a, &b), &c)| a + 1 == b && b + 1 == c);
    if !straight {
        return false;
    }

    // must contain two different pairs
    let mut pairs = 0;
    let mut prev = b' ';
    for (&a, &b) in bytes.iter().zip(&bytes[1..]) {
        if a == b && a != prev {
            pairs += 1;
            prev = a;
        }
    }
    if pairs < 2 {
        return false;
    }

    true
}
