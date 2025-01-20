use core::panic;
use itertools::Itertools;
use std::{
    collections::{btree_map::Range, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

fn update_map(map: &mut HashMap<char, Vec<(usize, usize)>>, c: &char, coords: &(usize, usize)) {
    if *c != '.' {
        map.entry(*c).or_insert(vec![]).push(*coords);
    }
}

fn read_data(path: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(usize, usize)>>) {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let f = File::open(path).unwrap_or_else(|e| panic!("###### File read error {}", e));
    let bf = BufReader::new(f);
    let mut data: Vec<Vec<char>> = vec![];
    bf.lines().enumerate().for_each(|(row, line)| {
        let mut rowc: Vec<char> = vec![];
        line.unwrap_or_else(|e| panic!("Error unwrapping lines {}", e))
            .chars()
            .enumerate()
            .for_each(|(col, c)| {
                update_map(&mut map, &c, &(row, col));
                rowc.push(c);
            });
        data.push(rowc);
    });
    (data, map)
}

fn combinations(n: &usize) -> Vec<Vec<usize>> {
    let seq = 0..*n;
    let x = seq.into_iter().combinations(2);
    x.collect_vec()
}

fn check_bounds<T>(pos: &(Option<usize>, Option<usize>), data: &[Vec<T>]) -> bool {
    let (_px, _py) = pos;
    if _px.is_none() || _py.is_none() {
        return false;
    } else {
        let (px, py) = (_px.unwrap(), _py.unwrap());
        let xexists = data.get(px).is_some();
        if xexists {
            return data.get(px).unwrap().get(py).is_some();
        } else {
            return false;
        }
    }
}

fn find_antinodes(
    p1: (&usize, &usize),
    p2: (&usize, &usize),
    dx: &usize,
    dy: &usize,
    data: &[Vec<char>],
) -> (
    (Option<usize>, Option<usize>),
    (Option<usize>, Option<usize>),
) {
    let (p1x, p1y) = p1;
    let (p2x, p2y) = p2;
    let mut a1x: Option<usize> = None;
    let mut a1y: Option<usize> = None;
    let mut a2x: Option<usize> = None;
    let mut a2y: Option<usize> = None;

    if p1x < p2x {
        a1x = p1x.checked_sub(*dx);
        a2x = p2x.checked_add(*dx);
    }
    if p1x > p2x {
        a1x = p1x.checked_add(*dx);
        a2x = p2x.checked_sub(*dx);
    }
    if p1y < p2y {
        a1y = p1y.checked_sub(*dy);
        a2y = p2y.checked_add(*dy);
    }
    if p1y > p2y {
        a1y = p1y.checked_add(*dy);
        a2y = p2y.checked_sub(*dy);
    }
    ((a1x, a1y), (a2x, a2y))
}

fn problem1(
    map: &HashMap<char, Vec<(usize, usize)>>,
    data: &[Vec<char>],
) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let keys: Vec<&char> = map.keys().collect();
    for key in keys {
        let positions = map.get(key).unwrap_or_else(|| panic!("Key not found"));
        let combs = combinations(&positions.len());
        for comb_i in combs {
            let (p1x, p1y) = positions.get(comb_i[0]).unwrap();
            let (p2x, p2y) = positions.get(comb_i[1]).unwrap();
            let dx = p1x.abs_diff(*p2x);
            let dy = p1y.abs_diff(*p2y);
            let (a1, a2) = find_antinodes((&p1x, &p1y), (&p2x, &p2y), &dx, &dy, data);
            let ea1 = check_bounds(&a1, data);
            let ea2 = check_bounds(&a2, data);
            if ea1 {
                let (a1x, a1y) = a1;
                let a1x = a1x.unwrap();
                let a1y = a1y.unwrap();
                let _ = antinodes.insert((a1x, a1y));
            }
            if ea2 {
                let (a2x, a2y) = a2;
                let a2x = a2x.unwrap();
                let a2y = a2y.unwrap();
                let _ = antinodes.insert((a2x, a2y));
            }
        }
    }
    antinodes
}

fn main() {
    let path = "../input.txt";
    let (data, map) = read_data(path);
    let res = problem1(&map, &data);
    println!("{}", res.len());
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t_read() {
        let path = "../test.txt";
        let (data, map) = read_data(path);
        println!("{}", data[5][6]);
        assert_eq!('A', data[5][6]);
        let length = map[&'0'].len();
        assert_eq!(4, length);
    }

    #[test]
    fn t_comb() {
        println!("Here");
        let r = combinations(&4);
        println!("{:?}", r)
    }

    #[test]
    fn t_indexbounds() {
        let path = "../test.txt";
        let (data, map) = read_data(path);
        println!("Index bounds");
        assert_eq!(true, check_bounds(&(Some(3usize), Some(1usize)), &data));
        assert_eq!(true, check_bounds(&(Some(3usize), Some(1usize)), &data));
        assert_eq!(false, check_bounds(&(Some(23usize), Some(1usize)), &data));
    }

    #[test]
    fn t_p1() {
        let path = "../test.txt";
        let (data, map) = read_data(path);
        let res = problem1(&map, &data);
        println!("{}", res.len());
    }
}
