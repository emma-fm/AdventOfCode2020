mod input_processor;

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day05", '\n');
    let values = &values[0..values.len() - 1];

    let ids : Vec<usize> = values.iter()
    .map(|c| get_seat_id(c))
    .collect();

    let max = ids.iter().max().unwrap();
    let min = ids.iter().min().unwrap();

    println!("Max seat ID: {}", max);


    let mut res = 0;
    for i in *min..*max {
        if !ids.contains(&i) && ids.contains(&(i+1)) && ids.contains(&(i+2)) {
            res = i;
            break;
        }
    }
    
    println!("My seat ID: {}", res);
}

fn get_seat_id(code: &String) -> usize {
    let mut bin = String::from("");
    for c in code.chars() {
        let mut ch = ' ';
        match c {
            'B' => {ch = '1'},
            'F' => {ch = '0'},
            'R' => {ch = '1'},
            'L' => {ch = '0'},
            _ => {}
        }
        bin.push(ch);
    }

    usize::from_str_radix(&bin,2).unwrap()
}