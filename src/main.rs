
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader, Error, ErrorKind, Read, Result};
use std::path::Path;

// Define a struct to represent an individual
#[derive(Debug, Clone)]
struct Individual {
    id: usize,
    group_id: usize,
    x: usize,
    y: usize,
    age: u32, 
    memory: IndividualMemory,
}

// Define a struct to represent an individual's memory
#[derive(Debug, Clone)]
struct IndividualMemory {
    known_cells: HashSet<(usize, usize)>,
    known_cells_order: Vec<(usize, usize)>,
    last_visited_cells: HashSet<(usize, usize)>,
    last_visited_cells_order: Vec<(usize, usize)>,
    group_member_ids: Vec<usize>,
    presence_timer: usize,
}

// Define a struct to represent a grid cell
#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    quality: f64,
    counter: usize,
}

// Define a struct to represent global variables
struct GlobalVariables {
    age_mortality: u32,
    n_individuals: usize,
}

// Landscape metadata i.e. ASCII header
#[derive(Debug)]
struct LandscapeMetadata {
    ncols: usize,
    nrows: usize,
    xllcorner: usize,
    yllcorner: usize,
    cellsize: f64,
    nodata_value: i32,
}

//Constants / inputs
const MAX_AGE: u32 = 365 * 12;
const PRESENCE_TIME_LIMIT: usize = 5;
const MOVE_CHANCE_PERCENTAGE: f64 = 5.0;
const MAX_KNOWN_CELLS: usize = 20;
const MAX_LAST_VISITED_CELLS: usize = 3;


fn landscape_setup_from_ascii(file_path: &str) -> io::Result<(Vec<Vec<Cell>>, LandscapeMetadata)> {
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

    // Initialize the grid with quality values from the ASCII file
    let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(nrows);

    // Read the rows
    for _ in 0..nrows {
        if let Some(Ok(line)) = lines.next() {
            let row: Vec<Cell> = line
                //.chars()
                //.map(|c| Cell {
                //    quality: c.to_digit(10).unwrap_or(0) as f64, // Convert ASCII char to digit
                //    counter: 0,
                .split_whitespace()
                .map(|s| Cell {
                    quality: s.parse().unwrap_or(0.0), // Assume it's a numeric, signed value
                    counter: 0,
                })
                .collect();

            grid.push(row);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data rows in the ASCII file",
            ));
        }
    }

    Ok((grid, metadata))
}

