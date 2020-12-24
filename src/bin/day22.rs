use std::collections::VecDeque;

mod input_processor;

fn main() {
    let values = input_processor::dump_vec_seq("inputs/day22", "\n\n");
    let mut player_one : Vec<&str> = (&values[0]).split('\n').collect();
    player_one.remove(0);
    let mut player_two : Vec<&str> = (&values[1]).split('\n').collect();
    player_two.remove(0);
    player_two.pop();

    let mut stack_one : VecDeque<usize> = VecDeque::new();
    let mut stack_two : VecDeque<usize> = VecDeque::new();

    for v in player_one {
        stack_one.push_back(v.parse::<usize>().unwrap());
    }

    for v in player_two {
        stack_two.push_back(v.parse::<usize>().unwrap());
    }


    while stack_one.len() > 0 && stack_two.len() > 0 {
        play(&mut stack_one, &mut stack_two);
    }

    let mut score = 0;
    for (i,e) in stack_one.iter().enumerate() {
        score += e * (stack_one.len() - i);
    }
    for (i,e) in stack_two.iter().enumerate() {
        score += e * (stack_two.len() - i);
    }

    println!("{}", score);

}

fn play(player_one: &mut VecDeque<usize>, player_two: &mut VecDeque<usize>) {
    let card_one = player_one.pop_front().unwrap();
    let card_two = player_two.pop_front().unwrap();
    if card_one > card_two {
        player_one.push_back(card_one);
        player_one.push_back(card_two);
    }
    else if card_one < card_two {
        player_two.push_back(card_two);
        player_two.push_back(card_one);
    }
    else {
        player_one.push_back(card_one);
        player_two.push_back(card_two);
    }
}