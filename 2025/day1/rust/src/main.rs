use std::ops::Add;

struct MyType {
    // Current state of the needle
    state: i64,
    m_0end: u64,
    m_0passed: u64,
}
impl Add<i64> for MyType {
    type Output = MyType;

    fn add(mut self, rhs: i64) -> MyType {
        let absrhs = rhs.unsigned_abs();
        self.m_0passed += if absrhs >= 100 { absrhs / 100 } else { 0 };
        let rem: i64 = rhs % 100;
        let temp = self.state + rem;
        if temp >= 100 {
            let new_state = temp - 100;
            if self.state != 0 && new_state == 0 {
                self.m_0end += 1;
            } else if self.state != 0 && new_state != 0 {
                self.m_0passed += 1;
            }
            self.state = new_state;
        } else if temp < 0 {
            let new_state = temp + 100;
            if self.state != 0 && new_state == 0 {
                self.m_0end += 1;
            } else if self.state != 0 && new_state != 0 {
                self.m_0passed += 1;
            }
            self.state = new_state;
        } else if temp == 0 {
            self.state = temp;
            self.m_0end += 1;
        } else {
            self.state = temp;
        }
        self
    }
}

fn main() {
    let mut start = MyType {
        state: 50,
        m_0end: 0,
        m_0passed: 0,
    };

    let args: Vec<String> = std::env::args().collect();
    let filepath = args.get(1).unwrap();
    let contents = match std::fs::read_to_string(filepath) {
        Ok(x) => x,
        Err(x) => panic!("Error reading file {x}"),
    };
    for x in contents.lines() {
        if let Some(num) = x.strip_prefix("L") {
            let val: i64 = -(num.parse::<i64>().unwrap());
            start = start + val;
        } else if let Some(num) = x.strip_prefix("R") {
            let val: i64 = num.parse::<i64>().unwrap();
            start = start + val;
        }
    }
    println!("{}, {}, {}", start.state, start.m_0end, start.m_0passed);
}
