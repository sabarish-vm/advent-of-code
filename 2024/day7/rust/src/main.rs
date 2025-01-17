use core::panic;
use itertools::Itertools;
use std::fs;

fn read_data(path: &str) -> (Vec<i64>, Vec<Vec<i64>>) {
    let contents =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("###### File read error {e}"));
    let mut totals: Vec<i64> = vec![];
    let mut numbers: Vec<Vec<i64>> = vec![];
    contents.lines().for_each(|l| {
        let mut colonsplit = l.split(":");
        let tot = colonsplit
            .next()
            .unwrap_or_else(|| panic!("#### SplitError "))
            .parse::<i64>()
            .unwrap();
        totals.push(tot);
        let num = colonsplit
            .next()
            .unwrap_or_else(|| panic!("###### Here"))
            .split_whitespace()
            .map(|n| {
                n.parse::<i64>()
                    .unwrap_or_else(|_| panic!("### Parse Error here"))
            })
            .collect::<Vec<i64>>();
        numbers.push(num)
    });
    (totals, numbers)
}

fn generate_sequences<'a>(n: &usize, seq: &'a [char; 2]) -> Vec<Vec<&'a char>> {
    let combs = (0..*n)
        .map(|_| seq.iter())
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect::<Vec<&char>>())
        .collect::<Vec<Vec<&char>>>();
    combs
}

fn store_sequences<'a>(data: &[Vec<i64>], seq: &'a [char; 2]) -> Vec<Vec<Vec<&'a char>>> {
    let mut all = vec![];
    for row in data {
        let rowseq = generate_sequences(&(row.len() - 1), seq);
        all.push(rowseq);
    }
    all
}

fn check_row(tot: &i64, list: &[i64]) -> bool {
    let seq = ['+', '*'];
    let chars = generate_sequences(&(list.len() - 1_usize), &seq);
    let mut res = false;
    // println!("{chars:?}");
    for s in chars.iter() {
        // println!("s={s:?}");
        let mut cur = list[0];
        for (id, &c) in s.iter().enumerate() {
            // println!("{c:?}");
            if *c == '+' {
                cur += list[id + 1];
                // println!("{cur:?}");
            } else if *c == '*' {
                cur *= list[id + 1];
            }
        }
        if cur == *tot {
            res = true;
            break;
        }
    }
    res
}

fn problem1(totals: &[i64], numbers: &[Vec<i64>]) -> i64 {
    assert_eq!(totals.len(), numbers.len());
    let mut result = 0;
    for row_id in 0..totals.len() {
        if check_row(&totals[row_id], &numbers[row_id]) {
            result += totals[row_id];
        }
    }
    result
}

fn main() {
    let seq: [char; 2] = ['+', '*'];
    let path = "../input.txt";
    let (totals, numbers) = read_data(path);
    let res = problem1(&totals, &numbers);
    println!("Problem result = {res}");
}

#[cfg(test)]
mod tests {
    use crate::{check_row, generate_sequences, problem1, read_data};

    #[test]
    fn t_read() {
        let path = "../test.txt";
        let (totals, numbers) = read_data(path);
        println!("Total_0 = {}", totals[0]);
        println!("numbers[0,1] = {},{}", numbers[0][0], numbers[0][1]);
        assert_eq!(totals[0], 190, "Pre colon number");
        assert_eq!((numbers[0][0], numbers[0][1]), (10, 19))
    }

    #[test]
    fn t_plusstar() {
        let n: usize = 2;
        let seq = ['+', '*'];
        let res = generate_sequences(&n, &seq);
        println!("{:?}", res);
        assert_eq!('+', *res[0][0]);
        assert_eq!('*', *res[2][0]);
    }

    #[test]
    fn t_rowcheck() {
        let tot = 190;
        let list = [10, 19];
        let res = check_row(&tot, &list);
        println!("{res}")
    }

    #[test]
    fn t_p1() {
        let path = "../test.txt";
        let (totals, numbers) = read_data(path);
        let res = problem1(&totals, &numbers);
        println!("Test problem result = {res}");
        assert_eq!(res, 3749, "Checking output of test problem 1 ")
    }
}
