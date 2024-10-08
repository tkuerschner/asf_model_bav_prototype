// Landscape / grid functions

use crate::*;
//use core::num;
//use std::f64::consts::PI;
//use rand_distr::{Distribution, Normal};

// Define a struct to represent a grid cellFinteraction
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub quality: f64,
    pub counter: usize,
    pub x_grid: usize,
    pub y_grid: usize,
    pub territory: AreaSeparation,
    pub hunting_zone: bool,
    pub associated_high_seat: Option<usize>,
}

impl Cell {
    pub fn is_valid(&self) -> bool {
        self.quality > 0.0
    }

    pub fn set_hunting_zone(&mut self) {
        self.hunting_zone = true;
    }

}

// Define a struct to represent the area separation
#[derive(Debug, Clone, PartialEq)]
pub struct AreaSeparation {
    pub is_ap: bool,
    pub is_taken:bool,
    pub taken_by_group: usize,
    pub core_cell_of_group: usize,
}

// Define a struct to represent the cell information
pub struct CellInfo {
    pub x_grid_o: usize,
    pub y_grid_o: usize,
    pub quality: f64,
}


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
    //let ncols = metadata.ncols;

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
                    hunting_zone:false,
                    associated_high_seat: None,
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
    //let mut ncols = 0;
    let mut nrows = 0;
    //let mut xllcorner = 0;
    //let mut yllcorner = 0;
    //let mut cellsize = 0.0;
    //let mut nodata_value = 0;

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
            //"NCOLS" => ncols = value as usize,
            "NROWS" => nrows = value as usize,
            //"XLLCORNER" => xllcorner = value as usize,
            //"YLLCORNER" => yllcorner = value as usize,
            //"CELLSIZE" => cellsize = value as f64,
            //"NODATA_value" => nodata_value = value,
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
        //ncols,
        nrows,
        //xllcorner,
        //yllcorner,
        //cellsize,
        //nodata_value,
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

//new function for checking if cell at xy is valid
pub fn is_valid_cell(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    if x >= grid.len() || y >= grid[0].len() {
        return false;
    }
    let cell = &grid[x][y];
    if cell.quality > 0.0 {
        return true;
    } else {
        return false;
    }
}

pub fn random_valid_cell(grid: &Vec<Vec<Cell>>, rng: &mut impl Rng) -> (usize, usize) {
    loop {
        let x = rng.gen_range(0..grid.len());
        let y = rng.gen_range(0..grid[0].len());
        if is_valid_cell(grid, x, y) {
            return (x, y);
        }
    }
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
    //let num_chunks = (cells_with_quality.len() + chunk_size - 1) / chunk_size;

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
                if grid[cell.0][cell.1].quality > 0.0 {
                    grid[cell.0][cell.1].territory.is_ap = true;
                }
                
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

//function that returns a subset of the grid for each group containing only the groups territory
pub fn place_additional_attraction_points(grid: &mut Vec<Vec<Cell>>, groups: &mut Vec<Groups>, num_points: usize, rng: &mut impl Rng) {

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


        //let center_x = min_x  + (width ) / 2;
        //let center_y = min_y  + (height) / 2;
        //let n_ap = rng.gen_range(4..8);
        let num_points_sqrt = (num_points as f64).sqrt() as usize;

        let mut max_jitter: isize = 4;

        if num_points_sqrt < 2 {
            max_jitter = 6;
        }

        //println!("max_jitter: {}", max_jitter);

        // Dereference min_x and min_y
        let min_x = *min_x;
        let min_y = *min_y;

        //let min_x_f64 = min_x as f64;
        //let max_x_f64 = *max_x as f64;

        //let min_y_f64 = min_y as f64;
        //let max_y_f64 = *max_y as f64;

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

                if grid[x_clamped as usize][y_clamped as usize].quality > 0.0 {
                grid[x_clamped as usize][y_clamped as usize].territory.is_ap = true;
                }

            }
        }

        //count the number of ap in the group
        let mut ap_count = 0;
        for row in grid.iter() {
            for cell in row.iter() {
                if cell.territory.taken_by_group == group.group_id && cell.territory.is_ap {
                    ap_count += 1;
                }
            }
        }
        //println!("AP count: {}", ap_count);

        if ap_count <= 2 {
        
        while ap_count < 4 {
            
           // create random ap in the groups territory
           // filter cells_of_group to exclude is_ap == true
            let territory_of_group: Vec<(usize, usize)> = cells_of_group.iter().filter(|(x, y)| grid[*x][*y].territory.is_ap == false).cloned().collect();
            let random_ap = territory_of_group.choose(rng).unwrap();
            grid[random_ap.0][random_ap.1].territory.is_ap = true;
            ap_count += 1;
         }
        }

    }
}

