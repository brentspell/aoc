use regex::Regex;
use std::io::prelude::*;
use std::{cmp, fs, io};

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r#"^(?P<name>[\w]+?): capacity (?P<capacity>[-\d]+?), durability (?P<durability>[-\d]+?), flavor (?P<flavor>[-\d]+?), texture (?P<texture>[-\d]+?), calories (?P<calories>[-\d]+?)$"#
    )
    .unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day15.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs: Vec<Ingredient> = lines.iter().map(|l| l.parse().unwrap()).collect();

    let mut result1 = 0;
    let mut result2 = 0;
    for amounts in combinations(&inputs, 100) {
        let (score, calories) = score(&inputs, &amounts);
        result1 = cmp::max(result1, score);
        if calories == 500 {
            result2 = cmp::max(result2, score);
        }
    }

    // part 1
    println!("part 1: {}", result1);

    // part 2
    println!("part 2: {}", result2);
}

impl std::str::FromStr for Ingredient {
    type Err = Box<dyn std::error::Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let caps = REGEX.captures(line).unwrap();
        Ok(Ingredient {
            name: caps.name("name").unwrap().as_str().to_string(),
            capacity: caps.name("capacity").unwrap().as_str().parse().unwrap(),
            durability: caps.name("durability").unwrap().as_str().parse().unwrap(),
            flavor: caps.name("flavor").unwrap().as_str().parse().unwrap(),
            texture: caps.name("texture").unwrap().as_str().parse().unwrap(),
            calories: caps.name("calories").unwrap().as_str().parse().unwrap(),
        })
    }
}

fn combinations(inputs: &[Ingredient], max: i32) -> Vec<Vec<i32>> {
    if inputs.len() == 1 {
        vec![vec![max]]
    } else {
        let mut vec: Vec<Vec<i32>> = Vec::new();
        for i in 0..=max {
            for v in combinations(&inputs[1..], max - i) {
                let mut v2: Vec<i32> = Vec::with_capacity(v.len() + 1);
                v2.push(i);
                v2.extend_from_slice(&v);
                vec.push(v2);
            }
        }
        vec
    }
}

fn score(inputs: &[Ingredient], amounts: &[i32]) -> (i32, i32) {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;
    for (i, s) in inputs.iter().zip(amounts) {
        capacity += i.capacity * s;
        durability += i.durability * s;
        flavor += i.flavor * s;
        texture += i.texture * s;
        calories += i.calories * s;
    }

    capacity = cmp::max(capacity, 0);
    durability = cmp::max(durability, 0);
    flavor = cmp::max(flavor, 0);
    texture = cmp::max(texture, 0);

    (capacity * durability * flavor * texture, calories)
}
