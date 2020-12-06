use std::collections::HashSet;

mod input_processor;

fn main() {
    let values = input_processor::dump_vec_seq("inputs/day06", "\n\n");

    let total : usize = values.iter().map(|s| count_group(s)).sum();

    println!("Total yes answers: {}", total);

    let answers : usize = values.iter().map(|s| count_by_person(s)).sum();

    println!("Total common answers: {}", answers);
}

fn count_group(group: &String) -> usize {
    let mut set = HashSet::new();
    let values : Vec<char> = (*group).chars().filter(|c| *c != '\n' && *c != ' ').collect();

    for v in values {
        set.insert(v);
    }

    set.len()
}

fn count_by_person(group: &String) -> usize {
    let mut group_set = HashSet::new();
    let values : Vec<char> = (*group).chars().filter(|c| *c != '\n' && *c != ' ').collect();
    let people = (*group).split('\n');

    for v in values {
        group_set.insert(v);
    }

    for p in people {
        let mut set = HashSet::new();
        for a in p.chars() {
            set.insert(a);
        }
        group_set = group_set.intersection(&set).map(|c| *c).collect();
    }

    group_set.len()
}