//pub fn place_dynamic_attraction_points(grid: &mut Vec<Vec<Cell>>, groups: &mut Vec<Groups>, num_points: usize, rng: &mut impl Rng, season: &str) {
//
//    //iterate through the groups
//    for group in groups.iter_mut() {
//        //take all the cells occupied by this group
//        let cells_of_group: Vec<(usize, usize)> = grid // FIX ME: This is a very inefficient way to get the cells of a group
//            .iter()
//            .enumerate()
//            .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, cell)| cell.territory.taken_by_group == group.group_id).map(move |(j, _)| (i, j)))
//            .collect();
//
//        // get the core cell of the group
//        let core_cell = group.core_cell.unwrap();
//        let mut ap_to_place = 1;
//
//        if season == "summer" {
//        ap_to_place = 6;
//        } else {
//        ap_to_place = 3;
//        }
//
//            // take 3 cells from the group cells that are equidistant from the core cell and each other (min distance 15 cells), jitter them and place an attraction point on them
//            let mut cells_to_place_ap: Vec<(usize, usize)> = Vec::new();
//            let mut cells_to_place_ap_jittered: Vec<(usize, usize)> = Vec::new();
//
//            // get the distance between the core cell and each cell in the group
//            let mut distances: Vec<(usize, usize, usize)> = Vec::new();
//            for cell in cells_of_group.iter() {
//                let distance = distance_squared(core_cell.0, core_cell.1, cell.0, cell.1);
//                distances.push((cell.0, cell.1, distance));
//            }
//
//            // sort the distances
//            distances.sort_by(|a, b| a.2.cmp(&b.2));
//
//            // get the 3 cells that are equidistant from the core cell and each other
//            let mut i = 0;
//            while cells_to_place_ap.len() < ap_to_place {
//                let cell = distances[i];
//                let mut is_equidistant = true;
//                for cell_to_place in cells_to_place_ap.iter() {
//                    if distance_squared(cell.0, cell.1, cell_to_place.0, cell_to_place.1) < 15 * 15 {
//                        is_equidistant = false;
//                        break;
//                    }
//                }
//                if is_equidistant {
//                    cells_to_place_ap.push((cell.0, cell.1));
//                }
//                i += 1;
//            }
//
//            // jitter the cells
//
//            for cell in cells_to_place_ap.iter() {
//                let x = cell.0 as f64;
//                let y = cell.1 as f64;
//                let x_offset = rng.gen_range(-4..=4) as f64;
//                let y_offset = rng.gen_range(-4..=4) as f64;
//                let x_jittered = (x + x_offset).max(0.0).min(grid.len() as f64 - 1.0);
//                let y_jittered = (y + y_offset).max(0.0).min(grid[0].len() as f64 - 1.0);
//                cells_to_place_ap_jittered.push((x_jittered as usize, y_jittered as usize));
//            }
//
//            // place the attraction points on the jittered cells
//            for cell in cells_to_place_ap_jittered.iter() {
//                grid[cell.0][cell.1].territory.is_ap = true;
//            }
//        }
//
//    }

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

pub fn get_closest_attraction_point(group: &Groups, ap_list: &[(usize, usize)]) -> (usize, usize) {
    ap_list
        .iter()
        .cloned()
        .min_by_key(|&ap| distance_squared(group.x, group.y, ap.0, ap.1))
        .expect("No attraction points in territory")
}

// Helper function to calculate squared distance between two points
//fn distance_squared(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
//    ((x1 as isize - x2 as isize).pow(2) + (y1 as isize - y2 as isize).pow(2)) as usize
//}

pub fn distance_squared(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let dx = x1 as isize - x2 as isize;
    let dy = y1 as isize - y2 as isize;
    (dx * dx + dy * dy) as usize
}

