use regex::Regex;

mod input_processor;

struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt_value: usize,
    hgt_unit: String,
    hcl: String,
    ecl: String,
    pid: String,
}

fn main() {
    let values = input_processor::dump_vec_seq("inputs/day04", "\n\n");

    // Take values not empty & Remove values without appropiate fields
    let values : Vec<&String> = values.iter()
    .filter(|s| **s != String::from(""))
    .filter(|s| check_valid(*s))
    .collect(); 

    println!("Valid passports: {}", values.len());

    let values : Vec<Passport> = values.iter().map(|s| parse_passport(s)).filter(|s| full_check(s)).collect();

    println!("Valid passports with field check: {}", values.len());
}

fn check_valid(p: &String) -> bool {
    let passport = &*p;
    passport.contains("byr:") && passport.contains("iyr:") && passport.contains("eyr:")
    && passport.contains("hgt:") && passport.contains("hcl:") && passport.contains("ecl:")
    && passport.contains("pid:") 
}

// Only for passports with all required fields
fn parse_passport(p: &String) -> Passport {
    let spl : Vec<&str> = p.split(|c| c == ' '|| c == '\n').collect();
    let mut res = Passport { byr: 0, iyr: 0, eyr: 0, hgt_value: 0, hgt_unit: String::from(""), hcl: String::from(""), ecl: String::from(""), pid: String::from("")};
    for s in spl {
        if s.contains("byr:") {
            res.byr = s[4..s.len()].parse::<usize>().expect("unvalid byr");
        }
        else if s.contains("iyr:") {
            res.iyr = s[4..s.len()].parse::<usize>().expect("unvalid iyr");
        }
        else if s.contains("eyr:") {
            res.eyr = s[4..s.len()].parse::<usize>().expect("unvalid eyr");
        }
        else if s.contains("hgt:") {
            if s.contains("cm") {
                res.hgt_unit = String::from("cm");
                match s[4..7].parse::<usize>() {
                    Ok(v) => {res.hgt_value = v;},
                    Err(_) => {res.hgt_value = 0;}
                }
            }
            else if s.contains("in") {
                res.hgt_unit = String::from("in");
                match s[4..6].parse::<usize>() {
                    Ok(v) => {res.hgt_value = v;},
                    Err(_) => {res.hgt_value = 0;}
                }   
            }
        }
        else if s.contains("hcl:") {
            res.hcl = s[4..s.len()].into();
        }
        else if s.contains("ecl:") {
            res.ecl = s[4..s.len()].into();
        }
        else if s.contains("pid:") {
            res.pid = s[4..s.len()].into();
        }
    }

    res
}

fn full_check(p: &Passport) -> bool {
    let regex_hcl = Regex::new(r"#[0-9a-f]{6}").unwrap();
    let regex_pid = Regex::new(r"[0-9]{9}").unwrap();
    (p.byr >= 1920 && p.byr <= 2002) &&
    (p.iyr >= 2010 && p.iyr <= 2020) &&
    (p.eyr >= 2020 && p.eyr <= 2030) &&

    (
        (p.hgt_unit == String::from("cm") && p.hgt_value >= 150 && p.hgt_value <= 193) ||
        (p.hgt_unit == String::from("in") && p.hgt_value >= 59 && p.hgt_value <= 76)
    ) &&
    (regex_hcl.is_match(&p.hcl)) &&
    (
        p.ecl == String::from("amb") || p.ecl == String::from("blu") || p.ecl == String::from("brn") ||
        p.ecl == String::from("gry") || p.ecl == String::from("grn") || p.ecl == String::from("hzl") ||
        p.ecl == String::from("oth")
    ) &&
    ((&p).pid.len() == 9 && regex_pid.is_match(&p.pid))
}
