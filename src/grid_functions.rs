// Landscape / grid functions

use crate::*;
use core::num;
use std::f64::consts::PI;
use rand_distr::{Distribution, Normal};


pub fn landscape_setup_from_ascii(file_path: &str) -> io::Result<(Vec<Vec<Cell>>, LandscapeMetadata)> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;

    // Create a cloneable reader
    let reader = BufReader::new(file);

    // Skip the header
    let mut lines = reader.lines();
    for _ in 0..6 {
        lines.next();
    }

    // Create a new BufReader from the file
    let mut new_reader = BufReader::new(File::open(file_path)?);

    // Extract metadata from the file
    let metadata = extract_metadata(&mut new_reader)?;

    // Determine grid size from the file
    let nrows = metadata.nrows;
    let ncols = metadata.ncols;

    // Initialize the grid with quality values from the ASCII file
    let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(nrows);

    // Read the rows
    for i in 0..nrows {
        if let Some(Ok(line)) = lines.next() {
            let row: Vec<Cell> = line
                .split_whitespace()
                .enumerate()
                .map(|(j, s)| Cell {
                    quality: s.parse().unwrap_or(0.0),
                    counter: 0,
                    x_grid: j,
                    y_grid: i,
                    territory: AreaSeparation{
                        is_ap:false,
                        is_taken:false,
                        taken_by_group:0,
                        core_cell_of_group:0,
                    },
                })
                //.filter(|cell| cell.quality > 0.0)
                .collect();

            grid.push(row);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data rows in the data file",
            ));
        }
    }

    Ok((grid, metadata))
}

pub fn extract_metadata(reader: &mut BufReader<File>) -> Result<LandscapeMetadata> {
    // Variables to store metadata values
    let mut ncols = 0;
    let mut nrows = 0;
    let mut xllcorner = 0;
    let mut yllcorner = 0;
    let mut cellsize = 0.0;
    let mut nodata_value = 0;

    // Read metadata lines
    for _ in 0..6 {
        let mut line = String::new();
        reader.read_line(&mut line)?;

        // Parse metadata from each line
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid format in metadata lines",
            ));
        }

        let key = parts[0];
        let value = parts[1].parse::<i32>().map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse metadata value for key: {}", key),
            )
        })?;

        // Assign values to the appropriate metadata fields
        match key {
            "NCOLS" => ncols = value as usize,
            "NROWS" => nrows = value as usize,
            "XLLCORNER" => xllcorner = value as usize,
            "YLLCORNER" => yllcorner = value as usize,
            "CELLSIZE" => cellsize = value as f64,
            "NODATA_value" => nodata_value = value,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unknown metadata key: {}", key),
                ));
            }
        }
    }

    // Create and return the metadata struct
    let metadata = LandscapeMetadata {
        ncols,
        nrows,
        xllcorner,
        yllcorner,
        cellsize,
        nodata_value,
    };

    Ok(metadata)
}

pub fn extract_cell_info(grid: &Vec<Vec<Cell>>) -> Vec<CellInfo> {
    let mut cell_info_list = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let info = CellInfo {
                x_grid_o: i,
                y_grid_o: j,
                quality: cell.quality,
            };
            cell_info_list.push(info);
        }
    }

    cell_info_list.retain(|cell_info| cell_info.quality >= 0.0);
    
    cell_info_list
}

pub fn save_cellinfo_as_csv(filename: &str, cell_info_list: &Vec<CellInfo> ) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "x_grid_o,y_grid_o,quality")?;

    // Write each iteration's global variables
    for (iteration, cellinfo) in cell_info_list.iter().enumerate() {
        writeln!(file, "{},{},{}",cellinfo.x_grid_o, cellinfo.y_grid_o, cellinfo.quality)?;
        // Add more variables as needed
    }

    println!("Global variables saved to: {}", filename);
    Ok(())
}

pub fn flip_grid(grid: &mut Vec<Vec<Cell>>) {
    let nrows = grid.len();
    let ncols = if nrows > 0 { grid[0].len() } else { 0 };

    for i in 0..nrows {
        for j in 0..ncols {
            let new_x = j; // New x gets the column index
            let new_y = nrows - i - 1; // New y gets the value max(old y) - old y

            grid[i][j].x_grid = new_x;//cols
            grid[i][j].y_grid = new_y;//rows
        }
    }
}

pub fn filter_grid(grid: &mut Vec<Vec<Cell>>) {
    for row in grid.iter_mut() {
        row.retain(|cell| cell.quality > 0.0);
    }
}

