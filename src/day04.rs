pub fn solve() {
    let input = std::fs::read_to_string("data/day04.txt")
        .unwrap();

    // part 1
    let result = (0..)
        .skip_while(|&i| {
            let h = hash(&input, i);
            h[0] != 0 || h[1] != 0 || (h[2] & 0xF0) != 0
        })
        .next()
        .unwrap();

    println!("part 1: {}", result);

    // part 2
    let result = (0..)
        .skip_while(|&i| {
            let h = hash(&input, i);
            h[0] != 0 || h[1] != 0 || h[2] != 0
        })
        .next()
        .unwrap();

    println!("part 2: {}", result);
}

fn hash(input: &String, i: i32) -> md5::Digest {
    let data: Vec<u8> = format!("{}{}", input, i).bytes().collect();
    md5::compute(data)
}