pub fn occupy_this_cell(cell: &mut Cell, group_id: usize) {
    if cell.territory.is_taken || cell.quality == 0.0{
        return;
    } else {
        cell.territory.is_taken = true;
        cell.territory.taken_by_group = group_id;
    }
    //cell.territory.is_taken = true;
    //cell.territory.taken_by_group = group_id;
}

pub fn make_core_cell(cell: &mut Cell, group_id: usize) {
    cell.territory.core_cell_of_group = group_id;
}

pub fn get_attraction_points_in_territory(grid: &Vec<Vec<Cell>>, group_id: usize) -> Vec<(usize, usize)> {
    // Assuming that get_free_attraction_points returns all attraction points, filter by group_id
    get_attraction_points(grid)
        .into_iter()
        .filter(|&(x, y)| grid[x][y].territory.taken_by_group == group_id)
        .collect()
}

pub fn get_random_cell_in_territory(grid: &Vec<Vec<Cell>>, group_id: usize, rng: &mut impl Rng) -> (usize, usize) {
    loop {
        let x = rng.gen_range(0..grid.len());
        let y = rng.gen_range(0..grid[0].len());
        if grid[x][y].territory.taken_by_group == group_id {
            return (x, y);
        }
    }
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

// function to select a cell that is not an attraction point that has no group occupying it and has a distance of at least 600 cells 
// to the nearest cell occupied by a group and is within 2000 cells of a specific set of xy coordinates
pub fn select_random_free_cell_in_range(grid: &Vec<Vec<Cell>>, x: usize, y: usize, rng: &mut impl Rng, groups: &Vec<Groups>) -> (usize, usize) {
    let mut free_cells = Vec::new();
    let mut free_cells_within_range = Vec::new();

    // iterate through the grid and select all cells that are not occupied by a group and are not an attraction point and have a quality > 0
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.territory.is_taken == false && cell.territory.is_ap == false && cell.quality > 0.0 {
                free_cells.push((i, j));
            }
        }
    }
    // print number of free cells
    //println!("Number of free cells: {}", free_cells.len());

    // iterate through the free cells and select all cells that are within 2000 cells of the input x and y coordinates
    for (i, j) in free_cells {
        if distance_squared(i, j, x, y) <= 200 * 200 { // FIX ME 2000 * 2000
            free_cells_within_range.push((i, j));
        }
    }
    // print number of free cells within range
    //println!("Number of free cells within range: {}", free_cells_within_range.len());

    let mut free_cells_within_range_and_far_enough = Vec::new();

    for (i, j) in free_cells_within_range {
        let mut far_enough = true;
        for group in groups.iter() {
            let distance = distance_squared(i, j, group.core_cell.unwrap().0, group.core_cell.unwrap().1);
            if distance <= 20 * 20 {
                //println!("Cell ({}, {}) is too close to group at ({}, {}). Distance: {}", i, j, group.core_cell.unwrap().0, group.core_cell.unwrap().1, distance);
                far_enough = false;
                break;
            }
        }
        if far_enough {
           // println!("Cell ({}, {}) is far enough from all groups", i, j);
            free_cells_within_range_and_far_enough.push((i, j));
        }
    }
    

    // iterate through the free cells within range and select all cells that are at least 600 cells away from the nearest cell occupied by a group
    //for (i, j) in free_cells_within_range {
    //    let mut far_enough = true;
    //    for group in groups.iter() {
    //        if distance_squared(i, j, group.core_cell.unwrap().0, group.core_cell.unwrap().1) <= 600 * 600 {
    //            //println!("Distance to nearest group core: {}", distance_squared(i, j, group.core_cell.unwrap().0, group.core_cell.unwrap().1));
    //            //println!("Cell ({}, {}) is too close to group at ({}, {}). Distance: {}", i, j, group.core_cell.unwrap().0, group.core_cell.unwrap().1, distance.sqrt());
    //            far_enough = false;
    //            break;
    //        }
    //    }
    //    if far_enough {
    //        free_cells_within_range_and_far_enough.push((i, j));
    //    }
    //}
    // print number of free cells within range and far enough
    //println!("Number of free cells within range and far enough: {}", free_cells_within_range_and_far_enough.len());

    // select a random cell from the free cells within range and far enough
    //let random_cell = free_cells_within_range_and_far_enough.choose(rng).unwrap();
    
    // if no cell is found return the x and y coordinates 1 / 1
    //if free_cells_within_range_and_far_enough.len() == 0 {
    //    return (1, 1);
    //}
    //else {
    //    let random_cell = free_cells_within_range_and_far_enough.choose(rng).unwrap();
    //    println!("Selected cell: {:?}", random_cell);
    //    return *random_cell
    //}

    // check if number of cells is  < 10

    //if free_cells_within_range_and_far_enough.len() < 10 {
    //    println!("Number of free cells within range and far enough: {}", free_cells_within_range_and_far_enough.len());
    //    return (1, 1);
    //} 


    if free_cells_within_range_and_far_enough.is_empty() {
        println!("No free cells within range and far enough");
        return (1, 1);
    } else {
       if rand::thread_rng().gen_bool(0.75) { //75% chance to select a random cell
           let random_cell = free_cells_within_range_and_far_enough.choose(rng).unwrap();
           //println!("Selected cell: {:?}", random_cell);
           return *random_cell;
       }else{
         // Select the closest cell to the given position
         let closest_cell = free_cells_within_range_and_far_enough.iter().min_by_key(|&&(i, j)| distance_squared(i, j, x, y)).unwrap();
        // println!("Selected cell: {:?}", closest_cell);
         return *closest_cell;
        }
    }

    
   // *random_cell
}