pub fn filter_grid2(original_grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    original_grid
        .iter()
        .cloned()
        .map(|row| {
            row.into_iter()
                .filter(|cell| cell.quality > 0.0)
                .collect()
        })
        .filter(|row: &Vec<Cell>| !row.is_empty())
        .collect()
}


//new function for checking if cell at xy is valid
pub fn is_valid_cell(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    if x >= grid.len() || y >= grid[0].len() {
        return false;
    }
    let cell = &grid[x][y];
    cell.quality > 0.0
}



pub fn place_attraction_points(grid: &mut Vec<Vec<Cell>>, min_ap_per_chunk: usize, max_ap_per_chunk: usize, chunk_size: usize) {
   

    // chunk the grid in 2x2km blocks and create n ap per chunk
    // cell is 50x50m >>> 2500m^2
    // chunk 4000000 meters^2
    // 1600 cells per chunk
    // no cell cant be in 2 chunks at a time
    // place 3-6 randomly positioned attraction points per chunk




    // Extract cells with quality > 0
    let cells_with_quality: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, cell)| cell.quality > 0.0).map(move |(j, _)| (i, j)))
        .collect();

    // Determine the number of chunks needed
    let num_chunks = (cells_with_quality.len() + chunk_size - 1) / chunk_size;

    // Shuffle the cells to randomize chunk distribution
    let mut rng = rand::thread_rng();
    let mut shuffled_cells = cells_with_quality.clone();
    shuffled_cells.shuffle(&mut rng);

    // Divide cells into chunks
    let chunks: Vec<Vec<(usize, usize)>> = shuffled_cells.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();

    // Place attraction points in each chunk
    for chunk in chunks {
        let num_ap = rng.gen_range(min_ap_per_chunk..=max_ap_per_chunk);

        for _ in 0..num_ap {
            if let Some(cell) = chunk.choose(&mut rng) {
                grid[cell.0][cell.1].territory.is_ap = true;
            }
        }
    }
}

//function that removes all attraction points that are not core cells rewrite using cell.territory.is_core_cell_of_group
pub fn remove_non_core_attraction_points(grid: &mut Vec<Vec<Cell>>) {

    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if cell.territory.core_cell_of_group == 0 {
                cell.territory.is_ap = false;
            }
        }
    }
}



//
////function that returns a subset of the grid for each group containing only the groups territory

pub fn get_group_territory(grid: &Vec<Vec<Cell>>, groups: &Vec<Groups>) -> Vec<Vec<Cell>> {
    let mut group_territory = Vec::new();

    // Iterate over the grid cells and filter those claimed by the group
    for row in grid.iter() {
        let mut filtered_row = Vec::new();
        for cell in row.iter() {
            if groups.iter().any(|group| cell.territory.taken_by_group == group.group_id as usize) {
                filtered_row.push(cell.clone());
            }
        }
        group_territory.push(filtered_row);
    }

    group_territory
}



pub fn place_additional_attraction_points(grid: &mut Vec<Vec<Cell>>, groups: &mut Vec<Groups>, mut num_points: usize, rng: &mut impl Rng) {

    //iterate through the groups
    for group in groups.iter_mut() {

        //get the cells of the group
        let cells_of_group: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, cell)| cell.territory.taken_by_group == group.group_id).map(move |(j, _)| (i, j)))
        .collect();

        //get the min x and y coordinates of the group
        let min_x = cells_of_group.iter().map(|(x, _)| x).min().unwrap();
        let min_y = cells_of_group.iter().map(|(_, y)| y).min().unwrap();

       // println!("min_x: {}, min_y: {}", min_x, min_y);

        //get the max x and y coordinates of the group
        let max_x = cells_of_group.iter().map(|(x, _)| x).max().unwrap();
        let max_y = cells_of_group.iter().map(|(_, y)| y).max().unwrap();

        //println!("max_x: {}, max_y: {}", max_x, max_y);

        //get the width and height of the group
        let width = max_x - min_x;
        let height = max_y - min_y;

       // println!("width: {}, height: {}", width, height);

        //get the center of the group
        let center_x = min_x  + (width ) / 2;
        let center_y = min_y  + (height) / 2;

        //println!("center_x: {}, center_y: {}", center_x, center_y);

       // let mut rng = rand::thread_rng();
       // let n_ap = rng.gen_range(2..6) as usize;
        // draw a random number with mean 4 and std 1 from a normal distribution
        //let n_ap = (rand::thread_rng().gen_range(2..6) as f64).round() as usize;

      // let n_ap = get_random_normal_int(4.5, 1.5, rng) as usize;
        let n_ap = rng.gen_range(4..8);
       // println!("n_ap: {}", n_ap);
       // num_points = n_ap;

     //   num_points = 8; //n_ap + 2;
