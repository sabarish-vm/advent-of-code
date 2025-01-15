use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_rules(rules_path: &str) -> Vec<Vec<i32>> {
    let f = File::open(rules_path).unwrap_or_else(|e| panic!("File read error {}", e));
    let bf = BufReader::new(f);
    let x: Vec<Vec<i32>> = bf
        .lines()
        .map(|line| {
            let t = line.unwrap_or_else(|e| panic!("Error unwrapping lines {}", e));
            t.split("|")
                .map(|part| {
                    part.parse::<i32>()
                        .unwrap_or_else(|e| panic!("Parsing error {}", e))
                })
                .collect::<Vec<i32>>()
        })
        .collect();
    x
}

fn read_seq(seq_path: &str) -> Vec<Vec<i32>> {
    let f = File::open(seq_path).unwrap_or_else(|e| panic!("Seq file read error : {}", e));
    let bf = BufReader::new(f);
    let x: Vec<Vec<i32>> = bf
        .lines()
        .map(|line| {
            let t = line.unwrap_or_else(|e| panic!("Error unwrapping lines {}", e));
            t.split(",")
                .map(|part| {
                    part.parse::<i32>()
                        .unwrap_or_else(|e| panic!("Parsing error {}", e))
                })
                .collect::<Vec<i32>>()
        })
        .collect();
    x
}

fn create_map(data: &[Vec<i32>]) -> HashMap<&i32, Vec<&i32>> {
    let mut sdic: HashMap<&i32, Vec<&i32>> = HashMap::new();
    for r in 0..data.len() {
        let n1 = &data[r][0];
        let x: Vec<&i32> = data
            .iter()
            .filter(|x| &x[0] == n1)
            .map(|x| &x[1])
            .collect::<Vec<&i32>>();
        sdic.insert(n1, x);
    }
    sdic
}

fn solution(sdic: HashMap<&i32, Vec<&i32>>, seq: &[Vec<i32>]) {
    let mut res1: i32 = 0;
    for l in seq {
        let llen = l.len();
        let mut resbool = true;
        for i in 0..(llen - 1) {
            let x = &l[i];
            let y = &l[i + 1];
            if sdic[x].contains(&y) {
            } else {
                resbool = false;
            }
        }
        if resbool {
            res1 += l[llen / 2];
        } else {
            for i in 0..(llen - 1) {
                // TODO : Implement this
            }
        }
    }
    println!("Part1 = {}", res1);
}

fn main() {
    let rules = read_rules("../../input_rules.txt");
    let sequence = read_seq("../../input_seq.txt");
    println!("{:?}", rules[0]);
    println!("{:?}", sequence[0]);
    let map = create_map(&rules);
    println!("sdic[32] = {:?}", map.get(&32).unwrap());
    solution(map, &sequence);
}
