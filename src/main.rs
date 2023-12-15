
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader, Error, ErrorKind, Result};

use std::fmt;

// Loading grid from ascii
mod grid_functions;
use grid_functions::*;
 
// Save to csv functions
mod save_functions;
use save_functions::*;

// Some individual related functions
mod ageing;
use ageing::ageing;

mod reproduction;
use reproduction::*;


// Define a struct to represent an individual
#[derive(Debug, Clone)]
pub struct Individual {
    id: usize,
    group_id: usize,
    x: usize,
    y: usize,
    age: u32,
    sex: Sex,
    has_reproduced: bool,
    time_of_reproduction: usize,
    //core_cell:Option<(usize,usize)>,
    //target_cell:Option<(usize,usize)>,
    //remaining_stay_time: usize,
    age_class: AgeClass, 
    memory: IndividualMemory,
    // add reset for reproduction
}

//impl Individual {
//    // Function to set a core cell
//    fn set_core_cell(&mut self, core_cell: (usize, usize)) {
//        self.core_cell = Some(core_cell);
//    }
//
//    // Function to set a target cell
//    fn set_target_cell(&mut self, target_cell: (usize, usize)) {
//        self.target_cell = Some(target_cell);
//    }
//
//    // Function to update the remaining stay time
//    fn update_remaining_stay_time(&mut self) {
//        if self.remaining_stay_time > 0 {
//            self.remaining_stay_time -= 1;
//        }
//    }
//}

// Define a struct to represent an individual's memory
#[derive(Debug, Clone)]
struct IndividualMemory {
    known_cells: HashSet<(usize, usize)>,
    known_cells_order: Vec<(usize, usize)>,
    //last_visited_cells: HashSet<(usize, usize)>,
    //last_visited_cells_order: Vec<(usize, usize)>,
    group_member_ids: Vec<usize>,
    presence_timer: usize,
}

// Define a struct to represent an individual's sex
#[derive(Debug, Clone, PartialEq)]
enum Sex {
    Male,
    Female,
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sex::Male => write!(f, "male"),
            Sex::Female => write!(f, "Female"),
        }
    }
}

// Define a struct to represent an individual's age class
#[derive(Debug, Clone, PartialEq)]
pub enum AgeClass {
    Piglet,
    Yearling,
    Adult,
}

impl fmt::Display for AgeClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgeClass::Piglet => write!(f, "Piglet"),
            AgeClass::Yearling => write!(f, "Yearling"),
            AgeClass::Adult => write!(f, "Adult"),
        }
    }
}

pub struct SurvivalProbability{
    adult: f64,
    piglet: f64,
}

// Define a struct to represent a grid cell
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    quality: f64,
    counter: usize,
    x_grid: usize,
    y_grid: usize,
    is_ap: bool,
}

pub struct CellInfo {
    x_grid_o: usize,
    y_grid_o: usize,
    quality: f64,
}

// Define a struct to represent global variables
#[derive(Clone)] 
pub struct GlobalVariables {
    age_mortality: u32,
    random_mortality: u32,
    n_individuals: usize,
    day: u32,
    month: u32,
    year: u32,
}

