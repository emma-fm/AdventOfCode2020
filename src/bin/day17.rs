mod input_processor;


fn main() {
    let values : Vec <String>= input_processor::dump_vec_separator("inputs/day17", '\n');
    let values = &values[0..values.len()-1];

    let mut cells : Vec<Vec<Vec<char>>> = vec!(vec!(vec!('.'; 30); 30); 30); 

    for (x, row) in values.iter().enumerate() {
        for (y, column) in row.chars().enumerate() {
            cells[x+10][y+10][10] = column;
        }
    }

    let mut active = 0;
    let end = simulate_3d(&cells, 6);

    for n in end {
        for line in n {
            let s : Vec<&char> = line.iter().filter(|c| **c == '#').collect();
            active += s.len();
        }
    }

    println!("Number of active cubes in 3 dimensions: {}", active);

    // PART 2, FOUR DIMENSIONS

    let mut cells : Vec<Vec<Vec<Vec<char>>>> = vec!(vec!(vec!(vec!('.'; 30); 30); 30); 30);

    for (x, row) in values.iter().enumerate() {
        for (y, column) in row.chars().enumerate() {
            cells[x+10][y+10][10][10] = column;
        }
    }


    let mut active = 0;
    let end = simulate_4d(&cells, 6);

    for n in end {
        for s in n {
            for line in s { 
                let s : Vec<&char> = line.iter().filter(|c| **c == '#').collect();
                active += s.len();
            }
        }
    }

    println!("Number of active cubes in 4 dimensions: {}", active);



}

// Iterates iterate_3d 'time' times (3D)
fn simulate_3d(cells: &Vec<Vec<Vec<char>>>, time: usize) -> Vec<Vec<Vec<char>>> {
    let mut res = iterate_3d(&cells);
    for _ in 1..time {
        res = iterate_3d(&res);
    }

    res
}


fn iterate_3d(cells: &Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let mut new = vec!(vec!(vec!(' '; 30); 30); 30);
    for i in 0..cells.len() {
        for j in 0..cells.len() { 
            for k in 0..cells.len() { //Cubic array
                let mut neighbors = 0;

                for x in (i as isize -1)..(i as isize + 2isize) {
                    for y in (j as isize - 1)..(j as isize + 2isize) {
                        for z in (k as isize  - 1)..(k as isize + 2isize) {

                            if x >= 0 && x < cells.len() as isize && y >= 0 && y < cells.len() as isize && z >= 0 && z < cells.len() as isize
                            && !(x == i as isize && y == j as isize && z == k as isize) {
                                match cells[x as usize][y as usize][z as usize] {
                                    '#' => {
                                        neighbors += 1;
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                match cells[i][j][k] {
                    '#' => {
                        if neighbors == 2 || neighbors == 3 {
                            new[i][j][k] = '#';
                        }
                        else {
                            new[i][j][k] = '.';
                        }
                    }
                    '.' => {
                        if neighbors == 3 {
                            new[i][j][k] = '#';                        
                        }
                        else {
                            new[i][j][k] = '.';
                        }
                    }
                    _ => {
                        panic!("Unexpected character!");
                    }
                }
            }
        }
    }

    new
}

// Iterates iterate_4d 'time' times (4D)
fn simulate_4d(cells: &Vec<Vec<Vec<Vec<char>>>>, time: usize) -> Vec<Vec<Vec<Vec<char>>>> {
    let mut res = iterate_4d(&cells);
    for _ in 1..time {
        res = iterate_4d(&res);
    }

    res
}


fn iterate_4d(cells: &Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<Vec<Vec<char>>>> {
    let mut new = vec!(vec!(vec!(vec!(' '; 30); 30); 30); 30);
    for i in 0..cells.len() {
        for j in 0..cells.len() { 
            for k in 0..cells.len() {
                for t in 0..cells.len() { //Hypercubic array

                    let mut neighbors = 0;

                    for x in (i as isize -1)..(i as isize + 2isize) {
                        for y in (j as isize - 1)..(j as isize + 2isize) {
                            for z in (k as isize  - 1)..(k as isize + 2isize) {
                                for w in (t as isize  - 1)..(t as isize + 2isize) {

                                    if x >= 0 && x < cells.len() as isize && y >= 0 && y < cells.len() as isize && z >= 0 && z < cells.len() as isize && w >= 0 && w < cells.len() as isize
                                    && !(x == i as isize && y == j as isize && z == k as isize && w == t as isize) {
                                        match cells[x as usize][y as usize][z as usize][w as usize] {
                                            '#' => {
                                                neighbors += 1;
                                            },
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    match cells[i][j][k][t] {
                        '#' => {
                            if neighbors == 2 || neighbors == 3 {
                                new[i][j][k][t] = '#';
                            }
                            else {
                                new[i][j][k][t] = '.';
                            }
                        }
                        '.' => {
                            if neighbors == 3 {
                                new[i][j][k][t] = '#';                        
                            }
                            else {
                                new[i][j][k][t] = '.';
                            }
                        }
                        _ => {
                            panic!("Unexpected character!");
                        }
                    }
                }
            }
        }
    }

    new
}

//Not necessary, debug purposes (or to see it pretty <3). Shows z coordinate specified
fn show_3d(cells: &Vec<Vec<Vec<char>>>, z: usize) {
    for x in 0..cells.len() {
        for y in 0..cells.len() {
            for z in 0..cells.len() {
                if z == 10 {
                    print!("{}", cells[x][y][z]);
                }
            }
        }
        println!("");
    }
    println!("---------------------------------");
}
