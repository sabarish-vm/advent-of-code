fn swapper<const N: usize>(characters: &mut [char], sv: &str, check_id: &usize, char_id: &usize) {
    let elem = sv.chars().nth(*char_id).unwrap();
    let mut reset: bool = false;
    for j in *check_id..N {
        let elem2 = characters.get(j).unwrap();
        if elem > *elem2 && !reset {
            characters[j] = elem;
            reset = true;
            continue;
        }
        if reset {
            characters[j] = '0';
        }
    }
}

fn problem<const N: usize>(lines: &str) {
    let lines_split = lines.split('\n');
    let mut res: i64 = 0;
    for line in lines_split {
        let mut characters: [char; N] = ['0'; N];
        let mut it: usize = 0;
        while it < line.len() {
            let rem = line.len() - it;
            let check_from = N.saturating_sub(rem);
            swapper::<N>(&mut characters, line, &check_from, &it);
            it += 1;
        }
        let val = characters
            .iter()
            .map(|e| e.to_digit(10).unwrap())
            .fold(0, |acc: i64, digit| acc * 10 + (digit as i64));
        res += val;
    }
    println!("{res}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = args.get(1).unwrap();
    let contents = std::fs::read_to_string(filepath).unwrap();
    problem::<2>(&contents);
    problem::<12>(&contents);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let data = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        problem::<2>(data);
        problem::<12>(data);
    }
}