// Landscape metadata i.e. ASCII header
#[derive(Debug)]
pub struct LandscapeMetadata {
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
const MOVE_CHANCE_PERCENTAGE: usize = 5;
const MAX_KNOWN_CELLS: usize = 20;
const MAX_LAST_VISITED_CELLS: usize = 3;
const RUNTIME: usize = 365 * 10;
const ADULT_SURVIVAL: f64 = 0.65; //annual
const PIGLET_SURVIVAL: f64 = 0.5; //annual
const ADULT_SURVIVAL_DAY: f64 =  0.9647;//daily //0.9647381; // monthly
const PIGLET_SURVIVAL_DAY: f64 = 0.9438;//daily //0.9438743;// monthly
// Individuals related functions

pub fn individuals_setup(cell_info_list: &Vec<CellInfo>, grid: &Vec<Vec<Cell>>, num_individuals: usize) -> Vec<Individual> {

    // Create individuals with unique IDs, group IDs, and memory
    let mut individuals: Vec<Individual> = Vec::with_capacity(num_individuals);
    let grid_size = grid.len();  

   // let tmp_Grid = grid.iter().iter().filter(|cell| cell.quality > 0.0);

    for id in 0..num_individuals {
      
        let (x, y) = loop {
            let x_candidate = rand::thread_rng().gen_range(0..grid_size);
            let y_candidate = rand::thread_rng().gen_range(0..grid_size);

            if grid[x_candidate][y_candidate].quality > 0.0 {
                break (x_candidate, y_candidate);
            }
        };

        let age = 730 + rand::thread_rng().gen_range(1..=1825);
        let group_id = rand::thread_rng().gen_range(1..=2);
        let presence_timer = 0;
        
        let sex;
        if rand::thread_rng().gen_bool(0.5) == true {
             sex = Sex::Female;
        }else{
             sex = Sex::Male;
        }

        let time_of_reproduction = 0;

        
       // if rand::thread_rng().gen_bool(0.5) == true { // random check male Female 50/50 if bool is true then Female else male
       //      sex = Sex { male: false, Female:true };
       // }else{
       //      sex = Sex { male: true, Female:false };
       // }

        let age_class = AgeClass::Adult;

        let has_reproduced = false;
        let memory = IndividualMemory {
            known_cells: HashSet::new(),
            group_member_ids: Vec::new(),
            //last_visited_cells: HashSet::new(),
            known_cells_order: Vec::new(),
            //last_visited_cells_order: Vec::new(),
            presence_timer,
        };

        


        individuals.push(Individual {
            id,
            group_id,
            x,
            y,
            age,
            sex,
            age_class,
            has_reproduced,
            time_of_reproduction,
            //core_cell,
            //target_cell,
            //remaining_stay_time,
            memory,
        });
    }

    individuals
}

// Mortality

fn mortality(surv_prob: &SurvivalProbability, individuals: &mut Vec<Individual>, random_mortality: &mut u32){

    let retained_individuals: Vec<Individual> = individuals
    .drain(..)
    .filter(|ind| {
       if ind.age_class != AgeClass::Piglet {

        let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0); // random floating point number
        let rounded_number = (random_number * 1e4).round() / 1e4; // rounded to 4 digits

        if rounded_number < surv_prob.adult 
         {true} else {
            *random_mortality += 1;
            false
        }
       }else{

        let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0); // random floating point number
        let rounded_number = (random_number * 1e4).round() / 1e4; // rounded to 4 digits

        if rounded_number < surv_prob.piglet
         {true} else {
            
            *random_mortality += 1;
            false
        }
       }
    })
    .collect();

    // Clear the original vector and insert retained individuals
    individuals.clear();
    individuals.extend_from_slice(&retained_individuals);
}


// Memory functions

//pub fn update_memory(memory: &mut HashSet<(usize, usize)>, order: &mut Vec<(usize, usize)>, new_cell: (usize, usize), max_size: usize) {
//    memory.insert(new_cell);
//
//    order.retain(|&cell| memory.contains(&cell)); // Remove cells that are not in the memory
//
//    if order.len() >= max_size {
//        let oldest_cell = order.remove(0); // Remove the oldest element
//        memory.remove(&oldest_cell);
//    }
//
//    order.push(new_cell);
//}

pub fn update_memory(memory: &mut HashSet<(usize, usize)>, order: &mut Vec<(usize, usize)>, new_cell: (usize, usize), max_size: usize) {
    memory.insert(new_cell);

    order.retain(|&cell| memory.contains(&cell)); // Remove cells that are not in the memory

    if order.len() >= max_size {
        let oldest_cell = order.remove(0); // Remove the oldest element
        memory.remove(&oldest_cell);
    }

    order.push(new_cell);
}

