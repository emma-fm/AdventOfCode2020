use crate::Movement::*;
mod input_processor;

struct Ship {
    x: isize,
    y: isize,
    direction: isize,
    instructions: Vec<Movement>
}

struct Waypoint {
    x: isize,
    y: isize
}

#[derive(Clone)]
enum Movement {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    TurnLeft(isize),
    TurnRight(isize),
    Forward(isize)
}

impl Ship {
    // Move ship 1 time
    fn movement(&mut self) {
        let dir = self.instructions.remove(0);
        match dir {
            North(v) => {
                self.y -= v;
            },
            South(v) => {
                self.y += v;
            },
            East(v) => {
                self.x += v;
            },
            West(v) => {
                self.x -= v;
            },
            TurnLeft(v) => {
                self.rotate(360 - v);
            },
            TurnRight(v) => {
                self.rotate(v);
            },
            Forward(v) => {
                match self.direction {
                    0 => {
                        self.y -=v;
                    },
                    90 => {
                        self.x += v;
                    },
                    180 => {
                        self.y += v;
                    },
                    270 => {
                        self.x -= v;
                    },
                    _ => {
                        panic!("Weird angle");
                    }
                }
            }
         }
    }

    fn rotate(&mut self, quant: isize) {
        self.direction = ((self.direction + quant) % 360).abs();
    }

    // Make all ship's movements
    fn go(&mut self) {
        while self.instructions.len() > 0 {
            self.movement();
        }
    }
}

impl Waypoint {
    fn rotate(&mut self, quant: isize) {
        let rotation = (quant % 360).abs();
        match rotation {
            90 => {
                let t = self.x;
                self.x = -self.y; //x = -y
                self.y = t; //y = x
            },
            180 => {
                self.rotate(90);
                self.rotate(90);
            },
            270 => {
                self.rotate(90);
                self.rotate(90);
                self.rotate(90);
            },
            _ => {}
        }
    }
}

fn main() {
    let values = input_processor::dump_vec_separator("inputs/day12", '\n');
    let values = &values[0..values.len() - 1];

    let mut instructions : Vec<Movement> = Vec::new();
    for v in values {
        let typemov = v.chars().next().unwrap();
        let num = (v[1..]).parse::<isize>().unwrap();
        match typemov {
            'N' => {
                instructions.push(Movement::North(num));
            },
            'S' => {
                instructions.push(Movement::South(num));
            },
            'E' => {
                instructions.push(Movement::East(num));
            },
            'W' => {
                instructions.push(Movement::West(num));
            }
            'L' => {
                instructions.push(Movement::TurnLeft(num));
            }
            'R' => {
                instructions.push(Movement::TurnRight(num));
            }
            'F' => {
                instructions.push(Movement::Forward(num));
            }
            _ => {
                panic!("Unexpected character")
            }
        }

    }

    let mut ship = Ship {x: 0, y: 0, direction: 90, instructions: instructions.clone()};
    ship.go();

    println!("Manhattan distance : {}", ship.x.abs() +  ship.y.abs());

    let mut ship = Ship {x: 0, y: 0, direction: 90, instructions: instructions};
    let mut waypoint = Waypoint {x: 10, y: -1};

    go_with_waypoint(&mut ship, &mut waypoint);

    println!("Manhattan distance with waypoint: {}", ship.x.abs() + ship.y.abs());

}

// Calculate movement with waypoint
fn go_with_waypoint(ship: &mut Ship, waypoint: &mut Waypoint) {
    for i in &ship.instructions {
        match i {
            North(v) => {
                waypoint.y -= v;
            },
            South(v) => {
                waypoint.y += v;
            },
            East(v) => {
                waypoint.x += v;
            },
            West(v) => {
                waypoint.x -= v;
            },
            TurnLeft(v) => {
                waypoint.rotate(360 - *v);
            },
            TurnRight(v) => {
                waypoint.rotate(*v);
            },
            Forward(v) => {
                ship.x += waypoint.x * v;
                ship.y += waypoint.y * v;
            }
         }

    }
}