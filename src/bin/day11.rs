mod input_processor;

#[derive(Clone)]
struct See {
    up: Option<(usize,usize)>,
    down: Option<(usize,usize)>,
    left: Option<(usize,usize)>,
    right: Option<(usize,usize)>,
    upleft: Option<(usize,usize)>,
    upright: Option<(usize,usize)>,
    downleft: Option<(usize,usize)>,
    downright: Option<(usize,usize)>
}

impl See {
    fn default() -> Self {
        See {up: Option::None, down: Option::None, left: Option::None, right: Option::None, upleft: Option::None, upright: Option::None, downleft: Option::None, downright: Option::None}
    }
}

fn main() {
    let values : Vec <String>= input_processor::dump_vec_separator("inputs/day11", '\n');
    let values = &values[0..values.len()-1];

    let mut cells : Vec<Vec<char>> = vec!(vec!(' '; 99); 99);

    for (x, line) in values.iter().enumerate() {
        for (y, column) in line.chars().enumerate() {
            cells[x][y] = column;
        }
    }

    let mut seats = 0;
    let end = simulate(&cells);
    for line in end {
        let s : Vec<&char> = line.iter().filter(|c| **c == '#').collect();
        seats += s.len();
    }

    println!("Number of occupied seats: {}", seats);

    let see = calc_see(&cells); // Calculate what seats sees every seat
    let end = simulate_see(&cells, &see); // Simulate with new rule
    let mut seats = 0;
    for line in end {
        let s : Vec<&char> = line.iter().filter(|c| **c == '#').collect();
        seats += s.len();
    }

    println!("Number of occupied seats with 2nd algorithm: {}", seats);

}

// Iterates iterate until it stops changing
fn simulate(cells: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut modified = false;
    let mut res = iterate(&cells, &mut modified);
    while modified {
        res = iterate(&res, &mut modified);
    }

    res

}

// Iteration with part 1 rules
fn iterate(cells: &Vec<Vec<char>>, modified: &mut bool) -> Vec<Vec<char>> {
    let mut new = vec!(vec!(' '; 99); 99);
    *modified = false;
    for i in 0..cells.len() {
        for j in 0..cells.len() { //Square array
            let mut occupied = 0;

            if j > 0 {
                match cells[i][j-1] {
                    '#' => {occupied += 1;},
                    _ => {}
                }
            } 
            if j + 1 < cells.len() {
                match cells[i][j+1] {
                    '#' => {occupied +=1;},
                    _ => {}
                }
            } 
            if i > 0 {
                match cells[i-1][j] {
                    '#' => {occupied +=1;},
                    _ => {}
                }
            } 
            if i + 1 < cells.len() {
                match cells[i+1][j] {
                    '#' => {occupied +=1;},
                    _ => {}
                }
            } 

            if i > 0 && j > 0 {
                match cells[i-1][j-1] {
                    '#' => {occupied +=1;},
                    _ => {}
                }
            } 

            if i + 1 < cells.len() && j > 0 {
                match cells[i+1][j-1] {
                    '#' => {occupied +=1;},
                    _ => {}
                }
            } 

            if i > 0 && j + 1 < cells.len() {
                match cells[i-1][j+1] {
                    '#' => {occupied +=1;},
                    _ => {}
                }
            } 

            if i + 1 < cells.len() && j + 1 < cells.len() {
                match cells[i+1][j+1] {
                    '#' => {occupied +=1;},
                    _ => {}
                }
            } 

            match cells[i][j] {
                'L' => {
                    if occupied == 0 {
                        new[i][j] = '#';
                        *modified = true;
                    }
                    else {
                        new[i][j] = cells[i][j];
                    }
                }
                '#' => {
                    if occupied >= 4 {
                        new[i][j] = 'L';
                        *modified = true;
                    }
                    else {
                        new[i][j] = cells[i][j];
                    }
                }
                _ => {
                    new[i][j] = cells[i][j];
                }
            }
        }
    }

    new
}

//Not necessary, debug purposes (or to see it pretty <3)
fn show(cells: &Vec<Vec<char>>) {
    println!("---------------------------------------");
    for vec in cells.iter() {
        for c in vec.iter() {
            print!("{}", c);
        }
        println!("");
    }
    println!("---------------------------------------");
}