pub fn check_surrounding(cells: &Vec<Vec<Cell>>, x: usize, y: usize, extent: usize) -> bool {
    // Define boundaries for surrounding area
    let start_x = if x >= extent { x - extent } else { 0 };
    let end_x = if x + extent < cells.len() { x + extent } else { cells.len() - 1 };
    let start_y = if y >= extent { y - extent } else { 0 };
    let end_y = if y + extent < cells[0].len() { y + extent } else { cells[0].len() - 1 };

    // Count consecutive cells with is_taken as false
    let mut consecutive_count = 0;
    for i in start_x..=end_x {
        for j in start_y..=end_y {
            if !cells[i][j].territory.is_taken && cells[i][j].quality > 0.0{
                consecutive_count += 1;
                if consecutive_count >= extent {
                    return true;
                }
            } else {
                consecutive_count = 0;
            }
        }
    }
    false
}

pub fn place_attraction_points_in_territory(grid: &mut Vec<Vec<Cell>>, group_id: usize, num_points: usize, rng: &mut impl Rng) -> bool  {
   
    //get the cells of the group
    let cells_of_group: Vec<(usize, usize)> = grid
    .iter()
    .enumerate()
    .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, cell)| cell.territory.taken_by_group == group_id && cell.quality > 0.0).map(move |(j, _)| (i, j)))
    .collect();

    if cells_of_group.is_empty() {
        println!("No cells in group!!"); // FIX ME find a proper way to deal with those groups
    
        return false;
    }

    if cells_of_group.len() < 25 {
        println!("Number of cells in group: {} < min cell count", cells_of_group.len());
        print!("Group: {} will be deleted", group_id);

        return false;
    }


        

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


    //let center_x = min_x  + (width ) / 2;
    //let center_y = min_y  + (height) / 2;
    //let n_ap = rng.gen_range(4..8);
    let num_points_sqrt = (num_points as f64).sqrt() as usize;

    let mut max_jitter: isize = 4;

    if num_points_sqrt < 2 {
        max_jitter = 6;
    }

    //println!("max_jitter: {}", max_jitter);

    // Dereference min_x and min_y
    let min_x = *min_x;
    let min_y = *min_y;

    //let min_x_f64 = min_x as f64;
    //let max_x_f64 = *max_x as f64;
//
    //let min_y_f64 = min_y as f64;
    //let max_y_f64 = *max_y as f64;

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

            if grid[x_clamped as usize][y_clamped as usize].quality > 0.0 {
            grid[x_clamped as usize][y_clamped as usize].territory.is_ap = true;
            }

        }
    }

            //count the number of ap in the group
            let mut ap_count = 0;
            for &(i, j) in &cells_of_group {
                if grid[i][j].territory.is_ap {
                    ap_count += 1;
                }
            }
            //println!("AP count: {}", ap_count);
    
            if ap_count <= 2 {
                //print number of cells ing group
               // println!("Number of cells in group: {}", cells_of_group.len());
               // println!("Number of ap in group: {}", ap_count);
            
            while ap_count < 4 {
                
               // create random ap in the groups territory
               // filter cells_of_group to exclude is_ap == true
                let territory_of_group: Vec<(usize, usize)> = cells_of_group.iter().filter(|(x, y)| grid[*x][*y].territory.is_ap == false).cloned().collect();
                let random_ap = territory_of_group.choose(rng).unwrap(); // FIX ME: sometimes this returns None
                grid[random_ap.0][random_ap.1].territory.is_ap = true;
                ap_count += 1;
             }
           //  println!("Number of ap in group after adding new ones: {}", ap_count);
            }