fn extract_metadata(reader: &mut BufReader<File>) -> Result<LandscapeMetadata> {
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

fn individuals_setup(grid_size: usize, num_individuals: usize) -> Vec<Individual> {
    // Create individuals with unique IDs, group IDs, and memory
    let mut individuals: Vec<Individual> = Vec::with_capacity(num_individuals);
    for id in 0..num_individuals {
        let x = rand::thread_rng().gen_range(0..grid_size);
        let y = rand::thread_rng().gen_range(0..grid_size);
        let age = 730 + rand::thread_rng().gen_range(1..=1825); // Renamed from capacity to age
        let group_id = rand::thread_rng().gen_range(1..=2); // Example: 2 groups
        let presence_timer = 0; 
        let memory = IndividualMemory {
            known_cells: HashSet::new(),
            group_member_ids: Vec::new(),
            last_visited_cells: HashSet::new(),
            known_cells_order: Vec::new(),  
            last_visited_cells_order: Vec::new(),  
            presence_timer,
        };
        individuals.push(Individual {
            id,
            group_id,
            x,
            y,
            age,
            memory,
        });
    }
    individuals
}

fn update_memory(memory: &mut HashSet<(usize, usize)>, order: &mut Vec<(usize, usize)>, new_cell: (usize, usize), max_size: usize) {
    memory.insert(new_cell);

    order.retain(|&cell| memory.contains(&cell)); // Remove cells that are not in the memory

    if order.len() >= max_size {
        let oldest_cell = order.remove(0); // Remove the oldest element
        memory.remove(&oldest_cell);
    }

    order.push(new_cell);
}

fn update_group_memory(individuals: &mut Vec<Individual>) {
    // Get the indices of individuals
    let indices: Vec<usize> = (0..individuals.len()).collect();

    // Iterate through indices to update group memory
    for &index in &indices {
        let group_id = individuals[index].group_id;

        // Find indices of group members with the same group_id
        let group_members_ids: Vec<usize> = indices
            .iter()
            .filter(|&&i| individuals[i].group_id == group_id)
            .map(|&i| individuals[i].id)
            .collect();

        // Update group memory with the IDs of group members
        individuals[index].memory.group_member_ids = group_members_ids;

        // Print debug information
        //println!(
        //    "Individual {}: Group ID: {}, Group members: {:?}",
        //    index, group_id, individuals[index].memory.group_member_ids
        //);
    }
}

fn ageing(individuals: &mut Vec<Individual>, age_mortality: &mut u32) {
    for individual in individuals.iter_mut() {
        individual.age += 1;
    }

    // Filter out individuals whose age exceeds the maximum age
    let retained_individuals: Vec<Individual> = individuals
        .drain(..)
        .filter(|individual| {
            if individual.age > MAX_AGE {
                // Increment age_mortality counter when an individual is removed
                *age_mortality += 1;
                false
            } else {
                true
            }
        })
        .collect();

    // Clear the original vector and insert retained individuals
    individuals.clear();
    individuals.extend_from_slice(&retained_individuals);
}

fn calculate_quality_score(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> f64 {
    // println!("Calculating quality score for ({}, {})", x, y);  
    // logic to calculate the quality score based on the cell's attributes
    // placeholder FIXME
    match &grid[x][y] {
        Cell { quality, .. } => *quality,
        // Add other cases as needed
    }
}

fn move_individuals<R: Rng>(grid: &Vec<Vec<Cell>>, individuals: &mut Vec<Individual>, rng: &mut R) {
    for individual in individuals.iter_mut() {
        // 25% chance to move randomly
        if rng.gen_range(0..100) < 25 {
            move_to_random_cell(grid.len(), individual, rng);
        } else {
            // Move towards the cell with the highest quality
            move_towards_highest_quality(grid, individual, rng);

            // Update presence timer
            individual.memory.presence_timer += 1;

            // Check if presence time limit is reached or 5% chance to move
            if individual.memory.presence_timer >= PRESENCE_TIME_LIMIT || rng.gen_range(0..100) < 5 {
                // Reset presence timer and force movement to a random cell
                individual.memory.presence_timer = 0;
                move_to_random_cell(grid.len(), individual, rng);
            }
        }
    }
}

fn move_towards_highest_quality(grid: &Vec<Vec<Cell>>, individual: &mut Individual, rng: &mut impl Rng) {
    // Generate a list of adjacent cells
    let adjacent_cells = vec![
        (individual.x.saturating_sub(1), individual.y),
        (individual.x.saturating_add(1), individual.y),
        (individual.x, individual.y.saturating_sub(1)),
        (individual.x, individual.y.saturating_add(1)),
    ];

    // Calculate the quality score for each adjacent cell and find the cell with the highest quality
    let (new_x, new_y) = adjacent_cells.iter()
    .filter(|&&(x, y)| x < grid.len() && y < grid[0].len())  // Check bounds
    .map(|&(x, y)| (x, y, calculate_quality_score(grid, x, y)))
    .max_by(|&(_, _, quality1), &(_, _, quality2)| quality1.partial_cmp(&quality2).unwrap_or(std::cmp::Ordering::Equal))
    .map(|(x, y, _)| (x, y))
    .unwrap_or_else(|| random_cell(grid.len(), rng));

   

    // Update known cells and last visited cells
    update_memory(&mut individual.memory.known_cells, &mut individual.memory.known_cells_order, (individual.x, individual.y), MAX_KNOWN_CELLS);
    update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);

    // Update individual's position
    individual.x = new_x;
    individual.y = new_y;
}

fn move_to_random_cell(grid_size: usize, individual: &mut Individual, rng: &mut impl Rng) {
    // Get the current position of the individual
    let current_x = individual.x;
    let current_y = individual.y;

    // Generate a list of adjacent cells
    let mut adjacent_cells = vec![
        (current_x.saturating_sub(1), current_y),
        (current_x.saturating_add(1), current_y),
        (current_x, current_y.saturating_sub(1)),
        (current_x, current_y.saturating_add(1)),
    ];

    // Shuffle the list of adjacent cells
    adjacent_cells.shuffle(rng);

    // Select the first cell (randomized)
    let target_cell = adjacent_cells
        .into_iter()
        .find(|&(x, y)| x < grid_size && y < grid_size)
        .unwrap_or_else(|| {
            // If no valid adjacent cells, move randomly within the grid
            random_cell(grid_size, rng)
        });

    // Update known cells and last visited cells
    update_memory(&mut individual.memory.known_cells, &mut individual.memory.known_cells_order, target_cell, MAX_KNOWN_CELLS);
    update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, target_cell, MAX_LAST_VISITED_CELLS);

    // Update individual's position
    individual.x = target_cell.0;
    individual.y = target_cell.1;
}

fn random_cell(grid_size: usize, rng: &mut impl Rng) -> (usize, usize) {
    let x = rng.gen_range(0..grid_size);
    let y = rng.gen_range(0..grid_size);
    (x, y)
}

