mod input_processor;

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day01", '\n');
    let values = &values[0..values.len() - 1];
    let mut values : Vec<usize> = values.into_iter().map(|n| n.parse::<usize>().unwrap()).collect();
    
    let mut a = 0;
    let mut b = 0;

    'outer: for va in &values {
        for vb in &values {
            if *va != *vb && *va + *vb == 2020 {
                a = *va;
                b = *vb;
                break 'outer;
            } 
        }
    }    

    println!("The product is {}!", a*b);

    // Remove all impossible values (min + 2nd min + value > 2020) -> value will never match

        values.sort_unstable();
        let min = values[0];
        let min2 = values[1];
        let max_possible = 2020 - min - min2;
        
        while *values.last().unwrap() > max_possible {
            values.pop();
        }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    'outer2: for va in &values {
        for vb in &values {
            for vc in &values {
                if *va != *vb && *va != *vc && *vb != *vc && *va + *vb + *vc == 2020 {
                    a = *va;
                    b = *vb;
                    c = *vc;
                    break 'outer2;
                }
            }
        }
    }
    println!("The 2nd product is {}!", a*b*c);
    
}