//
     //   println!("num_points: {}", num_points);
//
        let mut num_points_sqrt = (num_points as f64).sqrt() as usize;
//
     //   println!("num_points_sqrt: {}", num_points_sqrt);

        // Ensure that the number of points is at least 2
       // if num_points_sqrt == 1 {
       //     num_points_sqrt += 1;
       // }
        let mut max_jitter: isize = 4;

        if num_points_sqrt < 2 {
            max_jitter = 6;
        }

        //println!("max_jitter: {}", max_jitter);

        // Dereference min_x and min_y
        let min_x = *min_x;
        let min_y = *min_y;

        let min_x_f64 = min_x as f64;
        let max_x_f64 = *max_x as f64;

        let min_y_f64 = min_y as f64;
        let max_y_f64 = *max_y as f64;

        // Calculate the width and height of the bounding box as floating-point numbers
        let width_f64 = width as f64;
        let height_f64 = height as f64;

        // Calculate the step sizes for x and y directions
        let step_x = width_f64 / num_points_sqrt as f64;
        let step_y = height_f64 / num_points_sqrt as f64;

        // Iterate to place points
        for i in 0..num_points_sqrt {
            for j in 0..num_points_sqrt {
                // No jitter version
                // Calculate the x and y coordinates for this point
                //let x = min_x as f64 + (i as f64) * step_x + step_x / 2.0;
                //let y = min_y as f64 + (j as f64) * step_y + step_y / 2.0;
                
                //// Ensure the coordinates are within the territory bounds
                //let x_clamped = x.max(min_x_f64).min(max_x_f64) as usize;
                //let y_clamped = y.max(min_y_f64).min(max_y_f64) as usize;
                
                //// Place the attraction point at the calculated coordinates
                //grid[x_clamped as usize][y_clamped as usize].territory.is_ap = true;

               //Jitter version
                let x = min_x as f64 + (i as f64) * step_x + step_x / 2.0;
                let y = min_y as f64 + (j as f64) * step_y + step_y / 2.0;

               // let mut rng = rand::thread_rng();
                let x_offset = rng.gen_range(-max_jitter..=max_jitter + 1) as f64;
                let y_offset = rng.gen_range(-max_jitter..=max_jitter + 1) as f64;

                let x_jittered = (x + x_offset).max(min_x as f64).min(*max_x as f64);
                let y_jittered = (y + y_offset).max(min_y as f64).min(*max_y as f64);

                // Ensure the jittered coordinates are within the territory bounds
                let x_clamped = x_jittered as usize;
                let y_clamped = y_jittered as usize;

                grid[x_clamped as usize][y_clamped as usize].territory.is_ap = true;

            }
        }

    }
}


pub fn get_random_normal_int (mean: f64, std_dev: f64, rng: &mut impl Rng) -> i32 {

    // Create a normal distribution with mean 4.5 and standard deviation 1.5
 
    let normal = Normal::new(mean, std_dev).expect("Invalid parameters");

    // Generate a random number from the normal distribution
    let mut num: f64;
    loop {
        num = rng.sample(normal);
        if num >= 2.0 && num <= 7.0 {
            break;
        }
    }

    // Convert the floating-point number to an integer
    let random_int = num.round() as i32;

    random_int
}


pub fn set_ap_at_individual_position(grid: &mut Vec<Vec<Cell>>, group: &Groups) {
    
        grid[group.x][group.y].territory.is_ap = true;
    
}

// Get a list of all existing attraction points
pub fn get_attraction_points(grid: &Vec<Vec<Cell>>) -> Vec<(usize, usize)>{
    let all_ap: Vec<(usize, usize)> = grid
    .iter()
    .enumerate()
    .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, cell)| cell.territory.is_ap == true).map(move |(j, _)| (i, j)))
    .collect();

    all_ap

}

// Get a list of all existing free attraction points
pub fn get_free_attraction_points(grid: &Vec<Vec<Cell>>) -> Vec<(usize, usize)>{
    let all_ap: Vec<(usize, usize)> = grid
    .iter()
    .enumerate()
    .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, cell)| cell.territory.is_ap == true && cell.territory.is_taken == false).map(move |(j, _)| (i, j)))
    .collect();

    all_ap

}

