mod input_processor;

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day09", '\n');
    let values = &values[0..values.len() - 1];
    let values : Vec<usize> = values.iter().map(|n| n.parse::<usize>().expect("Not a valid number")).collect();

    let mut numbers : Vec<usize> = Vec::new();

    for i in 0..25 {
        numbers.push(values[i]);
    }

    let mut res = 0;
    for i in 26..values.len() {
        let num = values[i];
        let mut valid = false;
        'second: for n1 in &numbers {
            for n2 in &numbers {
                if num == *n1 + *n2 && *n1 != *n2 {
                    valid = true;
                    break 'second;
                }
            }
        }
        if valid {
            numbers.remove(0);
            numbers.push(num);
        }
        else {
            res = num;
            break;
        }
    }

    println!("The first number not valid is {}", res);

    // 1. All values since the first bigger element won't be addable
    // 2. If the value + its previous > res => won't be addable
    let mut possible : Vec<usize> = Vec::new();
    let mut prev = 0;
    for v in &values {
        if *v > res {
            break;
        }
        else {
            if prev != 0 {
                if prev + *v > res {
                    break;
                }
            }
            possible.push(*v);
            prev = *v;
        }
    }

    let mut upper = 0;
    let mut lower = 0;
    'outer: for i in 0..possible.len() {
        let mut sum = possible[i];
        'inner: for j in (i+1)..possible.len() {
            sum += possible[j];
            if sum > res {
                break 'inner;
            }
            else if sum == res {
                lower = i;
                upper = j + 1;
                break 'outer;
            }
        }
    }

    let seq : Vec<usize> = (&possible[lower..upper]).to_vec();
    let min = seq.iter().min().unwrap();
    let max = seq.iter().max().unwrap();

    println!("The encryption weakness is {}", min+max);    
    
}   