use core::panic;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
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
    let mut loops = HashSet::new();
    while idex {
        if data[x][y] == '#' {
            x = (x as i32 - nx) as usize;
            y = (y as i32 - ny) as usize;
            rotate(&mut nx, &mut ny);
        } else {
            loops.insert((x, y));
        }
        x = (x as i32 + nx) as usize;
        y = (y as i32 + ny) as usize;
        idex = index_exists(data, &x, &y);
    }
    loops
}

fn problem2_block(data: &[Vec<char>], rowid: &usize, colid: &usize, direction: (i32, i32)) -> i32 {
    let mut x = *rowid;
    let mut y = *colid;
    let mut nx;
    let mut ny;
    let mut idex = index_exists(data, &x, &y);
    (nx, ny) = direction;
    let mut visited: HashSet<(usize, usize, i32, i32)> = HashSet::new();
    let mut ret = 0;
    let mut already;
    while idex {
        if data[x][y] == '#' {
            x = (x as i32 - nx) as usize;
            y = (y as i32 - ny) as usize;
            already = !visited.insert((x, y, nx, ny));
            if already {
                ret += 1;
                break;
            }
            rotate(&mut nx, &mut ny);
        }
        x = (x as i32 + nx) as usize;
        y = (y as i32 + ny) as usize;
        idex = index_exists(data, &x, &y);
    }
    ret
}

fn problem2(
    places: &HashSet<(usize, usize)>,
    data: &mut [Vec<char>],
    rowid: &usize,
    colid: &usize,
    direction: (i32, i32),
) -> i32 {
    let xini = *rowid;
    let yini = *colid;
    let total = Arc::new(Mutex::new(0));
    places.par_iter().for_each(|(px, py)| {
        let mut datacp = data.to_owned();
        if (xini, yini) != (*px, *py) {
            let charcpy = datacp[*px][*py];
            datacp[*px][*py] = '#';
            let loops = problem2_block(&datacp, rowid, colid, direction);
            let mut totmutex = total.lock().unwrap();
            *totmutex += loops;
            datacp[*px][*py] = charcpy;
        }
    });

    // for (px, py) in places.iter() {
    //     if (xini, yini) != (*px, *py) {
    //         let charcpy = data[*px][*py];
    //         data[*px][*py] = '#';
    //         let loops = problem2_block(data, rowid, colid, direction);
    //         total += loops;
    //         data[*px][*py] = charcpy;
    //     }
    // }
    let value = *total.lock().unwrap();
    value
}

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    let path = "../input.txt";
    let mut contents = read_data(path);
    if let (Some(x), Some(y), (Some(nx), Some(ny))) = initial_position(&contents) {
        let places = problem1(&contents, &x, &y, (nx, ny));
        println!("No. of places = {}", places.len());
        let loops = problem2(&places, &mut contents, &x, &y, (nx, ny));
        println!("No. of loops = {}", loops)
    }
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
    fn test_problem_1() {
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
    #[test]
    fn test_loop_count() {
        let path = "../test.txt";
        let path2 = "../test_1.txt";
        let contents = read_data(path);
        let contents2 = read_data(path2);
        let (x, y, (nx, ny)) = initial_position(&contents);
        let places = problem2_block(
            &contents2,
            &x.unwrap(),
            &y.unwrap(),
            (nx.unwrap(), ny.unwrap()),
        );
        assert_eq!(1, places);
    }
    #[test]
    fn test_problem_2() {
        let path = "../test.txt";
        let mut contents = read_data(path);
        let (x, y, (nx, ny)) = initial_position(&contents);
        let places = problem1(
            &contents,
            &x.unwrap(),
            &y.unwrap(),
            (nx.unwrap(), ny.unwrap()),
        );
        let tot = problem2(
            &places,
            &mut contents,
            &x.unwrap(),
            &y.unwrap(),
            (nx.unwrap(), ny.unwrap()),
        );
        assert_eq!(tot, 6);
    }
}
