mod input_processor;

use std::collections::HashMap;

enum Instruction {
    Mask(String),
    Memory(usize, usize)
}

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day14", '\n');
    let values = &values[0..values.len() - 1];

    let mut instructions : Vec<Instruction> = Vec::new();

    for line in values {
        if line.contains("mask") {
            let mask = String::from(&line[7..]);
            instructions.push(Instruction::Mask(mask));
        }
        else {
            let spl: Vec<&str> = line.split(|c: char| c == '[' || c == ']' || c == '=').collect();
            let direction = String::from(spl[1]).parse::<usize>().unwrap();
            let number = String::from(spl[3].replace(" ", "")).parse::<usize>().unwrap();
            instructions.push(Instruction::Memory(direction, number));
        }
    }
    let mut memory : HashMap<usize,usize> = HashMap::new();
    let mut bitmask = String::from("");
    for i in &instructions {
        match i {
            Instruction::Mask(m) => {
                bitmask = m.clone();
            },
            Instruction::Memory(dir,num) => {
                memory.insert(*dir, mask(*num, &bitmask));
            }
        }
    }

    let res = memory.iter().map(|(_,val)| val).sum::<usize>();

    println!("Sum of all values in memory: {}", res);

    let mut memory : HashMap<usize,usize> = HashMap::new();
    let mut bitmask = String::from("");
    for i in instructions {
        match i {
            Instruction::Mask(m) => {
                bitmask = m;
            },
            Instruction::Memory(dir,num) => {
                let addresses = memory_mask(dir, &bitmask);
                for a in addresses {
                    memory.insert(a, num);
                }
            }
        }
    }

    let res = memory.iter().map(|(_,val)| val).sum::<usize>();

    println!("Sum of all values in memory with version 2: {}", res);
}

// Calculate the resulting number for using a bitmask in a number
// With part 1 rules
fn mask(number: usize, bitmask: &String) -> usize {
    let mut bits = format!("{:036b}", number);
    for (i,c) in bitmask.chars().enumerate() {
        match c {
            '1' => {
                bits.replace_range(i..(i+1), "1");
            },
            '0' => {
                bits.replace_range(i..(i+1), "0");
            },
            _ => {}
        }
    }

    usize::from_str_radix(bits.as_str(), 2).unwrap()
}

// Calculates all possible memory addresses for using a bitmask in an address
// With part 2 rules
fn memory_mask(address: usize, bitmask: &String) -> Vec<usize> {
    let mut bits = format!("{:036b}", address);
    for (i,c) in bitmask.chars().enumerate() {
        match c {
            '1' => {
                bits.replace_range(i..(i+1), "1");
            },
            'X' => {
                bits.replace_range(i..(i+1), "X");
            },
            _ => {}
        }
    }

    let mut vec : Vec<usize> = Vec::new();
    calc_address(bitmask, bits, &mut vec);

    vec
}

// Calculates all addresses for an address with floating values
// Returning them in the vec parameter.
// The calculation is done recursively, with each floating value
// making two recursive calls, one with 1 instead and another with 0
// Once the address has no floating values, it is pushed into 'vec'
fn calc_address(bitmask: &String, addr: String, vec: &mut Vec<usize>) {
    if !addr.contains("X") {
        vec.push(usize::from_str_radix(addr.as_str(), 2).unwrap());
    }
    else {
        let mut iter = addr.char_indices();
        let (mut i, mut c) = iter.next().unwrap();
        while c != 'X' {
            let t = iter.next().unwrap();
            i = t.0;
            c = t.1;
        }
        let mut zero = addr.clone();
        let mut one = addr.clone();
        zero.replace_range(i..(i+1), "0"); //addr with 0 instead of X
        one.replace_range(i..(i+1), "1"); //addr with 1 instead of X

        calc_address(bitmask, zero, vec);
        calc_address(bitmask, one, vec);
    }
}