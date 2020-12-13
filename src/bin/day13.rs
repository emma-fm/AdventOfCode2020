mod input_processor;

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day13", '\n');
    let departure = values[0].parse::<usize>().unwrap();

    let buses : Vec<usize>= (&values[1]).split(',').filter(|c| *c != "x").map(|c| c.parse::<usize>()).map(|c| c.unwrap()).collect();
    let mut closest : Vec<(usize,usize)> = Vec::new();
    
    for b in buses {
        let div = (departure as f64 / b as f64).ceil() as usize;
        closest.push((b,b * div));
    }

    let mut min = closest[0];
    for c in closest {
        if c.1 < min.1 {
            min = c;
        }
    }

    println!("ID of the earliest * waiting minutes: {}", min.0 * (min.1 - departure));
}