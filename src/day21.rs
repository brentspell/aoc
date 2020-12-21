use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{fs, io};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"^(?P<id>[\d]+): (?P<expr>.*)$"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day21.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[String]) -> u32 {
    // track the allergen->ingredient set mapping, reducing each set
    // whenever a new rule for the same allergen is encountered
    // also keep track of the rule count for every ingredient for output
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split(" (").collect();
        let ingredients: HashSet<&str> = split[0].split(' ').collect();
        for ingredient in &ingredients {
            *counts.entry(ingredient).or_insert(0) += 1;
        }
        for allergen in split[1][9..split[1].len() - 1].split(", ") {
            if let Some(e) = map.get_mut(allergen) {
                *e = e.intersection(&ingredients.clone()).copied().collect();
            } else {
                map.insert(allergen, ingredients.clone());
            }
        }
    }

    // get the final set of unsafe ingredients
    let nonsafe = map
        .values()
        .fold(HashSet::new(), |a, x| a.union(&x).copied().collect());

    // count the rules for each safe ingredient
    counts
        .iter()
        .filter(|&(k, _v)| !nonsafe.contains(k))
        .map(|(_k, v)| v)
        .sum()
}

fn part2(lines: &[String]) -> String {
    // track the allergen->ingredient set mapping, just like in part 1
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split(" (").collect();
        let ingredients: HashSet<&str> = split[0].split(' ').collect();
        for allergen in split[1][9..split[1].len() - 1].split(", ") {
            if let Some(e) = map.get_mut(allergen) {
                *e = e.intersection(&ingredients.clone()).copied().collect();
            } else {
                map.insert(allergen, ingredients.clone());
            }
        }
    }

    // filter down the ingredient list for each allergen
    // to a single ingredient by repeated removal of each unique ingredient
    // from all the other allergen rules
    let mut removed = true;
    while removed {
        removed = false;
        for (allergen, ingredients) in map.clone() {
            if ingredients.len() == 1 {
                let ingredient = *ingredients.iter().next().unwrap();
                for (&other, ingredients) in map.iter_mut() {
                    if other != allergen && ingredients.remove(ingredient) {
                        removed = true;
                    }
                }
            }
        }
    }

    // sort by allergen and return the unsafe ingredients
    let mut allergens: Vec<&str> = map.keys().copied().collect();
    allergens.sort_unstable();
    allergens
        .iter()
        .map(|k| *map[k].iter().next().unwrap())
        .collect::<Vec<&str>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = [
            String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            String::from("trh fvjkl sbzzf mxmxvkd (contains dairy)"),
            String::from("sqjhc fvjkl (contains soy)"),
            String::from("sqjhc mxmxvkd sbzzf (contains fish)"),
        ];

        assert_eq!(part1(&lines), 5);
        assert_eq!(part2(&lines), "mxmxvkd,sqjhc,fvjkl");
    }
}
