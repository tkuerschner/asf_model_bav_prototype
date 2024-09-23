

use crate::*;


//pub struct HrCoreCells {
//    x: usize,
//    y: usize,
//    is_taken: bool,
//    taken_by: Option<usize>,
//}
//
//impl HrCoreCells {
//  pub fn occupy_core_cell(&mut self, group_id: usize) {
//    self.is_taken = true;
//    self.taken_by = Some(group_id);
//  }
//    
//}


//pub fn circular_bfs_dummy(grid: &Vec<Vec<Cell>>, x: usize, y: usize, desired_total_cells: usize) -> usize {
//    let mut queue = VecDeque::new(); // Use a deque as a queue
//    let mut visited = vec![vec![false; grid[0].len()]; grid.len()]; // Create a 2D array to keep track of visited cells
//
//    queue.push_back((x, y)); // Add the core cell to the queue
//    visited[x][y] = true; // Mark the core cell as visited
//
//    let mut count = 0; // Keep track of the number of cells visited
//
//    while let Some((cx, cy)) = queue.pop_front() { // While the queue is not empty
//       // if grid[cx][cy].territory.is_taken { // If the cell is already occupied, skip it
//       //     continue;
//       // } else {
//       // occupy_this_cell(&mut grid[cx][cy], group_id); // Occupy the cell
//       // }
//        count += 1; // Increment the count of visited cells
//
//        if count >= desired_total_cells { // If the desired number of cells is reached, break the loop
//           return count;
//        }
//
//        // Explore neighbors in a circular fashion
//        let radius = 5.0;       // radius of the circle           
//        let mut angle = 0.0;    // angle in radians       
//
//        while angle <= 2.0 * std::f64::consts::PI {
//            let nx = (cx as f64 + (radius * angle.cos()).round()) as usize; // x = cx + r * cos(a)
//            let ny = (cy as f64 + (radius * angle.sin()).round()) as usize; // y = cy + r * sin(a)
//
//            if nx < grid.len() && ny < grid[0].len() && !visited[nx][ny] { // Check if the cell is within the grid and not visited
//                if grid[nx][ny].quality > 0.0 && !grid[nx][ny].territory.is_taken { // changed quality check
//                    queue.push_back((nx, ny));
//                    visited[nx][ny] = true;
//                }
//            }
//
//            angle += std::f64::consts::PI / 180.0; //12.0;      // increment the angle
//        }
//    }
//    return count;
//}

//pub fn is_valid_territory(grid: &Vec<Vec<Cell>>, x: usize, y: usize, desired_total_cells: usize) -> bool {
//
//    if circular_bfs_dummy(grid, x, y, desired_total_cells) < 400 {
//        return false;
//    } else {
//        return true;
//
//    }
//
//}



pub fn is_valid_territory(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    if dummy_expand_territory_with_natural_shape(x, y, grid) < 400 {
        //println!("Not a valid territory");
        false
    } else {
        true
    }
}

pub fn dummy_expand_territory_with_natural_shape(x: usize, y: usize, grid: &Vec<Vec<Cell>>) -> usize {
    // Constants for desired number of cells and shape
    //let min_desired_cells = 400;
    let max_desired_cells = 800;
    let shape_factor = 0.5; // Adjust shape factor for desired shape

    let mut territory_cells = HashSet::new();

    // Start with the core cell
    territory_cells.insert((x, y));

    // Keep track of the number of cells claimed
    let mut claimed_cells = 1;
    // Keep track of the number of iterations
    let mut iterations = 0;

    // Expand territory until desired number of cells is reached or max iterations exceeded
    while claimed_cells < max_desired_cells && iterations < 1000 {
         // Increment iterations count
         iterations += 1;
        // Clone the current set of territory cells
        let current_territory_cells = territory_cells.clone();
        // Iterate over the current territory cells
        for (x, y) in current_territory_cells {
            // Iterate over neighboring cells
            for dx in -1..= 1 {
                for dy in -1..= 1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let new_x = x as isize + dx;
                    let new_y = y as isize + dy;
                    // Check if the neighboring cell is within grid bounds
                    if new_x >= 0
                        && new_x < grid.len() as isize
                        && new_y >= 0
                        && new_y < grid[0].len() as isize
                    {
                        let new_x = new_x as usize;
                        let new_y = new_y as usize;
                        // Check if the cell is unoccupied and has positive quality
                        if !grid[new_x][new_y].territory.is_taken && grid[new_x][new_y].quality > 0.0 {
                            // Calculate distance from core cell
                            let distance = ((new_x as f64 - x as f64).powi(2) + (new_y as f64 - y as f64).powi(2)).sqrt();
                            // Bias selection based on distance for circular shape
                            let random_value = rand::random::<f64>();
                            if random_value < 1.0 / (1.0 + shape_factor * distance) {
                                territory_cells.insert((new_x, new_y));
                                claimed_cells += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    //println!("claimed_cells: {}", claimed_cells);
    // Return the number of cells claimed
    //territory_cells.len()
    claimed_cells
   
}

//pub fn territory_has_attraction_points(grid: &Vec<Vec<Cell>>, group_id: usize) -> bool {
//   
//}


