use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        let line = line.unwrap();
        let line = line.trim();
        let mut words = line.split(' ');
        let lhs = words.next().unwrap().parse::<isize>().unwrap();
        let operator = words.next().unwrap();
        let rhs = words.next().unwrap().parse::<isize>().unwrap();

        let result = match operator {
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            _ => panic!("Unknown operator: {}", operator),
        };
        println!("result: {}", result);
    }
}

// O 12 * 23
// X (12 * 23) + (12 * 23)
