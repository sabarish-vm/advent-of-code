use core::panic;
use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

fn read_contents(path: &str) -> HashMap<(i32, i32), String> {
    let t = match read_to_string(path) {
        Ok(x) => x,
        Err(x) => panic!("Err : {}", x),
    };
    let mut map: HashMap<(i32, i32), String> = HashMap::new();
    let t = t.split("\n");
    for (x, row) in t.enumerate() {
        for (y, ch) in row.chars().enumerate() {
            map.insert((x as i32, y as i32), ch.to_string());
        }
    }
    map
}

fn get_keys(map: &HashMap<(i32, i32), String>) -> Vec<(i32, i32)> {
    let x: Vec<(i32, i32)> = map
        .iter()
        .filter_map(|(k, v)| if v == "A" { Some(k.clone()) } else { None })
        .collect();
    x
}

fn get_char(map: &HashMap<(i32, i32), String>, ind: &(i32, i32)) -> String {
    let s = map.get(ind);
    match s {
        Some(x) => String::from(x),
        None => "".to_string(),
    }
}

fn soln(map: &HashMap<(i32, i32), String>, keys: &Vec<(i32, i32)>) {
    let mut sol1: usize = 0;
    let mut sol2: usize = 0;
    for (i, j) in keys {
        let r = *i;
        let c = *j;
        let center: String = get_char(map, &(r, c));
        let rows: String = get_char(map, &(r, c - 2))
            + &get_char(map, &(r, c - 1))
            + &center
            + &get_char(map, &(r, c + 1))
            + &get_char(map, &(r, c + 2));
        let cols = get_char(map, &(r - 2, c))
            + &get_char(map, &(r - 2, c))
            + &get_char(map, &(r - 1, c))
            + &center
            + &get_char(map, &(r + 1, c))
            + &get_char(map, &(r + 2, c));
        let offdiag = get_char(map, &(r - 2, c - 2))
            + &get_char(map, &(r - 1, c - 1))
            + &center
            + &get_char(map, &(r + 1, c + 1))
            + &get_char(map, &(r + 2, c + 2));
        let maindiag = get_char(map, &(r - 2, c + 2))
            + &get_char(map, &(r - 1, c + 1))
            + &center
            + &get_char(map, &(r + 1, c - 1))
            + &get_char(map, &(r + 2, c - 2));
        let rows_f = rows.chars().rev().collect::<String>();
        let cols_f = cols.chars().rev().collect::<String>();
        let offdiag_f = offdiag.chars().rev().collect::<String>();
        let maindiag_f = maindiag.chars().rev().collect::<String>();
        let xmas = Regex::new(r"XMAS").unwrap();
        let mas: Regex = Regex::new(r"MAS").unwrap();

        for var in [
            &rows,
            &rows_f,
            &cols,
            &cols_f,
            &maindiag,
            &maindiag_f,
            &offdiag,
            &offdiag_f,
        ] {
            sol1 += xmas.find_iter(var).count();
        }

        let cmd2 = mas.find_iter(&maindiag).count();
        let cmd2f = mas.find_iter(&maindiag_f).count();
        let cod2 = mas.find_iter(&offdiag).count();
        let cod2f = mas.find_iter(&offdiag_f).count();
        if (cmd2 > 0 && cod2 > 0)
            || (cmd2 > 0 && cod2f > 0)
            || (cmd2f > 0 && cod2 > 0)
            || (cmd2f > 0 && cod2f > 0)
        {
            sol2 += 1
        }
    }
    println!("{}", sol1);
    println!("{}", sol2);
}

fn main() {
    // Load the file
    let path = "../input.txt";
    let contents = read_contents(path);
    let akeys = get_keys(&contents);
    soln(&contents, &akeys);
}

#[cfg(test)]
mod tests {
    use super::*; // Bring items from the outer scope into this module
    #[test]
    fn test_example_data() {
        let path = "../test.txt";
        let contents = read_contents(path);
        let akeys = get_keys(&contents);
        // println!("{}", contents.get(&(23, 213)).unwrap_or(&"".to_string()));
        // println!("{:?}", akeys);
        soln(&contents, &akeys);
    }
}