// Currently unused helper function

fn random_known_cell(known_cells: &HashSet<(usize, usize)>, rng: &mut impl rand::Rng) -> Option<(usize, usize)> {
    let vec: Vec<&(usize, usize)> = known_cells.iter().collect();
    if let Some(&known_cell) = vec.choose(rng) {
        Some(*known_cell)
    } else {
        None
    }
}

fn random_cell_around(x: usize, y: usize, grid_size: usize, rng: &mut impl Rng) -> (usize, usize) {
    // Generate random offsets within a radius of 2 cells
    let dx = rng.gen_range(-2..=2);
    let dy = rng.gen_range(-2..=2);

    // Calculate the new coordinates, ensuring they stay within bounds
    let new_x = (x as isize + dx).clamp(0, grid_size as isize - 1) as usize;
    let new_y = (y as isize + dy).clamp(0, grid_size as isize - 1) as usize;

    (new_x, new_y)
}

fn random_cell_around_known(known_cells: &HashSet<(usize, usize)>, grid_size: usize, rng: &mut impl rand::Rng) -> Option<(usize, usize)> {
    let vec: Vec<&(usize, usize)> = known_cells.iter().collect();
    if let Some(&(x, y)) = vec.choose(rng) {
        Some(random_cell_around(*x, *y, grid_size, rng))
    } else {
        None
    }
}

fn random_known_cell_except_last_three(known_cells: &HashSet<(usize, usize)>,last_visited_cells: &HashSet<(usize, usize)>,rng: &mut impl Rng,) -> Option<(usize, usize)> {
    let available_cells: Vec<_> = known_cells
        .difference(last_visited_cells)
        .cloned()
        .collect();

    if let Some(&cell) = available_cells.choose(rng) {
        Some(cell)
    } else {
        None
    }
}

// End unused helper functions

