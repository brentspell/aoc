use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, io};

#[derive(Clone)]
struct Tile {
    id: u64,
    image: Vec<Vec<char>>,
}

#[derive(Clone, Copy)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

lazy_static! {
    static ref TILE_REGEX: Regex = Regex::new(r#"^Tile (?P<id>[\d]+):$"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day20.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let tiles: Vec<Tile> = lines.chunks(12).map(Tile::parse).collect();

    println!("part 1: {}", part1(&tiles));
    println!("part 2: {}", part2(&tiles));
}

fn part1(tiles: &[Tile]) -> u64 {
    // count matching edges for all tiles
    let mut counts: HashMap<Vec<char>, u64> = HashMap::new();
    for tile in tiles {
        for edge in &tile.edges() {
            *counts.entry(edge.to_vec()).or_insert(0) += 1;
            *counts.entry(reversed(edge)).or_insert(0) += 1;
        }
    }

    // the corners will only have one matching edge (forward/reversed) on two sides
    let mut result = 1;
    for tile in tiles {
        if tile.edges().iter().filter(|&e| counts[e] > 1).count() == 2 {
            result *= tile.id;
        }
    }

    result
}

fn part2(tiles: &[Tile]) -> usize {
    // precompute matching edge counts and edge->adjacent tile mappings
    let mut counts: HashMap<Vec<char>, u64> = HashMap::new();
    let mut edger: HashMap<Vec<char>, Vec<&Tile>> = HashMap::new();
    for tile in tiles {
        for edge in &tile.edges() {
            *counts.entry(edge.to_vec()).or_insert(0) += 1;
            *counts.entry(reversed(edge)).or_insert(0) += 1;
            edger
                .entry(edge.to_vec())
                .or_insert_with(Vec::new)
                .push(&tile);
            edger
                .entry(reversed(edge))
                .or_insert_with(Vec::new)
                .push(&tile);
        }
    }

    // to construct the grid, first find the tile in the upper-left corner
    // this tile may need to be rotated or flipped, so check all orientations
    // of corner tiles (using the same logic as part 1)
    let tile_dim = tiles[0].image.len();
    let grid_dim = (tiles.len() as f64).sqrt() as usize;
    let mut grid = vec![vec![tiles
        .iter()
        .flat_map(|t| t.orientations())
        .find(|t| counts[&t.edge(Edge::Top)] == 1 && counts[&t.edge(Edge::Left)] == 1)
        .unwrap()]];

    // advance from the top left tile to the right and then downward,
    // finding the unique matching tile at each grid position and orient it
    for i in 0..grid_dim {
        let row = &mut grid[i];
        for j in 1..grid_dim {
            // place the next column in the current row by finding a right/left edge match
            // search all orientations of the new tile until we get an exact edge match
            let left = row[j - 1].clone();
            let tile = edger[&left.edge(Edge::Right)]
                .iter()
                .find(|&&t| t.id != left.id)
                .unwrap()
                .orientations()
                .iter()
                .find(|&t| t.edge(Edge::Left) == left.edge(Edge::Right))
                .unwrap()
                .clone();
            row.push(tile);
        }
        // place the first column in the next row by finding a bottom/top edge match
        // search all orientations of the new tile until we get an exact edge match
        if i < grid_dim - 1 {
            let above = row[0].clone();
            let tile = edger[&above.edge(Edge::Bottom)]
                .iter()
                .find(|&&t| t.id != above.id)
                .unwrap()
                .orientations()
                .iter()
                .find(|&t| t.edge(Edge::Top) == above.edge(Edge::Bottom))
                .unwrap()
                .clone();
            grid.push(vec![tile]);
        }
    }

    // squash the grid down to a single tile, discarding borders of all sub-tiles
    let tile_dim = tile_dim - 2;
    let image: Vec<Vec<char>> = (0..tile_dim * grid_dim)
        .map(|i| {
            (0..tile_dim * grid_dim)
                .map(|j| grid[i / tile_dim][j / tile_dim].image[i % tile_dim + 1][j % tile_dim + 1])
                .collect()
        })
        .collect();

    // apply the sea monster mask to all orientations of the image
    // only one image should contain the mask, so the minimum number of
    // unmasked pixels should give us the answer
    let monster: Vec<Vec<char>> = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .iter()
    .map(|s| s.chars().collect())
    .collect();
    orientations(&image)
        .iter()
        .map(|i| mask(i, &monster))
        .min()
        .unwrap()
}

impl Tile {
    fn parse(lines: &[String]) -> Self {
        Self {
            id: TILE_REGEX
                .captures(&lines[0])
                .unwrap()
                .name("id")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            image: lines[1..lines.len() - 1]
                .iter()
                .map(|l| l.chars().collect())
                .collect(),
        }
    }

    fn edges(&self) -> Vec<Vec<char>> {
        [Edge::Top, Edge::Right, Edge::Bottom, Edge::Left]
            .iter()
            .map(|&e| self.edge(e))
            .collect()
    }

    fn edge(&self, edge: Edge) -> Vec<char> {
        match edge {
            Edge::Top => self.image[0].clone(),
            Edge::Right => self.image.iter().map(|r| r[self.image.len() - 1]).collect(),
            Edge::Bottom => self.image[self.image.len() - 1].clone(),
            Edge::Left => self.image.iter().map(|r| r[0]).collect(),
        }
    }

    fn orientations(&self) -> Vec<Tile> {
        orientations(&self.image)
            .iter()
            .map(|i| Tile {
                id: self.id,
                image: i.clone(),
            })
            .collect()
    }
}

fn orientations(image: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
    let mut image = image.to_vec();
    let mut result = vec![image.clone(), hflip(&image), vflip(&image)];
    for _ in 0..4 {
        image = rotate(&image);
        result.extend(vec![image.clone(), hflip(&image), vflip(&image)]);
    }
    result
}

fn rotate(image: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new = image.to_vec();
    let n = image.len();
    for (i, new) in new.iter_mut().enumerate() {
        for j in 0..n {
            new[j] = image[n - j - 1][i]
        }
    }
    new
}

fn hflip(image: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new = image.to_vec();
    let n = image.len();
    for i in 0..n {
        for j in 0..n {
            new[i][j] = image[i][n - j - 1]
        }
    }
    new
}

fn vflip(image: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new = image.to_vec();
    let n = image.len();
    for i in 0..n {
        for j in 0..n {
            new[i][j] = image[n - i - 1][j]
        }
    }
    new
}

fn mask(image: &[Vec<char>], mask: &[Vec<char>]) -> usize {
    let mut image = image.to_vec();
    for i in 0..image.len() - mask.len() + 1 {
        for j in 0..image[0].len() - mask[0].len() + 1 {
            // search for a full mask match
            let mut match_ = true;
            for u in 0..mask.len() {
                for v in 0..mask[u].len() {
                    if mask[u][v] == '#' && image[i + u][j + v] != '#' {
                        match_ = false;
                    }
                }
            }
            if match_ {
                // erase any masked pixels
                for u in 0..mask.len() {
                    for v in 0..mask[u].len() {
                        if mask[u][v] == '#' {
                            image[i + u][j + v] = '.';
                        }
                    }
                }
            }
        }
    }

    image.iter().flatten().filter(|&&c| c == '#').count()
}

fn reversed(v: &[char]) -> Vec<char> {
    let mut v = v.to_vec();
    v.reverse();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = [
            "Tile 2311:",
            "..##.#..#.",
            "##..#.....",
            "#...##..#.",
            "####.#...#",
            "##.##.###.",
            "##...#.###",
            ".#.#.#..##",
            "..#....#..",
            "###...#.#.",
            "..###..###",
            "",
            "Tile 1951:",
            "#.##...##.",
            "#.####...#",
            ".....#..##",
            "#...######",
            ".##.#....#",
            ".###.#####",
            "###.##.##.",
            ".###....#.",
            "..#.#..#.#",
            "#...##.#..",
            "",
            "Tile 1171:",
            "####...##.",
            "#..##.#..#",
            "##.#..#.#.",
            ".###.####.",
            "..###.####",
            ".##....##.",
            ".#...####.",
            "#.##.####.",
            "####..#...",
            ".....##...",
            "",
            "Tile 1427:",
            "###.##.#..",
            ".#..#.##..",
            ".#.##.#..#",
            "#.#.#.##.#",
            "....#...##",
            "...##..##.",
            "...#.#####",
            ".#.####.#.",
            "..#..###.#",
            "..##.#..#.",
            "",
            "Tile 1489:",
            "##.#.#....",
            "..##...#..",
            ".##..##...",
            "..#...#...",
            "#####...#.",
            "#..#.#.#.#",
            "...#.#.#..",
            "##.#...##.",
            "..##.##.##",
            "###.##.#..",
            "",
            "Tile 2473:",
            "#....####.",
            "#..#.##...",
            "#.##..#...",
            "######.#.#",
            ".#...#.#.#",
            ".#########",
            ".###.#..#.",
            "########.#",
            "##...##.#.",
            "..###.#.#.",
            "",
            "Tile 2971:",
            "..#.#....#",
            "#...###...",
            "#.#.###...",
            "##.##..#..",
            ".#####..##",
            ".#..####.#",
            "#..#.#..#.",
            "..####.###",
            "..#.#.###.",
            "...#.#.#.#",
            "",
            "Tile 2729:",
            "...#.#.#.#",
            "####.#....",
            "..#.#.....",
            "....#..#.#",
            ".##..##.#.",
            ".#.####...",
            "####.#.#..",
            "##.####...",
            "##..#.##..",
            "#.##...##.",
            "",
            "Tile 3079:",
            "#.#.#####.",
            ".#..######",
            "..#.......",
            "######....",
            "####.#..#.",
            ".#...#.##.",
            "#.#####.##",
            "..#.###...",
            "..#.......",
            "..#.###...",
            "",
        ];
        let tiles: Vec<Tile> = lines
            .iter()
            .copied()
            .map(String::from)
            .collect::<Vec<String>>()
            .chunks(12)
            .map(Tile::parse)
            .collect();

        assert_eq!(part1(&tiles), 20899048083289);
        assert_eq!(part2(&tiles), 273);
    }
}
