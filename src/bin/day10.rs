mod input_processor;

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day10", '\n');
    let values = &values[0..values.len() - 1];
    let mut values : Vec<usize> = values.iter().map(|x| x.parse::<usize>().unwrap()).collect();

    values.push(0); //outlet
    values.sort_unstable();
    let adapter = values.last().unwrap() + 3;
    values.push(adapter);
    
    let mut one = 0;
    let mut three = 0;
    for (i,v) in values.iter().enumerate() {
        if *v == adapter { break;}  
        if values[i+1] == v + 1 {
            one += 1;
        }
        else {
            three +=1;
        }
    }

    println!("Differences multiplied: {}", one * three);

    let mut cache : Vec<usize> = vec!(0; 94);

    let res = calc_from(&values, 0, &mut cache);

    println!("Total possible combinations: {}", res);
}

fn calc_from(values: &Vec<usize>, i: usize, mut cache: &mut Vec<usize>) -> usize {
    if i == values.len() - 1 {
        return 1;
    }
    else {
        let mut size = 0;
        let v = values[i];
        if i + 1 < values.len() && (values[i+1] == v + 1 || values[i+1] == v + 2 || values[i+1] == v + 3) {
            if cache[i+1] != 0 {
                size += cache[i+1];
            }
            else {
                let val = calc_from(&values, i+1, cache);
                cache[i+1] = val;
                size += val;
            }
            
        }
        if i + 2 < values.len() && (values[i+2] == v + 1 || values[i+2] == v + 2 || values[i+2] == v + 3) {
            if cache[i+2] != 0 {
                size += cache[i+2];
            }
            else {
                let val = calc_from(&values, i+2, cache);
                cache[i+2] = val;
                size += val;

            }
        }
        if i + 3 < values.len() && (values[i+3] == v + 1 || values[i+3] == v + 2 || values[i+3] == v + 3) {
            if cache[i+3] != 0 {
                size += cache[i+3];
            }
            else {
                let val = calc_from(&values, i+3, cache);
                cache[i+3] = val;
                size += val;
            }
        }

        size
    }
}   