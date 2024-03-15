
use crate::*;

// Function to perform circular BFS (Breadth First Search) from the core cell
pub fn circular_bfs(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize, group_id: usize, desired_total_cells: usize) {
    let mut queue = VecDeque::new(); // Use a deque as a queue
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()]; // Create a 2D array to keep track of visited cells

    queue.push_back((x, y)); // Add the core cell to the queue
    visited[x][y] = true; // Mark the core cell as visited

    let mut count = 0; // Keep track of the number of cells visited

    while let Some((cx, cy)) = queue.pop_front() { // While the queue is not empty
       // if grid[cx][cy].territory.is_taken { // If the cell is already occupied, skip it
       //     continue;
       // } else {
        occupy_this_cell(&mut grid[cx][cy], group_id); // Occupy the cell
       // }
        count += 1; // Increment the count of visited cells

        if count >= desired_total_cells { // If the desired number of cells is reached, break the loop
            break;
        }

        // Explore neighbors in a circular fashion
        let radius = 5.0;       // radius of the circle           
        let mut angle = 0.0;    // angle in radians       

        while angle <= 2.0 * std::f64::consts::PI {
            let nx = (cx as f64 + (radius * angle.cos()).round()) as usize; // x = cx + r * cos(a)
            let ny = (cy as f64 + (radius * angle.sin()).round()) as usize; // y = cy + r * sin(a)


            if nx < grid.len() && ny < grid[0].len() && !visited[nx][ny] { // Check if the cell is within the grid and not visited
                if grid[nx][ny].quality > 0.0 && !grid[nx][ny].territory.is_taken { // changed quality check
                    queue.push_back((nx, ny));
                    visited[nx][ny] = true;
                }
            }

            angle += std::f64::consts::PI / 180.0; //12.0;      // increment the angle
        }
    }
  //  println!("Group {} has occupied {} cells", group_id, count);
}


pub fn circular_bfs_dummy(grid: &Vec<Vec<Cell>>, x: usize, y: usize, desired_total_cells: usize) -> usize {
    let mut queue = VecDeque::new(); // Use a deque as a queue
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()]; // Create a 2D array to keep track of visited cells

    queue.push_back((x, y)); // Add the core cell to the queue
    visited[x][y] = true; // Mark the core cell as visited

    let mut count = 0; // Keep track of the number of cells visited

    while let Some((cx, cy)) = queue.pop_front() { // While the queue is not empty
       // if grid[cx][cy].territory.is_taken { // If the cell is already occupied, skip it
       //     continue;
       // } else {
       // occupy_this_cell(&mut grid[cx][cy], group_id); // Occupy the cell
       // }
        count += 1; // Increment the count of visited cells

        if count >= desired_total_cells { // If the desired number of cells is reached, break the loop
           return count;
        }

        // Explore neighbors in a circular fashion
        let radius = 5.0;       // radius of the circle           
        let mut angle = 0.0;    // angle in radians       

        while angle <= 2.0 * std::f64::consts::PI {
            let nx = (cx as f64 + (radius * angle.cos()).round()) as usize; // x = cx + r * cos(a)
            let ny = (cy as f64 + (radius * angle.sin()).round()) as usize; // y = cy + r * sin(a)


            if nx < grid.len() && ny < grid[0].len() && !visited[nx][ny] { // Check if the cell is within the grid and not visited
                if grid[nx][ny].quality > 0.0 && !grid[nx][ny].territory.is_taken { // changed quality check
                    queue.push_back((nx, ny));
                    visited[nx][ny] = true;
                }
            }

            angle += std::f64::consts::PI / 180.0; //12.0;      // increment the angle
        }
    }
    return count;
}