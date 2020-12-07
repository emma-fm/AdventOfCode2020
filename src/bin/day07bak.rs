mod input_processor;

struct TBag {
    color: String,
    contains: Vec<String>
}

#[derive(Clone,Debug)]
struct Bag {
    color: String,
    contains: Vec<Bag>
}

impl Bag {
    fn can_contain(&self, bag: &String) -> bool {

        for b in &self.contains {
            if b.color == *bag { //Color matches
                return true;
            }
        }

        for b in &self.contains {
            if b.can_contain(bag) { //Recursive search
                return true;
            }
        }

        false
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
        for w in words.iter() {
            let chars : Vec<char>= w.chars().collect();
            if chars[0].is_numeric() {
                isbag = true;
            }
            else if w.contains(',') || w.contains('.') {
                isbag = false;
                if color != String::from("") {bag.contains.push(color);}
                color = String::from("");
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
        let mut recursivebags : Vec<Bag> = Vec::new();
        if len > 0 {
            for b in tb.contains.iter() {
                match bags.iter().position(|x| x.color == *b) {
                    Some(i) => {recursivebags.push(bags[i].clone());}
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
                let mut recursivebags : Vec<Bag> = Vec::new();
                if len > 0 {
                    for b in tb.contains.iter() {
                        match bags.iter().position(|x| x.color == *b) {
                            Some(i) => {recursivebags.push(bags[i].clone());}
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
    for b in bags {
        if b.can_contain(&String::from("shiny gold")) {count += 1;}
    }

    println!("Shiny gold can be contained in {} bags", count);
}