// Calculates what seats are seen by every seat, storing the values as 2D vector of See structs
fn calc_see(cells: &Vec<Vec<char>>) -> Vec<Vec<See>> {
    let mut res = vec!(vec!(See::default(); 99); 99);
    for (i, line) in cells.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            let mut var = 0;
            let mut var_i = 0;
            let mut var_j = 0;
            if i > 0 { //up
                var = i;
                let mut found = false;
                while !found && var > 0 {
                    var -= 1;
                    if cells[var][j] == 'L' {
                        found = true;
                    }
                }

                if found {
                    res[i][j].up = Option::Some((var,j));
                }
            }

            if i < cells.len() - 1 { //down
                var = i + 1;
                let mut found = false;
                while !found && var < cells.len() {
                    if cells[var][j] == 'L' {
                        found = true;
                    }
                    else {
                        var += 1;
                    }
                }

                if found {
                    res[i][j].down = Option::Some((var,j));
                }
            }

            if j > 0 { //left
                var = j;
                let mut found = false;
                while !found && var > 0 {
                    var -= 1;
                    if cells[i][var] == 'L' {
                        found = true;
                    }
                }

                if found {
                    res[i][j].left = Option::Some((i,var));
                }
            }

            if j < cells.len() - 1 { //right
                var = j + 1;
                let mut found = false;
                while !found && var < cells.len() {
                    if cells[i][var] == 'L' {
                        found = true;
                    }
                    else {
                        var += 1;
                    }
                }

                if found {
                    res[i][j].right = Option::Some((i,var));
                }
            }

            if i > 0 && j > 0 { //upleft
                var_i = i;
                var_j = j;
                let mut found = false;
                while !found && var_i > 0 && var_j > 0{
                    var_i -= 1;
                    var_j -= 1;
                    if cells[var_i][var_j] == 'L' {
                        found = true;
                    }
                }

                if found {
                    res[i][j].upleft = Option::Some((var_i,var_j));
                }
            }

            if i > 0 && j < cells.len() - 1{ //upright
                var_i = i;
                var_j = j + 1;
                let mut found = false;
                while !found && var_i > 0 && var_j < cells.len() {
                    var_i -= 1;
                    if cells[var_i][var_j] == 'L' {
                        found = true;
                    }
                    else {
                        var_j += 1;
                    }
                }

                if found {
                    res[i][j].upright = Option::Some((var_i,var_j));
                }
            }

            if i < cells.len() - 1 && j > 0 { //downleft
                var_i = i + 1;
                var_j = j;
                let mut found = false;
                while !found && var_i < cells.len() && var_j > 0 {
                    var_j -=1;
                    if cells[var_i][var_j] == 'L' {
                        found = true;
                    }
                    else {
                        var_i += 1;
                    }
                }

                if found {
                    res[i][j].downleft = Option::Some((var_i,var_j));
                }
            }

            if i < cells.len() - 1 && j < cells.len() - 1 { //downright
                var_i = i + 1;
                var_j = j + 1;
                let mut found = false;
                while !found && var_i < cells.len() && var_j < cells.len() {
                    if cells[var_i][var_j] == 'L' {
                        found = true;
                    }
                    else {
                        var_i += 1;
                        var_j += 1;
                    }
                }

                if found {
                    res[i][j].downright = Option::Some((var_i,var_j));
                }
            }

        }
    }

    res
}

// Iterates iterate_see until it stops changing
fn simulate_see(cells: &Vec<Vec<char>>, see: &Vec<Vec<See>>) -> Vec<Vec<char>> {
    let mut modified = false;
    let mut res = iterate_see(&cells, &mut modified, &see);
    while modified {
        res = iterate_see(&res, &mut modified, &see);
    }

    res

}

// Iterate with the modified neighbor rule
fn iterate_see(cells: &Vec<Vec<char>>, modified: &mut bool, see: &Vec<Vec<See>>) -> Vec<Vec<char>> {
    let mut new = vec!(vec!(' '; 99); 99);
    *modified = false;
    for i in 0..cells.len() {
        for j in 0..cells.len() { //Square array
            let mut occupied = 0;
            
            match see[i][j].up {
                Some((a,b)) => {
                    if cells[a][b] == '#' {occupied += 1;}
                },
                _ => {}
            }

            match see[i][j].down {
                Some((a,b)) => {
                    if cells[a][b] == '#' {occupied += 1;}
                },
                _ => {}
            }

            match see[i][j].left {
                Some((a,b)) => {
                    if cells[a][b] == '#' {occupied += 1;}
                },
                _ => {}
            }

            match see[i][j].right {
                Some((a,b)) => {
                    if cells[a][b] == '#' {occupied += 1;}
                },
                _ => {}
            }

            match see[i][j].upleft {
                Some((a,b)) => {
                    if cells[a][b] == '#' {occupied += 1;}
                },
                _ => {}
            }

            match see[i][j].upright {
                Some((a,b)) => {
                    if cells[a][b] == '#' {occupied += 1;}
                },
                _ => {}
            }

            match see[i][j].downleft {
                Some((a,b)) => {
                    if cells[a][b] == '#' {occupied += 1;}
                },
                _ => {}
            }

            match see[i][j].downright {
                Some((a,b)) => {
                    if cells[a][b] == '#' {occupied += 1;}
                },
                _ => {}
            }

            match cells[i][j] {
                'L' => {
                    if occupied == 0 {
                        new[i][j] = '#';
                        *modified = true;
                    }
                    else {
                        new[i][j] = cells[i][j];
                    }
                }
                '#' => {
                    if occupied >= 5 {
                        new[i][j] = 'L';
                        *modified = true;
                    }
                    else {
                        new[i][j] = cells[i][j];
                    }
                }
                _ => {
                    new[i][j] = cells[i][j];
                }
            }
        }
    }

    new
}
