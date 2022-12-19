use std::cmp::min;

pub fn solve() {
    let data = std::fs::read_to_string("data/day17.txt").expect("data-file");

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    // cycle offset/length determined empirically using autocorrelation
    println!("part 2: {}", part2(&data, 5549, 2626));
}

fn part1(jets: &str) -> usize {
    simulate(jets, 2022, 10000, 10000)
}

fn part2(jets: &str, height_cycle_offset: usize, height_cycle_length: usize) -> usize {
    simulate(
        jets,
        1000000000000,
        height_cycle_offset,
        height_cycle_length,
    )
}

fn simulate(
    jets: &str,
    steps: usize,
    height_cycle_offset: usize,
    height_cycle_length: usize,
) -> usize {
    let jets: Vec<_> = jets.chars().collect();
    let mut stack: Vec<u32> = Vec::new();

    // used to determine the length/offset (in steps) of the stack height cycles
    let mut cycle_count = 0;
    let mut step_cycle_offset = 0;
    let mut step_cycle_length = 0;

    let mut i = 0; // step counter
    let mut j = 0; // current jet index
    while i < steps {
        let shape = &SHAPES[i % SHAPES.len()];
        let mut h = stack.len() + 3;
        let mut s = 0;
        loop {
            let jet = jets[j % jets.len()];
            j += 1;

            // determine whether we can shift the rock to the left/right
            // based on whether any of its leftmost/rightmost bits would hit a wall
            let mut sj = 0;
            if jet == '<' && !shape.iter().any(|&x| shift(x, s) & 0b1000000 != 0) {
                sj = -1;
            } else if jet == '>' && !shape.iter().any(|&x| shift(x, s) & 0b0000001 != 0) {
                sj = 1
            };

            // determine if the shift is valid based on whether it would cause a
            // collision with an existing settled rock
            if !collision(&stack, shape, s + sj, h) {
                s += sj;
            }

            // attempt to drop the rock one level
            // stop if we reach the floor or a collision with another rock would occur
            if h == 0 || collision(&stack, shape, s, h - 1) {
                // the shape may have fallen through the top of the existing stack,
                // so "or" together any overlapping bits
                for k in 0..min(shape.len(), stack.len() - h) {
                    stack[h + k] |= shift(shape[k], s);
                }

                // for any remaining layers of the shape, extend the height of the stack
                if h <= stack.len() {
                    for &x in shape.iter().skip(stack.len() - h) {
                        stack.push(shift(x, s));
                    }
                }
                break;
            }
            h -= 1;
        }
        i += 1;

        // the settled rock formation eventually cycles
        // the height cycle is determined empirically via autocorrelation
        // first, find the starting step for the first height cycle
        if stack.len() > height_cycle_offset && step_cycle_offset == 0 {
            step_cycle_offset = i;
        }

        // next, find the length of the height cycle, in steps
        // use this to skip over all cycles within the desired number of
        // total steps
        if stack.len() > height_cycle_offset + height_cycle_length && step_cycle_length == 0 {
            let step_length = i - step_cycle_offset;
            step_cycle_length = steps - step_cycle_offset;
            cycle_count = step_cycle_length / step_length - 1;
            i += cycle_count * step_length;
        }
    }

    // the total height of the stack is the height of each cycle
    // plus the number of steps needed to reach the start of the height cycle
    stack.len() + cycle_count * height_cycle_length
}

fn collision(stack: &[u32], shape: &[u32], s: i32, h: usize) -> bool {
    if h < stack.len() {
        let overlap = min(shape.len(), stack.len() - h);
        (0..overlap).any(|k| shift(shape[k], s) & stack[h + k] != 0)
    } else {
        false
    }
}

fn shift(x: u32, n: i32) -> u32 {
    match n.signum() {
        -1 => x << -n,
        1 => x >> n,
        _ => x,
    }
}

// use bitsets (vertically reversed) to represent each shape
lazy_static! {
    static ref SHAPES: Vec<Vec<u32>> = vec![
        // ####
        vec![0b0011110],
        // .#.
        // ###
        // .#.
        vec![0b0001000, 0b0011100, 0b0001000],
        // ..#
        // ..#
        // ###
        vec![0b0011100, 0b0000100, 0b0000100],
        // #
        // #
        // #
        // #
        vec![0b0010000, 0b0010000, 0b0010000, 0b0010000],
        // ##
        // ##
        vec![0b0011000, 0b0011000]
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 3068);
    }

    #[test]
    fn test_part2() {
        // cycle offset/length determined empirically using autocorrelation
        assert_eq!(part2(DATA, 131, 53), 1514285714288);
    }
}
