mod input_processor;

struct Travel {
    right: usize,
    down: usize
}

fn main() {
    let rows = input_processor::dump_vec_separator("inputs/day03", '\n');
    let rows = &rows[0..rows.len() - 1];
    // TO DO: Find this manually. Will have to use another structure, because Rust
    // doesn't allow two-dimensional arrays to have values calculated at runtime.
    // A HashMap of (Point {x,y}, char) may do the trick but has O(log n) access.
    // Another option is to make a big array like 350 x 50 or something, but 
    // it's not really that much different from this.
    let mut area = [['.'; 323]; 31];
    let width = 31;
    let height = 323;

    for i in 0..height {
        for (j,c) in rows[i].chars().enumerate() {
             area[j][i] = c;
        }
    }

    let count = calculate_trees(width, height, 3, 1, &area);

    println!("Trees found: {}", count);

    let mut res = count;
    let slopes = vec![
        Travel {right: 1, down: 1}, 
        Travel {right: 5, down: 1},
        Travel {right: 7, down: 1},
        Travel {right: 1, down: 2}];
    
    for s in slopes {
        res *= calculate_trees(width, height, s.right, s.down, &area);        
    }    

    println!("Number of trees multiplied: {}", res);
}


fn calculate_trees(width: usize, height: usize, right: usize, down: usize, area: &[[char; 323]; 31]) -> usize {
    let mut x = right;
    let mut y = down;
    let mut count = 0;
    while y < height {
        if area[x][y] == '#' { count += 1; }
        if x + right >= width {
            // The pattern repeats: wrap around area
            let distance_to_end = width - x - 1;
            x = right - 1 - distance_to_end;
        }
        else {
            x += right;
        }
        y += down;
    }

    count
}