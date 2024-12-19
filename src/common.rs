use std::fmt::Display;
use std::io::Write;

pub use fxhash::FxHashMap as HashMap;
pub use fxhash::FxHashSet as HashSet;
use pad::Alignment;
use pad::PadStr;

pub type Matrix<T> = Vec<Vec<T>>;

pub fn int_len<T>(num: T) -> u32
where
    f64: From<T>,
{
    f64::from(num).log10().floor() as u32 + 1u32
}

#[allow(dead_code)]
pub fn print_matrix<T>(matrix: &Matrix<T>)
where
    T: Display,
{
    let mut max_len = 0;
    for row in matrix.iter() {
        for item in row.iter() {
            max_len = max_len.max(item.to_string().len());
        }
    }
    for row in matrix {
        for item in row {
            let s = item
                .to_string()
                .pad_to_width_with_alignment(max_len, Alignment::Left);
            print!("{}", s);
        }
        println!();
    }
}

#[allow(dead_code)]
pub fn print_matrix_spaced<T>(matrix: &Matrix<T>, space: String)
where
    T: Display,
{
    let mut max_len = 0;
    for row in matrix.iter() {
        for item in row.iter() {
            max_len = max_len.max(item.to_string().len());
        }
    }
    for row in matrix {
        for item in row {
            let s = item
                .to_string()
                .pad_to_width_with_alignment(max_len, Alignment::Left);
            print!("{}{space}", s);
        }
        println!();
    }
}

#[allow(dead_code)]
pub fn write_matrix<T>(matrix: &Matrix<T>, file: &mut impl Write)
where
    T: Display,
{
    let mut max_len = 0;
    for row in matrix.iter() {
        for item in row.iter() {
            max_len = max_len.max(item.to_string().len());
        }
    }
    for row in matrix {
        for item in row {
            let s = item
                .to_string()
                .pad_to_width_with_alignment(max_len, Alignment::Left);
            write!(file, "{}", s).expect("Unable to write to file");
        }
        writeln!(file).expect("Unable to write to file");
    }
}
#[allow(dead_code)]
pub fn write_matrix_spaced<T>(matrix: &Matrix<T>, file: &mut impl Write, space: String)
where
    T: Display,
{
    let mut max_len = 0;
    for row in matrix.iter() {
        for item in row.iter() {
            max_len = max_len.max(item.to_string().len());
        }
    }
    for row in matrix {
        for item in row {
            let s = item
                .to_string()
                .pad_to_width_with_alignment(max_len, Alignment::Left);
            write!(file, "{}{space}", s).expect("Unable to write to file");
        }
        writeln!(file).expect("Unable to write to file");
    }
}

use std::cmp::Reverse; // Import Ordering and Reverse from cmp module.
use std::collections::BinaryHeap;

pub fn dijkstra(
    grid: &Vec<Vec<char>>,
    start_pos: (usize, usize),
) -> HashMap<(usize, usize), Option<(usize, usize)>> {
    let rows = grid.len();
    let cols = if !grid.is_empty() { grid[0].len() } else { 0 };

    // Define possible directions to move (up, down, left, right).
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut distances: HashMap<(usize, usize), usize> = HashMap::default();
    let mut previous_points: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::default();
    let mut priority_queue = BinaryHeap::new(); // This will act as our min heap.

    // Initialize distances and previous points.
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != '#' {
                // Ignore walls ('#').
                let point = (row, col);
                distances.insert(point, usize::MAX);
                previous_points.insert(point, None);
            }
        }
    }
    *distances.get_mut(&start_pos).unwrap() = 0; // Set the distance of start position to zero.
    priority_queue.push((Reverse(0), start_pos)); // Start from the given point (distance is wrapped with Reverse to act as min heap)

    while let Some((Reverse(_), current)) = priority_queue.pop() {
        // Unwrap distance when popping from queue.
        for direction in DIRECTIONS {
            // For each possible movement...
            let new_row = current.0 as i32 + direction.0; // Calculate the row of the next point based on the direction and current position.
            let new_col = current.1 as i32 + direction.1; // Calculate the column of the next point based on the direction and current position.

            if new_row >= 0
                && (new_row as usize) < rows
                && new_col >= 0
                && (new_col as usize) < cols
            {
                // If the next point is within grid bounds...
                let next = ((new_row as usize), (new_col as usize));
                if grid[next.0][next.1] != '#' {
                    // And it's not a wall...
                    let distance = distances[&current] + 1; // Calculate new distance to the next point.
                    if distance < distances[&next] {
                        // If this path is shorter than any known so far...
                        *distances.get_mut(&next).unwrap() = distance; // Update shortest known path to this point.
                        *previous_points.get_mut(&next).unwrap() = Some(current); // And update the previous point on that shortest path.
                        priority_queue.push((Reverse(distance), next)); // Add it to our queue (wrapping distance with Reverse) for further exploration.
                    }
                }
            }
        }
    }
    previous_points // Return the map of shortest paths from start position to every other reachable point on grid.
}
