use std::collections::VecDeque;

mod input_processor;


fn main() {
    let values = input_processor::dump_vec_separator("inputs/day18", '\n');
    let values = &values[0..values.len()-1];
    
    let res : usize = values.iter().map(|s| parse_expression(s,1)).sum();

    println!("Total sum with equal precedence: {}", res);

    let res : usize = values.iter().map(|s| parse_expression(s,2)).sum();

    println!("Total sum with + having more precedence: {}", res);
}

// Parse expression to Reverse Polish Notation with Shunting-yard algorithm
// Part indicates part 1 or part 2 rules
fn parse_expression(parse: &String, part: usize) -> usize {
    let mut output_queue : VecDeque<char> = VecDeque::new();
    let mut operator_stack : Vec<char> = Vec::new();
    for token in parse.chars() {
        if token.is_numeric() {
            output_queue.push_back(token);
        }

        else if token == '+' || token == '*' {
            while operator_stack.len() > 0 
            && *(operator_stack.last().unwrap()) != '(' 
            && get_precedence(*operator_stack.last().unwrap(), part) >= get_precedence(token, part){
                output_queue.push_back(operator_stack.pop().unwrap());
            }
            operator_stack.push(token);
        }

        else if token == '(' {
            operator_stack.push(token);
        }

        else if token == ')' {
            while *(operator_stack.last().unwrap()) != '(' { // If panics, there's mismatched parentheses
                output_queue.push_back(operator_stack.pop().unwrap());
            }
            if *(operator_stack.last().unwrap()) == '(' {
                operator_stack.pop().unwrap();
            }
        }
    }

    while operator_stack.len() > 0 {
        output_queue.push_back(operator_stack.pop().unwrap());
    }

    parse_rpn(output_queue)
}

// Get number from Reverse Polish Notation
fn parse_rpn(mut queue: VecDeque<char>) -> usize {
    let mut stack : Vec<usize> = Vec::new();
    while queue.len() > 0 {
        if queue.front().unwrap().is_numeric() {
            stack.push(queue.front().unwrap().to_digit(10).unwrap() as usize);
        }

        else {
            if stack.len() < 2 {
                panic!("Parsing error, wrong number of arguments at RPN");
            }
            else {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let mut f = 0;
                match queue.front().unwrap() {
                    '+' => {f = a+b;},
                    '*' => {f = a*b;},
                    _ => {panic!("Unexpected operator at RPN.");}
                }
                stack.push(f);
            }
        }

        queue.pop_front();
    }

    if stack.len() != 1 {
        panic!("Wrong number of operators at RPN.");
    }

    stack.pop().unwrap()
}

// Get operator precedence, with part 1 or part 2 rules
fn get_precedence(operator: char, part: usize) -> usize{
    if part == 1 {
        return 0;
    }

    else {
        match operator {
            '+' => {
                return 1;
            },
            '*' => {
                return 0;
            }
            _ => {
                panic!("Unexpected character");
            }
        }
    }
}