// function that returns a list of the 10 attraction points closest a specific groups core cell but not taken by that group
pub fn get_closest_attraction_points_outside_territory(grid: &Vec<Vec<Cell>>, group: &Groups) -> Vec<(usize, usize)>{
    let all_ap: Vec<(usize, usize)> = grid
    .iter()
    .enumerate()
    .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, cell)| cell.territory.is_ap == true && cell.territory.is_taken == false && cell.territory.taken_by_group != group.group_id).map(move |(j, _)| (i, j)))
    .collect();

    let mut closest_ap = Vec::new();

    for _ in 0..10 {
        if let Some(ap) = all_ap.iter().min_by_key(|&ap| distance_squared(group.x, group.y, ap.0, ap.1)) {
            closest_ap.push(*ap);
        }
    }

    closest_ap

}

//function to get all the attraction points not take by a specific group
pub fn get_free_attraction_points_for_group(grid: &Vec<Vec<Cell>>, group_id: usize) -> Vec<(usize, usize)>{
    let all_ap: Vec<(usize, usize)> = grid
    .iter()
    .enumerate()
    .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, cell)| cell.territory.is_ap == true && cell.territory.is_taken == false && cell.territory.taken_by_group != group_id).map(move |(j, _)| (i, j)))
    .collect();

    all_ap

}


//TESTER
pub fn get_attraction_points2(grid: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> { // Test which function is faster
   let mut attraction_points = Vec::new();
   for (i, row) in grid.iter().enumerate() {
       for (j, cell) in row.iter().enumerate() {
           if cell.territory.is_ap {
               attraction_points.push((i, j));
           }
       }
   }
   attraction_points
}

pub fn get_closest_attraction_point(group: &Groups, ap_list: &[(usize, usize)]) -> (usize, usize) {
    ap_list
        .iter()
        .cloned()
        .min_by_key(|&ap| distance_squared(group.x, group.y, ap.0, ap.1))
        .expect("No attraction points in territory")
}

// Helper function to calculate squared distance between two points
fn distance_squared(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    ((x1 as isize - x2 as isize).pow(2) + (y1 as isize - y2 as isize).pow(2)) as usize
}

// Occupy an attraction point as core cell and claim the surrounding ap
pub fn occupy_cell_here(grid: &mut Vec<Vec<Cell>>, group: &Groups) {

    grid[group.x][group.y].territory.is_taken = true;
    grid[group.x][group.y].territory.taken_by_group = group.group_id;

}

//pub fn occupy_this_cell(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize, group_id: usize) {
//
//    grid[x][y].territory.is_taken = true;
//    grid[x][y].territory.taken_by_group = group_id;
//
//}

pub fn occupy_this_cell(cell: &mut Cell, group_id: usize) {
    cell.territory.is_taken = true;
    cell.territory.taken_by_group = group_id;
}

pub fn make_core_cell(cell: &mut Cell, group_id: usize) {
    cell.territory.core_cell_of_group = group_id;
}

// Use cells around individuals with radius 1600 to create a full territory
pub fn occupy_territory(grid: &mut Vec<Vec<Cell>>, positions: Vec<(usize, usize)>, id:usize){

    for (x, y) in positions {
        if x < grid.len() && y < grid[0].len() {
            
            grid[x][y].territory.is_taken = true;
            grid[x][y].territory.taken_by_group = id;
            
        }
    }
}

pub fn get_attraction_points_in_territory(grid: &Vec<Vec<Cell>>, group_id: usize) -> Vec<(usize, usize)> {
    // Assuming that get_free_attraction_points returns all attraction points, filter by group_id
    get_attraction_points(grid)
        .into_iter()
        .filter(|&(x, y)| grid[x][y].territory.taken_by_group == group_id)
        .collect()
}

// Returns all cells within a given radius around an individual
pub fn get_cells_around_individual(group: &Groups, grid: &Vec<Vec<Cell>>, range: usize) -> Vec<(usize, usize)> {
    let mut cells_around = Vec::new();
    let current_x = group.x;
    let current_y = group.y;

    for i in (current_x.saturating_sub(range))..=(current_x + range) {
        for j in (current_y.saturating_sub(range))..=(current_y + range) {
            if i < grid.len() && j < grid[0].len() {
                cells_around.push((i, j));
            }
        }
    }

    cells_around
}



//function to remove attraction points that are on cells with quality 0
pub fn remove_ap_on_cells_with_quality_0(grid: &mut Vec<Vec<Cell>>) {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if cell.quality == 0.0 {
                cell.territory.is_ap = false;
            }
        }
    }
}