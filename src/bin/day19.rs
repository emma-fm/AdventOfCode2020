use regex::Regex;
use std::collections::HashMap;

mod input_processor;

fn main() {
    let values = input_processor::dump_vec_seq("inputs/day19", "\n\n");
    let values = &values[0..2];

    let rules_lines : Vec<&str> = (&values[0]).split('\n').collect();
    let input : Vec<&str> = (&values[1]).split('\n').collect();

    let mut rules : HashMap<usize,String> = HashMap::new();

    for line in rules_lines {
        let spl : Vec<&str> = line.split(':').collect();
        let n = spl[0].parse::<usize>().unwrap();
        rules.insert(n, spl[1].replace('"', "").to_string());
    }


    let mut rule = find_expression(&rules, 0);
    rule.insert(0, '^');
    rule.push_str("$");

    let mut count = 0;

    let rule = Regex::new(&rule).unwrap();
    for i in input {
        if rule.is_match(i) {
            count += 1;
        }
    }

    println!("Number of strings that match rule number 0: {}", count);
}

fn find_expression(rules: &HashMap<usize,String>, num: usize) -> String{
    //Arguments: map with rules, rule number
    //Base case: it's a or b
    //General case: traverse all value split by ' '
    //If the character is a number -> add '(', call recursion with this value and at return add ')'
    //If the character is | -> add |
    //Any other character (white spaces), gets ignored
    //At the end, we return the resulting expression
    //Note that this algorithm will result in a regex with plenty of unnecessary parentheses,
    //such as (a) or (b)
    
    let expr = rules.get(&num).unwrap();
    let mut res = String::from("");

    if expr.contains("a") {
        res = String::from("a");
    }
    else if expr.contains("b") {
        res = String::from("b");
    }
    else {
        let spl : Vec<&str> = expr.split(' ').collect();
        for value in spl {
            if value.parse::<usize>().is_ok() { //Is number
                res.push_str("(");
                res.push_str(&find_expression(rules, value.parse::<usize>().unwrap()));
                res.push_str(")");
            }
            else if value == "|" { //Is OR (|)
                res.push_str("|");
            }
        }
    }   

    res
} 