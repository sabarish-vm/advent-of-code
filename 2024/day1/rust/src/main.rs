use core::panic;
use ndarray::Array;

fn problem1(matrix: &Vec<Vec<f64>>) {
    let mut col0: Vec<f64> = matrix.iter().map(|row| row[0]).collect();
    let mut col1: Vec<f64> = matrix.iter().map(|row| row[1]).collect();
    col0.sort_by(|a: &f64, b| a.partial_cmp(b).unwrap());
    col1.sort_by(|a: &f64, b| a.partial_cmp(b).unwrap());
    let array0 = Array::from_vec(col0);
    let array1 = Array::from_vec(col1);
    let diff = array1 - array0;
    let ans = diff.abs().sum();
    println!("Part1 = {}", ans);
}

fn problem2(matrix: &Vec<Vec<f64>>) {
    let mut counts: Vec<f64> = Vec::new();
    for irow in 0..matrix.len() {
        let val1 = matrix[irow][0];
        let count1 = matrix.iter().filter(|el| el[1] == val1).count();
        counts.push(val1 * count1 as f64)
    }
    let ans: f64 = counts.iter().sum();
    println!("Part2 = {}", ans);
}

fn main() {
    let contents = match std::fs::read_to_string(
        "/Users/sabarish/Documents/Computing/advent24/day1/input.txt",
    ) {
        Ok(x) => x,
        Err(x) => panic!("Error reading file {x}"),
    };
    let matrix: Vec<Vec<f64>> = contents
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<f64>().unwrap())
                .collect()
        })
        .collect();
    problem1(&matrix);
    problem2(&matrix);
}