pub fn update_group_memory(individuals: &mut Vec<Individual>) {
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

// Movement functions

pub fn calculate_quality_score(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> f64 {
    // println!("Calculating quality score for ({}, {})", x, y);  
    // logic to calculate the quality score based on the cell's attributes
    // placeholder FIXME
    match &grid[x][y] {
        Cell { quality, .. } => *quality,
        // Add other cases as needed
    }
}

pub fn move_individuals<R: Rng>(grid: &Vec<Vec<Cell>>, individuals: &mut Vec<Individual>, rng: &mut R) {
    for individual in individuals.iter_mut() {
        // 25% chance to move randomly
        if rng.gen_range(0..100) < 25 {
            //move_to_random_adjacent_cells(grid.len(), individual, rng);
            move_to_random_adjacent_cells_2(grid, individual, rng);
        } else {
            // Move towards the cell with the highest quality
            //move_towards_highest_quality(grid, individual, rng);
            move_to_random_adjacent_cells_2(grid, individual, rng);

            // Update presence timer
            individual.memory.presence_timer += 1;

            // Check if presence time limit is reached or 5% chance to move
            if individual.memory.presence_timer >= PRESENCE_TIME_LIMIT || rng.gen_range(0..100) < MOVE_CHANCE_PERCENTAGE {
                // Reset presence timer and force movement to a random cell
                individual.memory.presence_timer = 0;
                move_to_random_adjacent_cells(grid.len(), individual, rng);
            }
        }
    }
}

pub fn move_towards_highest_quality(grid: &Vec<Vec<Cell>>, individual: &mut Individual, rng: &mut impl Rng) {
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
    //update_memory(&mut individual.memory.known_cells, &mut individual.memory.known_cells_order, (individual.x, individual.y), MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);

    update_memory(&mut individual.memory.known_cells, &mut individual.memory.known_cells_order, (individual.x, individual.y), MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);


    // Update individual's position
    individual.x = new_x;
    individual.y = new_y;
}

//TEST
pub fn move_to_random_adjacent_cells_2(grid: &Vec<Vec<Cell>>, individual: &mut Individual, rng: &mut impl Rng){
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


          // Select the first cell (randomized) with quality > 0
     let target_cell = adjacent_cells
     .into_iter()
     .filter(|&(x, y)| x < grid.len() && y < grid[0].len() && grid[x][y].quality > 0.0)
     .next()
     .unwrap_or_else(|| {
         // If no valid adjacent cells with quality > 0, move randomly within the grid
         // TEMP FIX ME 
         random_cell_with_quality(grid, rng)
     });

       // Update known cells and last visited cells
    //update_memory(&mut individual.memory.known_cells, &mut individual.memory.known_cells_order, target_cell, MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, target_cell, MAX_LAST_VISITED_CELLS);

    update_memory(&mut individual.memory.known_cells, &mut individual.memory.known_cells_order, (individual.x, individual.y), MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);

    // Update individual's position
    individual.x = target_cell.0;
    individual.y = target_cell.1;
}
   
//TEST
fn random_cell_with_quality(grid: &Vec<Vec<Cell>>, rng: &mut impl Rng) -> (usize, usize) {
    // Generate a random cell within the grid with quality > 0
    loop {
        let x = rng.gen_range(0..grid.len());
        let y = rng.gen_range(0..grid[0].len());
        if grid[x][y].quality > 0.0 {
            return (x, y);
        }
    }
}


pub fn move_to_random_adjacent_cells(grid_size: usize, individual: &mut Individual, rng: &mut impl Rng) {
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
    //update_memory(&mut individual.memory.known_cells, &mut individual.memory.known_cells_order, target_cell, MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, target_cell, MAX_LAST_VISITED_CELLS);

    update_memory(&mut individual.memory.known_cells, &mut individual.memory.known_cells_order, (individual.x, individual.y), MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);

    // Update individual's position
    individual.x = target_cell.0;
    individual.y = target_cell.1;
}

pub fn random_cell(grid_size: usize, rng: &mut impl Rng) -> (usize, usize) {
    let x = rng.gen_range(0..grid_size);
    let y = rng.gen_range(0..grid_size);
    (x, y)
}

// unused

pub fn random_known_cell(known_cells: &HashSet<(usize, usize)>, rng: &mut impl rand::Rng) -> Option<(usize, usize)> {
    let vec: Vec<&(usize, usize)> = known_cells.iter().collect();
    if let Some(&known_cell) = vec.choose(rng) {
        Some(*known_cell)
    } else {
        None
    }
}

pub fn random_cell_around(x: usize, y: usize, grid_size: usize, rng: &mut impl Rng) -> (usize, usize) {
    // Generate random offsets within a radius of 2 cells
    let dx = rng.gen_range(-2..=2);
    let dy = rng.gen_range(-2..=2);

    // Calculate the new coordinates, ensuring they stay within bounds
    let new_x = (x as isize + dx).clamp(0, grid_size as isize - 1) as usize;
    let new_y = (y as isize + dy).clamp(0, grid_size as isize - 1) as usize;

    (new_x, new_y)
}

pub fn random_cell_around_known(known_cells: &HashSet<(usize, usize)>, grid_size: usize, rng: &mut impl rand::Rng) -> Option<(usize, usize)> {
    let vec: Vec<&(usize, usize)> = known_cells.iter().collect();
    if let Some(&(x, y)) = vec.choose(rng) {
        Some(random_cell_around(*x, *y, grid_size, rng))
    } else {
        None
    }
}

pub fn random_known_cell_except_last_three(known_cells: &HashSet<(usize, usize)>,last_visited_cells: &HashSet<(usize, usize)>,rng: &mut impl Rng,) -> Option<(usize, usize)> {
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
 
// Update functions

pub fn update_counter(n_individuals: &mut usize,individuals: &mut Vec<Individual>){

    *n_individuals = individuals.len();
}
 
pub fn progress_time(global_variables: &mut GlobalVariables) {
    // Increment the day
    global_variables.day += 1;

    // Check if a month has passed (28 days in a month)
    if global_variables.day > 28 {
        global_variables.day = 1;
        global_variables.month += 1;

        // Check if a year has passed (12 months in a year)
        if global_variables.month > 12 {
            global_variables.month = 1;
            global_variables.year += 1;
        }
    }
}

// General setup

pub fn setup(file_path: &str, num_individuals: usize) -> (Vec<Vec<Cell>>, Vec<Individual>) {
    // Setup the landscape (grid)
    let (mut grid, metadata) = match landscape_setup_from_ascii(file_path) {
        Ok((g, m)) => (g, m),
        Err(e) => {
            eprintln!("Error setting up landscape: {}", e);
            // Handle the error, maybe return a default grid or exit the program
            std::process::exit(1);
        }
    };

    //let SurvivalProbability {adult = ADULT_SURVIVAL, piglet = PIGLET_SURVIVAL}
    //let mut survival_prob: Vec<SurvivalProbability> = Vec::new();
   

    //extract cell info
    let cell_info_list = extract_cell_info(&grid);

    // save_cellinfo_as_csv("output/debugCellInfo.csv",&cell_info_list);

    // Flip the grid for northing
    flip_grid(&mut grid);

    // Setup the individuals
    let individuals = individuals_setup(&cell_info_list, &grid, num_individuals);

      // Check if any individual is outside the bounds
      if individuals.iter().any(|ind| ind.x >= grid.len() || ind.y >= grid[0].len()) {
        println!("Some individuals are outside the bounds of the grid.");
    }

    (grid, individuals)
}

// Main model

fn main() {
    // Define grid dimensions
    //let grid_size = 25;
    let num_individuals = 1;

    let file_path = "input/landscape/redDeer_global_50m.asc";
   
    // Setup the landscape and individuals

    let (mut grid, mut individuals) = setup(file_path, num_individuals);

    // Vector to store grid states for all iterations
    let mut all_grid_states: Vec<(usize, Vec<Vec<Cell>>)> = Vec::new();

    // Vector to store individual states for all iterations
    let mut all_individuals_states: Vec<(usize, Vec<Individual>)> = Vec::new();

    // Vector to store global variables for all iterations
    let mut all_global_variables: Vec<GlobalVariables> = Vec::new();

       let mut global_variables = GlobalVariables {
        age_mortality: 0,
        random_mortality: 0,
        n_individuals: individuals.len(),
        day: 1,   // Initialize with 1
        month: 1, // Initialize with 1
        year: 1,  // Initialize with 1
        // Add more variables as needed here
    };

    // Allocate survival probabilities
    let survival_prob = SurvivalProbability {
        adult: ADULT_SURVIVAL_DAY,
        piglet: PIGLET_SURVIVAL_DAY,
    };

    place_attraction_points(&mut grid, 3,6,1600);

    //Debug print:
    println!("Setup complete -> starting iteration");

    // Simulate and save the grid state and individual state for each iteration
    for iteration in 1..= RUNTIME {

        // Simulate movement of individuals
        let mut rng = rand::thread_rng();
        move_individuals(&grid, &mut individuals, &mut rng);

        if global_variables.month == 5 {
            //debug print REMOVE ME
            //print!("reproduction is triggered");

         // reproduction(global_variables.month, &mut individuals, iteration);  // Adjust num_new_individuals               //   <-----------------temp OFF
        }

        if global_variables.day == 15 {

        //  mortality(&survival_prob, &mut individuals, &mut global_variables.random_mortality);                    //   <-----------------temp OFF
        }

        //age individuals by one day
        //ageing(&mut individuals, &mut global_variables.age_mortality);                                         //   <-----------------temp OFF

        //Updating various counters such as number of individuals
        update_counter(&mut global_variables.n_individuals, &mut individuals);

        // Update group memory
        //update_group_memory(&mut individuals); // turned off for speed

        if iteration == (RUNTIME) {
            // Save the grid state for the current (last) iteration
            //println!("its happening");
            all_grid_states.push((iteration, grid.clone()));
            }
    
            // Save the individual state for the current iteration
           all_individuals_states.push((iteration, individuals.clone()));

        // Stop the sim when all individuals are dead

        if global_variables.n_individuals == 0 {
            println!("Simulation terminated: No individuals remaining.");
            println!("Simulation terminated at timeindex: {}", iteration);
            all_grid_states.push((iteration, grid.clone())); // update gridstates wen simulation finished
            break;
        }

         all_global_variables.push(GlobalVariables {
            age_mortality: global_variables.age_mortality,
            random_mortality: global_variables.random_mortality,
            n_individuals: global_variables.n_individuals,
            day: global_variables.day,
            month: global_variables.month,
            year: global_variables.year,
        });


        // Debug print time

        //print!("Day:{}, Month:{}, Year:{}, Individuals:{}\n", global_variables.day, global_variables.month, global_variables.year, global_variables.n_individuals);
        if global_variables.month == 1 && global_variables.day == 1{
            let perc = (iteration as f64 / RUNTIME as f64 * 100.0).round();
        println!("Simulation {}% complete!", perc);
        }
        // Progress time 
        
        progress_time(&mut global_variables);
 

    }
    println!("Simulation complete, saving output\n");

    // Save all grid states to a single CSV file
    save_grid_as_csv("output/all_grid_states.csv", &all_grid_states).expect("Failed to save grid states as CSV");                //   <-----------------temp OFF

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