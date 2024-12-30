use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let contents: String = read_to_string("../input.txt").unwrap().replace("\n", "");
    // Part 1 of the problem
    let renum: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sol1: i32 = 0;
    renum.captures_iter(&contents).for_each(|cap| {
        let (_, [n1, n2]): (_, [&str; 2]) = cap.extract();
        sol1 += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
    });
    println!("Part1 = {}", sol1);

    // Part 2 of the problem
    //
    // Part 2.1 : Extract all mul() before the first don't
    let mut sol21: i32 = 0;
    let re21: Regex = Regex::new(r"^.*?don").unwrap();
    let line21: &str = re21.find(&contents).unwrap().as_str();
    renum.captures_iter(line21).for_each(|cap| {
        let (_, [n1, n2]): (_, [&str; 2]) = cap.extract();
        sol21 += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
    });

    // Part 2.2 : Extract all mul() after the first do after all the first don't(s)
    let mut sol22: i32 = 0;

    // Extract string containing everythin after the first don
    let re22: Regex = Regex::new(r".*?don(.*)").unwrap();
    let (_, [line22]): (_, [&str; 1]) = re22.captures(&contents).unwrap().extract();

    // Capture all strings inbetween do()....don. This removes the strings that
    // lie inbetween don't and do and iterate over them.
    let re221: Regex = Regex::new(r"do\(\).*?don").unwrap();

    re221.find_iter(line22).for_each(|cap| {
        renum.captures_iter(cap.as_str()).for_each(|subcap| {
            let (_, [n1, n2]): (_, [&str; 2]) = subcap.extract();
            sol22 += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
        });
    });
    println!(
        "Part2 :\n\tTotal = {},\n\tPart2.1 = {},\n\tPart2.2 = {}",
        sol21 + sol22,
        sol21,
        sol22
    );
}
