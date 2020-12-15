mod input_processor;

#[derive(Debug)]
struct Number {
    number: usize,
    rounds: Vec<usize>
}

impl Number {
    fn new(num: usize, round: usize) -> Self {
        let mut n = Number {number: num, rounds: Vec::new()};
        n.rounds.push(round);

        n
    }
}

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day15", ',');
    let values : Vec<usize> = values.iter().map(|v| v.parse::<usize>().unwrap()).collect();

    let mut numbers : Vec<Number> = Vec::new();
    for (i,v) in values.iter().enumerate() {
        numbers.push(Number::new(*v, i+1));
    }

    let mut speak = *(values.last().unwrap());
    for i in values.len() + 1 .. 2021 { 
        let num = numbers.iter().find(|v| (**v).number == speak).unwrap();
        if num.rounds.len() == 1 { //First time spoken
            speak = 0;
        }
        else {
            speak = *num.rounds.last().unwrap() - num.rounds[num.rounds.len() - 2];
        }

        let f = numbers.iter_mut().find(|v| (**v).number == speak);

        match f {
            Some(item) => {
                item.rounds.push(i);
            }
            None => {
                numbers.push(Number::new(speak, i));
            }
        }
        
    }
    
    println!("2020th number: {}", speak);
}