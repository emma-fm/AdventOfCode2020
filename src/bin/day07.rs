mod input_processor;

struct TBag {
    color: String,
    contains: Vec<(usize,String)>
}

#[derive(Clone)]
struct Bag {
    color: String,
    contains: Vec<(usize,Bag)>
}


impl Bag {
    fn can_contain(&self, bag: &String) -> bool {

        for b in &self.contains {
            if b.1.color == *bag { //Color matches
                return true;
            }
        }

        for b in &self.contains {
            if b.1.can_contain(bag) { //Recursive search
                return true;
            }
        }

        false
    }

    fn bags_required_inside(&self) -> usize {
        let mut count = 0;
        for b in &self.contains {
            count += b.0 + b.0*b.1.bags_required_inside();
        }

        count
    }
}

impl PartialEq for Bag {
    fn eq(&self, other:&Self) -> bool {
        self.color == other.color
    }
}

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day07", '\n');
    let values = &values[0..values.len() - 1];

    let mut tbags : Vec<TBag> = Vec::new();

    for line in values {
        let words : Vec<&str> = line.split(' ').collect();
        let mut color = String::from(words[0]);
        color.push_str(" ");
        color.push_str(words[1]);

        let mut bag = TBag {color: color, contains: Vec::new()};
        
        let mut isbag = false;
        let mut color = String::from("");
        let mut num = 0;
        for w in words.iter() {
            let chars : Vec<char>= w.chars().collect();

            if chars[0].is_numeric() {
                isbag = true;
                num = w.parse::<usize>().unwrap();
            }
            else if w.contains(',') || w.contains('.') {
                isbag = false;
                if color != String::from("") {bag.contains.push((num,color));}
                color = String::from("");
                num = 0;
            }
            else if isbag {
                if color != String::from("") { color.push_str(" ");}
                color.push_str(w);
            }
        }
        tbags.push(bag);
    }

    tbags.sort_by(|a, b| a.contains.len().partial_cmp(&b.contains.len()).unwrap());

    let mut bags : Vec<Bag> = Vec::new();
    let mut queue : Vec<&TBag> = Vec::new();

    for tb in tbags.iter() {
        let len = tb.contains.len();
        let mut recursivebags : Vec<(usize,Bag)> = Vec::new();
        if len > 0 {
            for b in tb.contains.iter() {
                match bags.iter().position(|x| x.color == *b.1) {
                    Some(i) => {recursivebags.push((b.0, bags[i].clone()));}
                    None => {}
                };
            }
        }
        if recursivebags.len() == len {
            bags.push(Bag {color: tb.color.clone(), contains: recursivebags});
        }
        else {
            queue.push(tb);
        }
    }

    while bags.len() < tbags.len() {
        for tb in queue.iter() {
            if !bags.contains(&(Bag {color: tb.color.clone(), contains: Vec::new()})) {
                let len = tb.contains.len();
                let mut recursivebags : Vec<(usize,Bag)> = Vec::new();
                if len > 0 {
                    for b in tb.contains.iter() {
                        match bags.iter().position(|x| x.color == *b.1) {
                            Some(i) => {recursivebags.push((b.0, bags[i].clone()));}
                            None => {}
                        };
                    }
                }
                if recursivebags.len() == len {
                    bags.push(Bag {color: tb.color.clone(), contains: recursivebags});
                }
            }
        }
    }

    let mut count = 0;
    for b in &bags {
        if b.can_contain(&String::from("shiny gold")) {count += 1;}
    }

    println!("Shiny gold can be contained in {} bags", count);

    let mut count = 0;
    match &bags.iter().position(|x| x.color == "shiny gold") {
        Some(i) => {
            count += bags[*i].bags_required_inside();
        }
        _ => {panic!("Bag not found");}
    }

    println!("Shiny gold must have {} bags inside", count);

}