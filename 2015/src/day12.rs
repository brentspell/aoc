use serde_json::Value as Json;

pub fn solve() {
    let data = serde_json::from_str(&std::fs::read_to_string("data/day12.txt").unwrap()).unwrap();

    // part 1
    let result = sum1(&data);
    println!("part 1: {}", result);

    // part 2
    let result = sum2(&data);
    println!("part 2: {}", result);
}

fn sum1(value: &Json) -> i64 {
    match value {
        Json::Number(n) => n.as_i64().unwrap(),
        Json::Array(v) => v.iter().map(sum1).sum(),
        Json::Object(m) => m.values().map(sum1).sum(),
        _ => 0,
    }
}

fn sum2(value: &Json) -> i64 {
    match value {
        Json::Number(n) => n.as_i64().unwrap(),
        Json::Array(v) => v.iter().map(sum2).sum(),
        Json::Object(m) => {
            let mut s = 0;
            for v in m.values() {
                if v == "red" {
                    return 0;
                }
                s += sum2(v);
            }
            s
        }
        _ => 0,
    }
}