fn save_individuals_as_csv(filename: &str, individuals_states: &[(usize, Vec<Individual>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,id,group_id,x,y,age,known_cells,group_member_ids, last_three_cells")?;

    // Write each individual's data for each iteration
    for (iteration, individuals) in individuals_states {
        for individual in individuals {
        // Convert variables to strings for CSV output
            let known_cells_str: String = individual
                .memory
                .known_cells
                .iter()
                .map(|&(x, y)| format!("[{}_{}]", x, y))
                .collect::<Vec<String>>()
                .join(";");
            
                let group_member_ids_str: String = format!(
                    "[{}]",
                    individual
                        .memory
                        .group_member_ids
                        .iter()
                        .map(|&id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(";")
                );
            
            let last_three_cells_str: String = individual
                .memory
                .last_visited_cells_order
                .iter()
                .map(|&(x, y)| format!("[{}_{}]", x, y))
                .collect::<Vec<String>>()
                .join(";");

            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{}",
                iteration,
                individual.id,
                individual.group_id,
                individual.x,
                individual.y,
                individual.age,
                known_cells_str,
                group_member_ids_str,
                last_three_cells_str
            )?;
    }
}


    println!("Individuals saved to: {}", filename);
    Ok(())
}

fn save_grid_as_csv(filename: &str, grid_states: &[(usize, Vec<Vec<Cell>>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,x,y,quality,counter")?;

    // Write each cell's data for each iteration
    for (iteration, grid) in grid_states {
        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                writeln!(file, "{},{},{},{},{}", iteration, x, y, cell.quality, cell.counter)?;
            }
        }
    }

    println!("Grid states saved to: {}", filename);
    Ok(())
}

fn save_global_variables_as_csv(filename: &str, global_variables: &[GlobalVariables]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,n_individuals,age_mortality")?;

    // Write each iteration's global variables
    for (iteration, globals) in global_variables.iter().enumerate() {
        writeln!(file, "{},{},{}", iteration + 1, globals.n_individuals, globals.age_mortality)?;
        // Add more variables as needed
    }

    println!("Global variables saved to: {}", filename);
    Ok(())
}

fn update_counter(n_individuals: &mut usize,individuals: &mut Vec<Individual>){

    *n_individuals = individuals.len();
}

fn setup(file_path: &str, num_individuals: usize) -> (Vec<Vec<Cell>>, Vec<Individual>, u32, usize) {
    // Setup the landscape (grid)
    let (grid, metadata) = match landscape_setup_from_ascii(file_path) {
        Ok((g, m)) => (g, m),
        Err(e) => {
            eprintln!("Error setting up landscape: {}", e);
            // Handle the error, maybe return a default grid or exit the program
            std::process::exit(1);
        }
    };
    // Setup the individuals
    let individuals = individuals_setup(grid.len(), num_individuals);

    // Initialize the age_mortality counter to 0
    let age_mortality = 0;

    // Initialize the age_mortality counter to the starting individual number num_individuals
    let n_individuals = individuals.len();

    (grid, individuals, age_mortality, n_individuals)
}

fn main() {
    // Define grid dimensions
    let grid_size = 25;
    let num_individuals = 10;

    let file_path = "input/landscape/redDeer_global_50m.asc";
   
    // Setup the landscape and individuals
    //let (mut grid, mut individuals, mut age_mortality, mut n_individuals) = setup(grid_size, num_individuals);
    //let (mut individuals, mut age_mortality, mut n_individuals) = setup(grid_size, num_individuals);

    let (grid, mut individuals,mut age_mortality,mut n_individuals) = setup(file_path, num_individuals);

    // Vector to store grid states for all iterations
    let mut all_grid_states: Vec<(usize, Vec<Vec<Cell>>)> = Vec::new();

    // Vector to store individual states for all iterations
    let mut all_individuals_states: Vec<(usize, Vec<Individual>)> = Vec::new();

    // Vector to store global variables for all iterations
    let mut all_global_variables: Vec<GlobalVariables> = Vec::new();

    // Simulate and save the grid state and individual state for each iteration
    for iteration in 1..= 1 {

        // Simulate movement of individuals
        let mut rng = rand::thread_rng();
        move_individuals(&grid, &mut individuals, &mut rng);

        // Save the grid state for the current (last) iteration
        all_grid_states.push((iteration, grid.clone()));

        // Save the individual state for the current iteration
        all_individuals_states.push((iteration, individuals.clone()));

        //age individuals by one day
        ageing(&mut individuals, &mut age_mortality);

        //Updating various counters such as number of individuals
        update_counter(&mut n_individuals, &mut individuals);

        // Update group memory
        update_group_memory(&mut individuals);


        // Save global variables for the current iteration
        all_global_variables.push(GlobalVariables {
        age_mortality,
        n_individuals
        // Add more variables as needed here
        });

        // Stop the sim when all individuals are dead

        if n_individuals == 0 {
            println!("Simulation terminated: No individuals remaining.");
            break;
        }

    }

    // Save all grid states to a single CSV file
    save_grid_as_csv("output/all_grid_states.csv", &all_grid_states).expect("Failed to save grid states as CSV");

    // Save all individual states to a single CSV file
    save_individuals_as_csv("output/all_individuals.csv", &all_individuals_states).expect("Failed to save individuals as CSV");

    // Save all global variables to a single CSV file
    save_global_variables_as_csv("output/all_global_variables.csv", &all_global_variables).expect("Failed to save global variables as CSV");


}



//save as image

//fn save_grid_as_image(iteration: usize, grid: &mut [Vec<Cell>], individuals: &[Individual]) {
//    let width = grid.len();
//    let height = grid[0].len();
//
//    // Create an RGB image with white background
//    let mut image = RgbImage::new(width as u32, height as u32);
//    for pixel in image.pixels_mut() {
//        *pixel = Rgb([255, 255, 255]);
//    }
//
//    // Set individual positions on the image and update counters
//    for individual in individuals {
//        let x = individual.x as u32;
//        let y = individual.y as u32;
//        let color = Rgb([0, 0, 255]); // Blue color for individuals
//
//        image.put_pixel(x, y, color);
//
//        // Increment counter for the cell
//        grid[individual.x][individual.y].counter += 1;
//    }
//
//    // Save the image to a file with a name corresponding to the iteration and counter
//    let filename = format!("grid_image_iter{}_counter{}.png", iteration, grid[0][0].counter);
//
//    // Remove existing file if it exists
//    if fs::metadata(&filename).is_ok() {
//        fs::remove_file(&filename).expect("Failed to remove existing file");
//    }
//
//    // Save the new image
//    image.save(&filename).expect("Failed to save image");
//}

//save as multiple csv

//fn save_grid_as_csv(iteration: usize, grid: &[Vec<Cell>]) -> io::Result<()> {
//    // Create or open the CSV file
//    let filename = format!("grid_state_iter{}.csv", iteration);
//    let mut file = File::create(&filename)?;
//
//    // Write the header line
//    writeln!(file, "x,y,quality,counter")?;
//
//    // Write each cell's data
//    for (x, row) in grid.iter().enumerate() {
//       for (y, cell) in row.iter().enumerate() {
//           writeln!(file, "{},{},{},{}", x, y, cell.quality, cell.counter)?;
//       }
//   }
//
//    println!("Grid state saved to: {}", filename);
//    Ok(())
//}



