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
}