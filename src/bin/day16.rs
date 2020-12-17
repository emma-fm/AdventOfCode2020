use std::collections::HashSet;
use std::ops::Range;

mod input_processor;

#[derive(Clone)]
struct Ticket {
    numbers: Vec<usize>
}

impl Ticket {
    fn from(values: Vec<&str>) -> Self {
        let mut v = Vec::new();
        values.iter().map(|n| v.push(n.parse::<usize>().unwrap())).for_each(drop);

        Ticket {numbers: v}
    }
}

#[derive(Clone)]
struct Field {
    name: String,
    ranges: (Range<usize>, Range<usize>)
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn main() {
    // PARSING

    let values = input_processor::dump_vec_seq("inputs/day16", "\n\n");
    let fields = values[0].split('\n').collect::<Vec<&str>>();
    let myticket = values[1].split('\n').collect::<Vec<&str>>()[1].split(',').collect::<Vec<&str>>();
    let nearby_tickets = values[2].split('\n').collect::<Vec<&str>>();
    let nearby_tickets = &nearby_tickets[1..nearby_tickets.len() - 1];
    let nearby_tickets = nearby_tickets.iter().map(|n| n.split(',').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let mut tickets : Vec<Ticket> = Vec::new();
    nearby_tickets.iter().map(|n| tickets.push(Ticket::from(n.to_vec()))).for_each(drop);

    let mut validnumbers : HashSet<usize> = HashSet::new();
    for f in &fields {
        let spl = f.split(' ').collect::<Vec<&str>>();
        for s in spl {
            if s.contains("-") {
                let n = s.split('-').collect::<Vec<&str>>();
                let n1 = n[0].parse::<usize>().unwrap();
                let n2 = n[1].parse::<usize>().unwrap();
                for v in n1..(n2+1) {
                    validnumbers.insert(v);
                }
            }
        }
    }

    let mut ticket_fields : Vec<Field> = Vec::new();
    for f in fields {
        let spl = f.split(':').collect::<Vec<&str>>();
        let name = String::from(spl[0]);

        let spl = f.split(' ').collect::<Vec<&str>>();
        let mut ranges : Vec<Range<usize>> = Vec::new();
        for s in spl {
            if s.contains("-") {
                let n = s.split('-').collect::<Vec<&str>>();
                let n1 = n[0].parse::<usize>().unwrap();
                let n2 = n[1].parse::<usize>().unwrap();
                ranges.push(n1..(n2+1));
            }
        }
        ticket_fields.push(Field {name: name, ranges: (ranges[0].clone(), ranges[1].clone())});
    }

    // END PARSING

    // Part 1, check for every ticket if their values are in our ranges set
    let mut error = 0;
    let mut valid_tickets : Vec<Ticket> = Vec::new();
    for t in &tickets {
        let mut ok = true;
        for v in &t.numbers {
            if !validnumbers.contains(&v) {
                error += v;
                ok = false;
            }
        }
        if ok {
            valid_tickets.push((*t).clone());
        }
    }

    println!("Ticket scanning error rate: {}", error);


    // Part 2, while there's fields not assigned, check all
    // not assigned columns and fields, find matches.
    // If there's only 1 match -> that's the right column
    // Else, keep trying
    let mut fieldsorder : Vec<(usize,&Field)> = Vec::new(); 
    let mut assigned : Vec<usize> = Vec::new();
    let mut assigned_fields : Vec<&Field> = Vec::new();

    while fieldsorder.len() < ticket_fields.len() {
        for i in 0..valid_tickets[0].numbers.len() {
            if !assigned.contains(&i) {
                let mut matches : Vec<&Field> = Vec::new();
                for f in &ticket_fields {
                    if !assigned_fields.contains(&&f) {
                        let mut err = false;
                        for t in &valid_tickets {
                            if !f.ranges.0.contains(&t.numbers[i]) && !f.ranges.1.contains(&t.numbers[i]) {
                                err = true;
                                break;
                            }
                        }

                        if !err {
                            // If here, successful
                            matches.push(&f);
                        }

                    }
                }

                if matches.len() == 1 {
                    // Found
                    assigned.push(i);
                    assigned_fields.push(matches[0]);
                    fieldsorder.push((i, matches[0]));
                }
            }
        }
    }

    fieldsorder = fieldsorder.iter().filter(|t| t.1.name.contains("departure")).map(|t| (*t).clone()).collect();

    let mut res = 1;
    let myticket = Ticket::from(myticket);
    for f in fieldsorder {
        res *= myticket.numbers[f.0];
    }

    println!("Multiplying your ticket's departure fields: {}", res);
}