return true;
}

// function to check all groups territory and if there are no attraction points call place additional attraction points in territopry
pub fn check_attraction_points_in_territory(grid: &mut Vec<Vec<Cell>>, groups: &mut Vec<Groups>, num_points: usize, rng: &mut impl Rng) {
    let mut groups_to_delete: Vec<usize> = Vec::new();
    for group in groups.iter_mut() {
        let group_ap = get_attraction_points_in_territory(grid, group.group_id);
        if group_ap.is_empty() {
            if place_attraction_points_in_territory(grid, group.group_id, num_points, rng) == false {
                groups_to_delete.push(group.group_id);
            }
            if group_ap.is_empty() { 
                println!("ERROR: No attraction points in territory after trying to placing new ones");
            }
        }
    }
    //delete groups with no attraction points
    for group_id in groups_to_delete {
        groups.retain(|group| group.group_id != group_id);
        println!("Group {} deleted", group_id);
    }
}

// function to make a specifics groups core cell an ap called by core cell x and y

pub fn make_core_cell_an_ap(grid: &mut Vec<Vec<Cell>>, cx: usize, cy: usize) {
    grid[cx][cy].territory.is_ap = true;
    
}


pub fn dynamic_ap(grid: &mut Vec<Vec<Cell>>, groups: &mut Vec<Groups>, rng: &mut impl Rng, globals: &mut GlobalVariables) {
    
    //if globals.year > 2 && ((globals.day == 1 && globals.month == 7) || (globals.day == 1 && globals.month == 10)){
    if globals.year > 2 && ((globals.day == 1 ) || (globals.day == 10 ) || (globals.day == 19 )){ // FIX ME: changed to 3 times a month
       // println!("Dynamic AP placement");
       //log::info!("Dynamic AP placement");
       let mut groups_to_delete: Vec<usize> = Vec::new();
        for group in groups.iter_mut() {
        
            if globals.year > 2 && globals.month == 7 && globals.day == 1 {
                remove_non_core_attraction_points_this_group(grid, group.group_id);
                remove_non_core_attraction_points_this_group(grid, group.group_id);
                if place_attraction_points_in_territory(grid, group.group_id, 6, rng) == false {
                    groups_to_delete.push(group.group_id);
                }

                group.current_ap = get_attraction_points_in_territory(grid, group.group_id);
            } else if globals.year > 2 && globals.month == 10 && globals.day == 1  {
                remove_non_core_attraction_points_this_group(grid, group.group_id);
                remove_non_core_attraction_points_this_group(grid, group.group_id);
                if place_attraction_points_in_territory(grid, group.group_id, 3, rng) == false {
                    groups_to_delete.push(group.group_id);
                }
                group.current_ap = get_attraction_points_in_territory(grid, group.group_id);
            }

        }
        //delete groups with no attraction points
        for group_id in groups_to_delete {
            groups.retain(|group| group.group_id != group_id);
            println!("Group {} deleted", group_id);
        }
        //println!("Dynamic AP placement done");
    }
}

pub fn remove_non_core_attraction_points_this_group(grid: &mut Vec<Vec<Cell>>, group_id: usize) {
    
    let terr: Vec<(usize, usize)> = grid
    .iter()
    .enumerate()
    .flat_map(|(i, row)| row.iter().enumerate().filter(|&(_, cell)| cell.territory.taken_by_group == group_id).map(move |(j, _)| (i, j)))
    .collect();
    
    for (i, j) in terr {
        if grid[i][j].territory.core_cell_of_group != group_id {
            grid[i][j].territory.is_ap = false;
        }
    }
    
    
    
    
  //  for row in grid.iter_mut() {
  //      for cell in row.iter_mut() {
  //          if cell.territory.taken_by_group == group_id && cell.territory.core_cell_of_group != group_id {
  //              cell.territory.is_ap = false;
  //          }
  //      }
  //  }
}

