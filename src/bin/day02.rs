mod input_processor;

struct Password {
    policy_min: usize,
    policy_max: usize,
    policy_char: char,
    password: String
}

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day02", '\n');
    let values = &values[0..values.len() - 1];
    let mut passwords = Vec::new();

    for v in values {
        let spl: Vec<&str> = v.split(|c: char| c == ' ' || c == ':' || c == '-').collect();
        passwords.push(
            Password {
                policy_min: spl[0].parse::<usize>().unwrap(), 
                policy_max: spl[1].parse::<usize>().unwrap(), 
                policy_char: spl[2].parse::<char>().unwrap(),
                password: String::from(spl[4]) //spl[3] is an empty character
                })
    }


    let res: Vec<&Password> = passwords.iter().filter(|p| check(p)).collect();
    println!("Correct passwords: {}", res.len());

    let res: Vec<&Password> = passwords.iter().filter(|p| check_two(p)).collect();
    println!("Correct passwords (second pattern): {}", res.len());
}

fn check(p: &Password) -> bool {
    let matches: Vec<&str> = p.password.matches(p.policy_char).collect();
    matches.len() >= p.policy_min && matches.len() <= p.policy_max
}

fn check_two(p: &Password) -> bool {
    let seq = p.password.as_bytes();
    (seq[p.policy_min-1] as char == p.policy_char && seq[p.policy_max-1] as char != p.policy_char) ||
    (seq[p.policy_min-1] as char != p.policy_char && seq[p.policy_max-1] as char == p.policy_char)
}