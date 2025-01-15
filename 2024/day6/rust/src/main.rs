use core::panic;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_data(path: &str) -> Vec<Vec<char>> {
    let f = File::open(path).unwrap_or_else(|e| panic!("###### File read error {}", e));
    let bf = BufReader::new(f);
    let x: Vec<Vec<char>> = bf
        .lines()
        .map(|line| {
            let t = line.unwrap_or_else(|e| panic!("Error unwrapping lines {}", e));
            t.chars().collect::<Vec<char>>()
        })
        .collect();
    x
}

fn initial_position(
    data: &[Vec<char>],
) -> (Option<usize>, Option<usize>, (Option<i32>, Option<i32>)) {
    let mut rowid: Option<usize> = None;
    let mut colid: Option<usize> = None;
    let mut direction: (Option<i32>, Option<i32>) = (None, None);
    for (xid, line) in data.iter().enumerate() {
        let cid = line.iter().position(|&char| char == '^');
        if cid.is_some() {
            colid = cid;
            rowid = Some(xid);
            direction = (Some(-1), Some(0));
            break;
        }
        let cid = line.iter().position(|&char| char == '>');
        if cid.is_some() {
            colid = cid;
            rowid = Some(xid);
            direction = (Some(0), Some(1));
            break;
        }
        let cid = line.iter().position(|&char| char == '<');
        if cid.is_some() {
            colid = cid;
            rowid = Some(xid);
            direction = (Some(0), Some(-1));
            break;
        }
    }
    (rowid, colid, direction)
}

fn index_exists(data: &[Vec<char>], x: &usize, y: &usize) -> bool {
    let xexists: bool = data.get(*x).is_some();
    let mut yexists: bool = false;
    if xexists {
        yexists = data[*x].get(*y).is_some();
    }
    xexists && yexists
}

fn rotate(x: &mut i32, y: &mut i32) {
    let xt = *x;
    *x = *y;
    *y = -xt;
}

fn problem1(
    data: &[Vec<char>],
    rowid: &usize,
    colid: &usize,
    direction: (i32, i32),
) -> HashSet<(usize, usize)> {
    let mut x = *rowid;
    let mut y = *colid;
    let mut nx;
    let mut ny;
    let mut idex = index_exists(data, &x, &y);
    (nx, ny) = direction;
    let mut places = HashSet::new();

    while idex {
        if data[x][y] == '#' {
            x = (x as i32 - nx) as usize;
            y = (y as i32 - ny) as usize;
            rotate(&mut nx, &mut ny);
        } else {
            places.insert((x, y));
        }
        x = (x as i32 + nx) as usize;
        y = (y as i32 + ny) as usize;
        idex = index_exists(data, &x, &y);
    }
    places
}

fn main() {
    let path = "../input.txt";
    let contents = read_data(path);
    let (x, y, (nx, ny)) = initial_position(&contents);
    let places = problem1(
        &contents,
        &x.unwrap(),
        &y.unwrap(),
        (nx.unwrap(), ny.unwrap()),
    );
    println!("No. of places = {}", places.len())
}

#[cfg(test)]
mod tests {
    use super::*; // Bring items from the outer scope into this module

    #[test]
    fn test_rotate() {
        let mut nx = 0;
        let mut ny = 1;
        rotate(&mut nx, &mut ny);
        assert_eq!((1, 0), (nx, ny));
    }

    #[test]
    fn test_data() {
        let path = "../test.txt";
        let contents = read_data(path);
        let (x, y, (nx, ny)) = initial_position(&contents);
        println!("{}", contents[0][1]);
        assert_eq!(
            '^', contents[6][4],
            "Testing if the starting position of the '^' character is determined properly"
        );
        assert_eq!(
            '.', contents[0][1],
            "Testing if the first character read is '.'"
        );
        println!(
            "x={},y={},nx={},ny={}",
            x.unwrap(),
            y.unwrap(),
            nx.unwrap(),
            ny.unwrap()
        );
    }

    #[test]
    fn test_problem() {
        let path = "../test.txt";
        let contents = read_data(path);
        let (x, y, (nx, ny)) = initial_position(&contents);
        problem1(
            &contents,
            &x.unwrap(),
            &y.unwrap(),
            (nx.unwrap(), ny.unwrap()),
        );
    }
}
