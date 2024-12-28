use std::fs::read_to_string;

fn extract_data(data: &str) -> Vec<Vec<i32>> {
    let lines: Vec<Vec<i32>> = data
        .split("\n")
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .filter(|x: &Vec<i32>| !x.is_empty())
        .collect();
    lines
}

fn check(x: &[i32]) -> bool {
    let diff: Vec<i32> = x.windows(2).map(|w| w[1] - w[0]).collect();
    let mono_inc: bool = diff.iter().all(|&e| e > 0);
    let mono_dec: bool = diff.iter().all(|&e| e < 0);
    let maxdiff: bool = diff.iter().all(|e| e.abs() < 4);
    let res: bool = maxdiff && (mono_dec || mono_inc);
    res
}

fn problem(data: &[Vec<i32>]) {
    let mut res: i32 = 0;
    let mut fixes: i32 = 0;
    data.iter().for_each(|v| {
        if check(v) {
            res += 1
        } else {
            for i in 0..v.len() {
                let mut v1 = v.clone();
                let _ = v1.remove(i);
                if check(&v1) {
                    fixes += 1;
                    break;
                }
            }
        }
    });
    println!("Part1 :\n Total = {}", res);
    println!("Part2 :\n Fixable = {} \n Total = {}", fixes, res + fixes);
}

fn main() {
    let path = "../input.txt";
    let contents = read_to_string(path).unwrap();
    let data = extract_data(&contents);
    problem(&data);
}
