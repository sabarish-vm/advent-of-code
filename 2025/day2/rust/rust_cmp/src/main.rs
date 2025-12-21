const fn build_table() -> ([[usize; 10]; 15], [usize; 15]) {
    let mut storage = [[0usize; 10]; 15];
    let mut counts = [0usize; 15];

    let mut n = 0;
    while n < 15 {
        if n == 0 {
            storage[n][0] = 0;
            counts[n] = 1;
        } else if n == 1 {
            storage[n][0] = 1;
            counts[n] = 1;
        } else {
            let mut count = 0;
            let mut i = 1;
            while i < n {
                if n % i == 0 {
                    storage[n][count] = i;
                    count += 1;
                }
                i += 1;
            }
            counts[n] = count;
        }
        n += 1;
    }

    (storage, counts)
}
const FACTORS: ([[usize; 10]; 15], [usize; 15]) = build_table();

fn get_factors(n: usize) -> &'static [usize] {
    let (storage, counts) = &FACTORS;
    &storage[n][0..counts[n]]
}

fn counter(vec: &[i64], count1: &mut i64, count2: &mut i64) {
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
            let factors = get_factors(slen);
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
    for ele in csplit {
        let (v1, v2) = ele
            .split_once('-')
            .map(|(x, y)| (x.trim_end(), y.trim_end()))
            .unwrap();
        let v1 = v1.parse::<i64>().unwrap();
        let v2 = v2.parse::<i64>().unwrap();
        let vec: Vec<i64> = (v1..=v2).collect();
        counter(&vec, &mut c1, &mut c2);
    }
    println!("Count1 = {c1}\nCount2 = {c2}\nCountT = {}", c1 + c2);
}
