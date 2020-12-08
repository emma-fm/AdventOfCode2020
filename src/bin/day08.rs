mod input_processor;

#[allow(non_snake_case)]
struct Console {
    accumulator: isize,
    instructions: Vec<Instruction>,
    PC: isize //program counter
}

impl Console {
    // Returns true if execution has ended
    fn run(&mut self) -> bool {
        let inst = &self.instructions[self.PC as usize];
        match inst {
            Instruction::acc(v) => {
                self.accumulator += v;
                self.PC += 1;
            },
            Instruction::jmp(v) => {
                self.PC += v;
            },
            Instruction::nop(_) => {
                self.PC += 1;
            }
        }

        self.PC as usize >= self.instructions.len() 
    }

    fn new(instructions: Vec<Instruction>) -> Self {
        Console {accumulator: 0, instructions: instructions, PC: 0}
    }
}

#[derive(Clone)]
#[allow(non_camel_case_types)]
enum Instruction {
    acc(isize),
    jmp(isize),
    nop(isize)
}

impl Instruction {
    fn from(line: &String) -> Self {
        let type_i = &line[0..3];
        let quantity = line[4..].parse::<isize>().expect("Quantity not valid");
        match type_i {
            "acc" => Instruction::acc(quantity),
            "jmp" => Instruction::jmp(quantity),
            "nop" => Instruction::nop(quantity),
            _ => panic!("Wrong instruction!")
        }
    }
}

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day08", '\n');
    let values = &values[0..values.len() - 1];
    let instructions : Vec<Instruction> = values.iter().map(|v| Instruction::from(v)).collect();

    let mut console = Console::new(instructions);
    let mut visited : Vec<isize> = Vec::new(); //Store visitied addresses
    while !visited.contains(&console.PC) {
        visited.push(console.PC);
        console.run();
    }

    println!("Accumulator value when repeating: {}", console.accumulator);

    let instructions : Vec<Instruction> = values.iter().map(|v| Instruction::from(v)).collect();
    let mut res = 0;

    for i in 0..instructions.len() {
        let mut new_instructions = instructions.clone();
        let mut change = false;
        let instruction = &instructions[i];
        match instruction {
            Instruction::jmp(v) => {
                new_instructions[i] = Instruction::nop(*v);
                change = true;
            },
            Instruction::nop(v) => {
                new_instructions[i] = Instruction::jmp(*v);
                change = true;
            },
            _ => {}
        }

        if change {
            let mut console = Console::new(new_instructions.clone());
            let mut visited : Vec<isize> = Vec::new(); //Store visitied addresses
            let mut retval = false;
            while !retval && !visited.contains(&console.PC) {
                visited.push(console.PC);
                retval = console.run();
            }
            
            if retval {
                res = console.accumulator;
                break;
            }
        }
    }

    println!("Accumulator value for correct termination is: {}", res);
}
