use std::collections::HashMap;

use num::Integer;

fn gen_factors() -> HashMap<usize, std::vec::Vec<usize>> {
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 2..15 {
        map.insert(i, get_factors(i));
    }
    map
}

fn get_factors<T: Integer + Copy + std::fmt::Display>(a: T) -> Vec<T> {
    let mut start = T::one() + T::one();
    let mut vec = vec![T::one()];
    let limit = a / start + T::one();

    while start < limit {
        if a % start == T::zero() {
            vec.push(start);
        }
        start = start + T::one();
    }
    vec
}

fn counter(vec: &[i64], count1: &mut i64, count2: &mut i64, map: &HashMap<usize, Vec<usize>>) {
    let mut current_num = 0;
    for num in vec {
        let s1 = num.to_string();
        let slen = s1.len();
        if slen > 1 {
            let half_len = slen / 2;
            let s11 = &s1[0..half_len];
            let s12 = &s1[half_len..];
            if s11 == s12 {
                *count1 += num;
                current_num = *num;
                continue;
            }
            let factors = map.get(&slen).unwrap();
            for &factor in factors {
                let prev = &s1[..factor];
                let mut i = 1;
                let mut break_condition = false;
                while i * factor < slen && !break_condition {
                    let current_str = &s1[(i * factor)..((i + 1) * factor)];
                    if current_str != prev {
                        break_condition = true;
                    }
                    i += 1;
                }
                if !break_condition && num != &current_num {
                    *count2 += num;
                    current_num = *num;
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut c1 = 0;
    let mut c2 = 0;
    let filepath = args.get(1).unwrap();
    let contents = std::fs::read_to_string(filepath).unwrap();
    let csplit = contents.split(',');
    let facmap = gen_factors();
    for ele in csplit {
        let (v1, v2) = ele
            .split_once('-')
            .map(|(x, y)| (x.trim_end(), y.trim_end()))
            .unwrap();
        let v1 = v1.parse::<i64>().unwrap();
        let v2 = v2.parse::<i64>().unwrap();
        let vec: Vec<i64> = (v1..=v2).collect();
        counter(&vec, &mut c1, &mut c2, &facmap);
    }
    println!("Count1 = {c1}\nCount2 = {c2}\nCountT = {}", c1 + c2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factors() {
        let a = 9;
        let b = get_factors(a);
        assert_eq!(b, vec![1i64, 3i64]);
        let a: usize = 9;
        let b = get_factors(a);
        assert_eq!(b, vec![1usize, 3usize]);
    }

    #[test]
    fn test_counter_simple() {
        let facmap = gen_factors();
        let a = vec![1234, 1212];
        let mut c1 = 0;
        let mut c2 = 0;
        counter(&a, &mut c1, &mut c2, &facmap);
        assert_eq!(c1, 1212);
    }
    #[test]
    fn test_p1() {
        let facmap = gen_factors();
        let mut c1 = 0;
        let mut c2 = 0;
        let contents = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let csplit = contents.split(',');
        for ele in csplit {
            let (v1, v2) = ele
                .split_once('-')
                .map(|(x, y)| (x.trim_end(), y.trim_end()))
                .unwrap();
            let v1 = v1.parse::<i64>().unwrap();
            let v2 = v2.parse::<i64>().unwrap();
            let vec: Vec<i64> = (v1..=v2).collect();
            counter(&vec, &mut c1, &mut c2, &facmap);
        }
        assert_eq!(c1, 1227775554);
        assert_eq!(c2 + c1, 4174379265)
    }
}
