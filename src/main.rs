use rand::{rngs, Rng};
use rand::seq::SliceRandom;
use rand_distr::num_traits::int;
use core::time;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader, Error, ErrorKind, Result, Read,};
use std::path::Path;
use std::collections::VecDeque;
use std::time::Instant;
use serde::{de, Deserialize, Serialize};
use chrono::Datelike;
use chrono::Timelike;
use std::fs;
use chrono::Local;
use zip::{ZipWriter, write::FileOptions, CompressionMethod};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;


//use lazy_static::lazy_static;
//use std::sync::Mutex;
//use std::f64::consts::PI;

use std::fmt;

// Loading grid from ascii
mod grid_functions;
use grid_functions::*;
 
// Save to csv functions
mod save_functions;
use save_functions::*;

// Some individual related functions
mod ageing;
use ageing::{ageing};

mod reproduction;
use reproduction::*;

mod homerange;
use homerange::*;

mod group_functions;
use group_functions::*;

mod dispersal;
use dispersal::*;

mod mortality;
use mortality::*;

mod roamers;
use roamers::*;

mod interaction_layer;
use interaction_layer::*;

mod utility;
use utility::*;

mod pathogen;
use pathogen::*;

mod movement;
use movement::*;

mod carcass;
use carcass::*;
//type InteractionLayer = HashMap<(usize, usize, usize), InteractionCell>;



  // Register Ctrl+C handler


// Define a struct to represent a group
#[derive(Debug, Clone)]
pub struct Model {
    pub groups: Vec<Groups>,
    pub grid: Vec<Vec<Cell>>,
    pub global_variables: GlobalVariables,
    pub roamers: Vec<RoamingIndividual>,
    pub dispersers: Vec<DispersingFemaleGroup>,
    //pub interaction_layer: Vec<Vec<InteractionCell>>,
    pub interaction_layer: InteractionLayer,
    pub carcasses: Vec<Carcass>,
}

#[derive(Debug, Clone)]
pub struct Groups {
    group_id: usize,
    x: usize,
    y: usize,
    core_cell: Option<(usize, usize)>,
    target_cell: Option<(usize, usize)>,
    remaining_stay_time: usize,
    memory: GroupMemory,
    group_members: Vec<GroupMember>,
    movement: MovementMode,
    daily_movement_distance: usize,
    max_size: usize,
    current_ap: Vec<(usize, usize)>,
    active: bool,
}

// implementation of the group struct
impl Groups {
   

    // Function to set a target cell
    fn set_target_cell(&mut self, target_cell: (usize, usize)) {
        self.target_cell = Some(target_cell);
    }

    // Function to update the remaining stay time
    fn update_remaining_stay_time(&mut self) {
        if self.remaining_stay_time > 0 {
            self.remaining_stay_time -= 1;
        }
    }

    // Function to add a group member
    pub fn add_group_member(&mut self, member_info: GroupMember) {
        self.group_members.push(member_info);
    }

    // Method to get a reference to a specific group member
    pub fn get_group_member(&self, index: usize) -> Option<&GroupMember> {
        self.group_members.get(index)
    }

    // Method to perform logic on each group member
    pub fn process_group_members(&self) {
        for member in &self.group_members {
            // Perform logic on each group member
            // Example: println!("{:?}", member);
        }
    }

    // Method to get the distance to the target cell
    pub fn distance_to_target (&self) -> i32 {
        (self.x as i32 - self.target_cell.unwrap().0 as i32).abs() + (self.y as i32 - self.target_cell.unwrap().1 as i32).abs() // manhattan distance
    }

    // Method to create a new initial group member
   pub fn create_new_initial_group_member(&mut self) -> Result<GroupMember> {
    let mut rng = rand::thread_rng();
    let rand: f64 = rng.gen_range(0.0..1.0);

    let var_value = 0; // FIX me add age blur variance

    let individual_id = generate_individual_id(); 

    let tmp_age = match rand { 
         r if r <= 0.38 => 52 + var_value,
         r if r <= 0.62 => 104 + var_value,
         r if r <= 0.77 => 156 + var_value,
         r if r <= 0.86 => 208 + var_value,
         r if r <= 0.92 => 260 + var_value,
         r if r <= 0.95 => 312 + var_value,
         r if r <= 0.97 => 364 + var_value,
         r if r <= 0.98 => 416 + var_value,
         r if r <= 0.99 => 468 + var_value,
         _ => 520 + var_value,
    }; // from Kramer-Schadt et al. 2009 - age in weeks

    let age = tmp_age * 7; // age in days

    //set age class according to age in weeks (CITE)
    let age_class = if tmp_age <= 21 {
        AgeClass::Piglet
    } else if tmp_age <= 104 {
        AgeClass::Yearling
    } else {
        AgeClass::Adult
    };

    // randomly assign sex to the individual 50:50 ratio
    let sex = if rand::thread_rng().gen_bool(0.5) {
        Sex::Female
    } else {
        Sex::Male
    };

    let health_status = HealthStatus::Susceptible;
    let time_of_birth = 0;
    let has_reproduced = false;
    let time_of_reproduction = 0;
    let origin_group_id = self.group_id;
    let has_dispersed = false;
    let current_group_id = self.group_id;

    // Create a new group member
    let new_member = GroupMember {
        individual_id,
        age,
        age_class,
        sex,
        health_status,
        time_of_birth,
        has_reproduced,
        time_of_reproduction,
        origin_group_id,
        has_dispersed,
        current_group_id,
    };

    // Add the new group member to the group
    self.group_members.push(new_member.clone());
    Ok(new_member)
}

    pub fn expand_territory(&mut self, grid: &mut Vec<Vec<Cell>>) {
        let mut territory_cells = HashSet::new();
        if let Some((x, y)) = self.core_cell {
            // Iterate over neighboring cells
            for dx in -1..=1 {
                for dy in -1..=1 {
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
                            territory_cells.insert((new_x, new_y));
                        }
                    }
                }
            }
        }
        // If territory needs expansion, claim unoccupied cells around the core cell
        for (x, y) in territory_cells {
            grid[x][y].territory.is_taken = true;
            grid[x][y].territory.taken_by_group = self.group_id;
        }
    }

    pub fn adapt_territory(&mut self, grid: &mut Vec<Vec<Cell>>) {
        // Check if group size exceeds maximum size
            if self.group_members.len() > self.max_size {
            // Call territory expansion function
            self.expand_territory(grid);
         }
        }
    
    pub fn claim_territory(&mut self, grid: &mut Vec<Vec<Cell>>) {
        if let Some((core_x, core_y)) = self.core_cell {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let new_x = core_x as isize + dx;
                    let new_y = core_y as isize + dy;
                    if new_x >= 0
                        && new_x < grid.len() as isize
                        && new_y >= 0
                        && new_y < grid[0].len() as isize
                    {
                        let new_x = new_x as usize;
                        let new_y = new_y as usize;
                        grid[new_x][new_y].territory.is_taken = true;
                        grid[new_x][new_y].territory.taken_by_group = self.group_id;
                    }
                }
            }
        }
    }

    pub fn expand_territory_with_natural_shape(&mut self, grid: &mut Vec<Vec<Cell>>) {
        // Constants for desired number of cells and shape
       // let min_desired_cells = 1000;
        let max_desired_cells = 1600;
        let shape_factor = 0.5; // Adjust shape factor for desired shape
    
        let mut territory_cells = HashSet::new();
        if let Some((x, y)) = self.core_cell {
            // Start with the core cell
            territory_cells.insert((x, y));
    
            // Keep track of the number of iterations
            let mut iterations = 0;
    
            // Expand territory until desired number of cells is reached or max iterations exceeded
            while territory_cells.len() < max_desired_cells && iterations < 10000 {
                // Increment iterations count
                iterations += 1;
    
                // Clone the current set of territory cells
                let current_territory_cells = territory_cells.clone();
                // Iterate over the current territory cells
                for (x, y) in current_territory_cells {
                    // Iterate over neighboring cells
                    for dx in -1..=1 {
                        for dy in -1..=1 {
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
                                    }
                                }
                            }
                        }
                    }
                }
            }
    
            // Claim territory cells
            for (x, y) in territory_cells {
                grid[x][y].territory.is_taken = true;
                grid[x][y].territory.taken_by_group = self.group_id;
            }
        }
    }

    //pub fn dummy_expand_territory_with_natural_shape(&self, grid: &Vec<Vec<Cell>>) -> usize {
    //    // Constants for desired number of cells and shape
    //    //let min_desired_cells = 200;
    //    let max_desired_cells = 800;
    //    let shape_factor = 0.5; // Adjust shape factor for desired shape
    //
    //    let mut territory_cells = HashSet::new();
    //    if let Some((x, y)) = self.core_cell {
    //        // Start with the core cell
    //        territory_cells.insert((x, y));
    //
    //        // Keep track of the number of cells claimed
    //        let mut claimed_cells = 1;
    //
    //        // Expand territory until desired number of cells is reached or max iterations exceeded
    //        while claimed_cells < max_desired_cells {
    //            // Clone the current set of territory cells
    //            let current_territory_cells = territory_cells.clone();
    //            // Iterate over the current territory cells
    //            for (x, y) in current_territory_cells {
    //                // Iterate over neighboring cells
    //                for dx in -1..=1 {
    //                    for dy in -1..=1 {
    //                        if dx == 0 && dy == 0 {
    //                            continue;
    //                        }
    //                        let new_x = x as isize + dx;
    //                        let new_y = y as isize + dy;
    //                        // Check if the neighboring cell is within grid bounds
    //                        if new_x >= 0
    //                            && new_x < grid.len() as isize
    //                            && new_y >= 0
    //                            && new_y < grid[0].len() as isize
    //                        {
    //                            let new_x = new_x as usize;
    //                            let new_y = new_y as usize;
    //                            // Check if the cell is unoccupied and has positive quality
    //                            if !grid[new_x][new_y].territory.is_taken && grid[new_x][new_y].quality > 0.0 {
    //                                // Calculate distance from core cell
    //                                let distance = ((new_x as f64 - x as f64).powi(2) + (new_y as f64 - y as f64).powi(2)).sqrt();
    //                                // Bias selection based on distance for circular shape
    //                                let random_value = rand::random::<f64>();
    //                                if random_value < 1.0 / (1.0 + shape_factor * distance) {
    //                                    territory_cells.insert((new_x, new_y));
    //                                    claimed_cells += 1;
    //                                }
    //                            }
    //                        }
    //                    }
    //                }
    //            }
    //        }
    //    }
    //
    //    // Return the number of cells claimed
    //    territory_cells.len()
    //}
    
    pub fn expand_territory_with_natural_shape_and_radius(&mut self, grid: &mut Vec<Vec<Cell>>) {
    // Constants for desired number of cells, shape, and radius
    let min_desired_cells = 1000;
    let max_desired_cells = 1600;
    let shape_factor = 0.5; // Adjust shape factor for desired shape
    let radius = 50;

    let mut territory_cells = HashSet::new();
    if let Some((x, y)) = self.core_cell {
        // Start with the core cell
        territory_cells.insert((x, y));

        // Check if there are enough cells left in the grid within the radius
        let mut cells_left = 0;
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                // Check if the neighboring cell is within the specified radius
                let distance_squared = (new_x - x as isize).pow(2) + (new_y - y as isize).pow(2);
                if distance_squared <= radius * radius {
                    // Check if the neighboring cell is within grid bounds
                    if new_x >= 0
                        && new_x < grid.len() as isize
                        && new_y >= 0
                        && new_y < grid[0].len() as isize
                    {
                        let new_x = new_x as usize;
                        let new_y = new_y as usize;
                        // Check if the cell is unoccupied and has positive quality
                        if grid[new_x][new_y].quality > 0.0 && !grid[new_x][new_y].territory.is_taken {
                            cells_left += 1;
                        }
                    }
                }
            }
        }

        if cells_left < min_desired_cells {
            println!("Not enough cells left in the grid within the radius to create territory for group {}!", self.group_id);
            return; // Abort expansion process
        }

        // Expand territory until desired number of cells is reached
        while territory_cells.len() < min_desired_cells || territory_cells.len() > max_desired_cells {
            // Clone the current set of territory cells
            let current_territory_cells = territory_cells.clone();
            // Iterate over the current territory cells
            for (x, y) in current_territory_cells {
                // Iterate over neighboring cells within the radius
                for dx in -radius..=radius {
                    for dy in -radius..=radius {
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;
                        // Check if the neighboring cell is within the specified radius
                        let distance_squared = (new_x - x as isize).pow(2) + (new_y - y as isize).pow(2);
                        if distance_squared <= radius * radius {
                            // Check if the neighboring cell is within grid bounds
                            if new_x >= 0
                                && new_x < grid.len() as isize
                                && new_y >= 0
                                && new_y < grid[0].len() as isize
                            {
                                let new_x = new_x as usize;
                                let new_y = new_y as usize;
                                // Check if the cell is unoccupied and has positive quality
                                if grid[new_x][new_y].quality > 0.0 && !grid[new_x][new_y].territory.is_taken {
                                    // Bias selection based on distance for circular shape
                                    let random_value = rand::random::<f64>();
                                    if random_value < 1.0 / (1.0 + shape_factor * distance_squared as f64) {
                                        territory_cells.insert((new_x, new_y));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Only keep cells within the desired range
    territory_cells = territory_cells.into_iter().take(max_desired_cells).collect();

    // Claim territory cells
    for (x, y) in territory_cells {
        grid[x][y].territory.is_taken = true;
        grid[x][y].territory.taken_by_group = self.group_id;
    }
}

//function to return true if a specific group members health status is infected
//pub fn is_infected(group: &Groups, member_id: usize) -> bool {
//    let member = group.group_members.iter().find(|&member| member.individual_id == member_id).//unwrap();
//    member.health_status == HealthStatus::Infected
//
//}

    pub fn infected_member_present(&self)->bool{
        let mut infected = false;
        for member in &self.group_members {
            if member.health_status == HealthStatus::Infected {
                infected = true;
                break;
            }
        }
        infected
    }
}



static mut INDIVIDUAL_COUNTER: usize = 0;

// Function to generate a unique individual_id
fn generate_individual_id() -> usize {
    unsafe {
        INDIVIDUAL_COUNTER += 1;
        INDIVIDUAL_COUNTER
    }
}

// Static counter for group_id
static mut GROUP_COUNTER: usize = 0;

// Function to generate a unique group_id
fn generate_group_id() -> usize {
    unsafe {
        GROUP_COUNTER += 1;
        GROUP_COUNTER
    }
}

// Define a struct to represent an individual
#[derive(Debug, Clone)]
pub struct GroupMember {
    individual_id: usize,
    age: u32,
    age_class: AgeClass,
    sex: Sex,
    health_status: HealthStatus, 
    time_of_birth: usize,
    has_reproduced: bool,
    time_of_reproduction: usize,
    origin_group_id: usize,
    has_dispersed: bool,
    current_group_id: usize,
}

// Define a struct to represent a groups's memory
#[derive(Debug, Clone)]
struct GroupMemory {
    known_cells: HashSet<(usize, usize)>,
    known_cells_order: Vec<(usize, usize)>,
    //last_visited_cells: HashSet<(usize, usize)>,
    //last_visited_cells_order: Vec<(usize, usize)>,
    group_member_ids: Vec<usize>,
    presence_timer: usize,
}

// Define a struct to represent an individual's health status
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Susceptible,
    Infected,
    Immune,
}

// Implement the Display trait for HealthStatus
impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Susceptible => write!(f, "Susceptible"),
            HealthStatus::Infected => write!(f, "Infected"),
            HealthStatus::Immune => write!(f, "Immune"),
        }
    }
}

// Define a struct to represent an individual's sex
#[derive(Debug, Clone, PartialEq)]
pub enum Sex {
    Male,
    Female,
}

// Implement the Display trait for Sex
impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sex::Male => write!(f, "male"),
            Sex::Female => write!(f, "female"),
        }
    }
}

// Define a enum to represent an individual's age class
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum AgeClass {
    Piglet,
    Yearling,
    Adult,
}

// Implement the Display trait for AgeClass
impl fmt::Display for AgeClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgeClass::Piglet => write!(f, "Piglet"),
            AgeClass::Yearling => write!(f, "Yearling"),
            AgeClass::Adult => write!(f, "Adult"),
        }
    }
}

// Define a struct to represent the survival probability
pub struct SurvivalProbability{
    adult: f64,
    piglet: f64,
}

// Define a struct to represent a grid cell
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    quality: f64,
    counter: usize,
    x_grid: usize,
    y_grid: usize,
    territory: AreaSeparation
}

// Define a struct to represent the area separation
#[derive(Debug, Clone, PartialEq)]
pub struct AreaSeparation {
    is_ap: bool,
    is_taken:bool,
    taken_by_group: usize,
    core_cell_of_group: usize,
}

// Define a struct to represent the cell information
pub struct CellInfo {
    x_grid_o: usize,
    y_grid_o: usize,
    quality: f64,
}

// Define a struct to represent global variables
#[derive(Debug, Clone)]
pub struct GlobalVariables {
    age_mortality: u32,
    random_mortality: u32,
    overcapacity_mortality: u32,
    n_individuals: usize,
    day: u32,
    month: u32,
    year: u32,
    n_groups: usize,
    n_roamers: usize,
    n_dispersers: usize,
    good_year: bool,
    current_time: usize,
}

// Define a struct to represent the landscape
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

// Define a struct to represent the output
#[derive(Debug, Serialize, Deserialize)]
struct Output {
    iteration: usize,
    id: usize,
    group_id: usize,
    x: usize,
    y: usize,
    sex: String,
    age: u32,
    age_class: String,
    known_cells: Vec<(usize, usize)>,
    target_cell: Option<(usize, usize)>,
    core_cell: Option<(usize, usize)>,
    movement_type: String,
    remaining_stay_time: usize,
}

// Define a struct to represent the input
#[derive(Debug, Serialize, Deserialize)]
struct Input {
max_age: u32,
presence_time_limit: usize,
move_chance_percentage: usize,
max_known_cells: usize,
runtime: usize,
adult_survival: f64,
piglet_survival: f64,
adult_survival_day: f64,
piglet_survival_day: f64,
min_stay_time:usize,
max_stay_time:usize,
default_daily_movement_distance:usize,

}

//lazy_static! { //Default values
//    static ref MAX_AGE: Mutex<u32> = Mutex::new(365 * 12);
//    static ref PRESENCE_TIME_LIMIT: Mutex<usize> = Mutex::new(5);
//    static ref PRESENCE_TIME_LIMIT1: Mutex<usize> = Mutex::new(5);
//    static ref MOVE_CHANCE_PERCENTAGE: Mutex<usize> = Mutex::new(5);
//    static ref MAX_KNOWN_CELLS: Mutex<usize> = Mutex::new(20);
//    static ref MAX_LAST_VISITED_CELLS: Mutex<usize> = Mutex::new(3);
//    static ref RUNTIME: Mutex<usize> = Mutex::new(365);
//    static ref ADULT_SURVIVAL: Mutex<f64> = Mutex::new(0.65);
//    static ref PIGLET_SURVIVAL: Mutex<f64> = Mutex::new(0.5);
//    static ref ADULT_SURVIVAL_DAY: Mutex<f64> = Mutex::new(0.9647);
//    static ref PIGLET_SURVIVAL_DAY: Mutex<f64> = Mutex::new(0.9438);
//    static ref MIN_STAY_TIME: Mutex<usize> = Mutex::new(1);
//    static ref MAX_STAY_TIME: Mutex<usize> = Mutex::new(14);
//    static ref DEFAULT_DAILY_MOVEMENT_DISTANCE: Mutex<usize> = Mutex::new(20);
//}

//fn assign_to_constants(input_struct: &Input) {
//    // Assign values to constants
//    *MAX_AGE.lock().unwrap()                         = input_struct.max_age;
//    *PRESENCE_TIME_LIMIT.lock().unwrap()             = input_struct.presence_time_limit;
//    *MOVE_CHANCE_PERCENTAGE.lock().unwrap()          = input_struct.move_chance_percentage;
//    *MAX_KNOWN_CELLS.lock().unwrap()                 = input_struct.max_known_cells;
//    *RUNTIME.lock().unwrap()                         = input_struct.runtime;
//    *ADULT_SURVIVAL.lock().unwrap()                  = input_struct.adult_survival;
//    *PIGLET_SURVIVAL.lock().unwrap()                 = input_struct.piglet_survival;
//    *ADULT_SURVIVAL_DAY.lock().unwrap()              = input_struct.adult_survival_day;
//    *PIGLET_SURVIVAL_DAY.lock().unwrap()             = input_struct.piglet_survival_day;
//    *MIN_STAY_TIME.lock().unwrap()                   = input_struct.min_stay_time;
//    *MAX_STAY_TIME.lock().unwrap()                   = input_struct.max_stay_time;
//    *DEFAULT_DAILY_MOVEMENT_DISTANCE.lock().unwrap() = input_struct.default_daily_movement_distance;
//
//}

// consants
const MAX_AGE: u32 = 365 * 12;
const PRESENCE_TIME_LIMIT: usize = 5;
const MOVE_CHANCE_PERCENTAGE: usize = 5;
const MAX_KNOWN_CELLS: usize = 60; // DEBUG FIX ME with actual values
const MAX_LAST_VISITED_CELLS: usize = 3;
const RUNTIME: usize = 365 * 2; 
const ADULT_SURVIVAL: f64 = 0.65;
const PIGLET_SURVIVAL: f64 = 0.5;
const ADULT_SURVIVAL_DAY: f64 =  0.9647;
const PIGLET_SURVIVAL_DAY: f64 = 0.9438;
const MIN_STAY_TIME: usize = 1;
const MAX_STAY_TIME: usize = 14;
const DEFAULT_DAILY_MOVEMENT_DISTANCE: usize = 20;
const GODD_YEAR_CHANCE: usize = 15; // 15% chance of a good year



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


// Movement functions

pub fn calculate_quality_score(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> f64 {
    // println!("Calculating quality score for ({}, {})", x, y);  
    // logic to calculate the quality score based on the cell's attributes
    // placeholder FIXME
    match &grid[x][y] {
        Cell { quality, .. } => *quality,
        
    }
}

pub fn find_closest_adjacent_cell_to_target(group: &Groups) -> Option<(usize, usize)> {
    // Define the adjacent cells
    let adjacent_cells = vec![
        (group.x.saturating_sub(1), group.y),
        (group.x.saturating_add(1), group.y),
        (group.x, group.y.saturating_sub(1)),
        (group.x, group.y.saturating_add(1)),
    ];

    // Calculate the distance to the target for each adjacent cell
    let distances: Vec<_> = adjacent_cells
        .iter()
        .map(|&(x, y)| (x, y, group.distance_to_target()))
        .collect();

    // Find the adjacent cell with the minimum distance
    distances
        .into_iter()
        .min_by_key(|&(_, _, distance)| distance)
        .map(|(x, y, _)| (x, y))
}


pub fn random_cell(grid_size: usize, rng: &mut impl Rng) -> (usize, usize) {
    let x = rng.gen_range(0..grid_size);
    let y = rng.gen_range(0..grid_size);
    (x, y)
}

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

pub fn update_counter(globals: &mut GlobalVariables , groups: &mut Vec<Groups>, disperser_vector: &Vec<DispersingIndividual>, roamers: &Vec<RoamingIndividual>) {

    //let nd = disperser_vector.len();
    //let ng: usize = groups.iter().map(|group| group.group_members.len()).//sum();
    //*n_groups = nd + ng;
    // count number of groups, dispersers groups, roamers and total individuals
    //let mut tmp_n_groups = glob;
    //let mut tmp_n_dispersers = 0;
    //let mut tmp_n_roamers = 0;
    //let mut tmp_n_total = 0;

    //for group in groups.iter() {
    //    globals.n_groups += 1;
    //    globals.n_individuals = group.group_members.len();
    //}
//
    //globals.n_dispersers = disperser_vector.len();
    //globals.n_individuals += disperser_vector.len();
    //
    //globals.n_roamers = roamers.len();
    //globals.n_individuals += roamers.len();
    
    // count all individuals, roamers, diserpersing individuals and group members
    globals.n_individuals = groups.iter().map(|group| group.group_members.len()).sum::<usize>() + disperser_vector.len() + roamers.len();

    // count all roamers
    globals.n_roamers = roamers.len();

    // count all dispersing individuals
    globals.n_dispersers = disperser_vector.len();

    // count all groups
    globals.n_groups = groups.len();

    
    
    
   

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

pub fn setup(file_path: &str, num_groups: usize) -> (Vec<Vec<Cell>>, Vec<Groups>) {
    // Setup the landscape (grid)
    let (mut grid, metadata) = match landscape_setup_from_ascii(file_path) {
        Ok((g, m)) => (g, m),
        Err(e) => {
            eprintln!("Error setting up landscape: {}", e);
            // Handle the error, maybe return a default grid or exit the program
            std::process::exit(1);
        }
    };

    place_attraction_points(&mut grid, 3,6,1600);

    //let SurvivalProbability {adult = ADULT_SURVIVAL, piglet = PIGLET_SURVIVAL}
    //let mut survival_prob: Vec<SurvivalProbability> = Vec::new();
   

    //extract cell info
    let cell_info_list = extract_cell_info(&grid);

    // save_cellinfo_as_csv("output/debugCellInfo.csv",&cell_info_list);

    // Flip the grid for northing
    //flip_grid(&mut grid);

    // Setup the individuals
    let mut groups = group_setup(&cell_info_list, &mut grid, num_groups);

      // Check if any individual is outside the bounds
      if groups.iter().any(|ind| ind.x >= grid.len() || ind.y >= grid[0].len()) {
        println!("Some individuals are outside the bounds of the grid.");
    }

    fill_initial_groups(&mut groups, &grid);

    remove_non_core_attraction_points(&mut grid);

    //let terr = get_group_territory(&mut grid, &mut groups);
    //println!("Territory size: {}", terr.len());
    //place_additional_attraction_points(&mut grid, &mut groups, 5);

    (grid, groups)
}

// Main model

fn main() {

    //purge old log file if it exists and was not saved
    let _ = std::fs::remove_file("logs/outputLog.log");
    
    // check the logs folder, if there is 10 ore more files in there zip them and move them to the archive folder
   // let log_folder = Path::new("logs");
   // let archive_folder = Path::new("logs/archive");
   // let log_files = fs::read_dir(log_folder).unwrap();
   // let mut log_files_count = 0;
   // let mut has_archived = false;
   // for _ in log_files {
   //     log_files_count += 1;
   // }
//
   // if log_files_count >= 10 {
   //     has_archived = true;
   //     let now = Local::now();
   //     let zip_name = format!("log_archive_{}_{}_{}_{}_{}.zip", now.year(), now.month(), now.day(), now.hour(), now.minute());
   //     let zip_path = archive_folder.join(zip_name);
   //     let mut zip = ZipWriter::new(fs::File::create(zip_path).unwrap());
   //     let options = FileOptions::default().compression_method(CompressionMethod::Stored);
   //     let log_files = fs::read_dir(log_folder).unwrap();
   //     for file in log_files {
   //         let file = file.unwrap();
   //         let path = file.path();
   //         let file_name = path.file_name().unwrap().to_str().unwrap();
   //         zip.start_file(file_name, options).unwrap();
   //         let mut file = fs::File::open(path).unwrap();
   //         io::copy(&mut file, &mut zip).unwrap();
   //     }
   // }

    // pause execution for 30 second to allow the zip file to be created
    //thread::sleep(Duration::from_secs(30));

     // Initialize the logger
     log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    
    // Define grid dimensions
    //let grid_size = 25;

    //assign_to_constants(&Input);
    
    //let mut input_json = String::new();
    //io::stdin().read_to_string(&mut input_json).expect("Failed to read from stdin");
//
    //// Deserialize JSON into input structure
    //let input: Input = serde_json::from_str(&input_json).expect("Failed to deserialize JSON");
//
    //assign_to_constants(&input);

   

    // variable that is set to the system time when the simulation starts
    
  
    let start_time = Instant::now();
    //if has_archived {
    //    log::info!("Archived log files");
    //}

    log::info!("--------------------------->>> Starting simulation at time: {:?}", start_time);

    let mut rng = rand::thread_rng();
    let num_groups = 25; // FIX ME DEBUG CHANGE TO 1

    let file_path = "input/landscape/redDeer_global_50m.asc";
   //let file_path = "input/landscape/test.asc";
   // let file_path = "input/landscape/wb_50x50_prob_pred_s18.asc";

    // Setup the landscape and individuals

    let (mut grid, mut groups) = setup(file_path, num_groups); 

    // adjust attraction points
    log::info!("Adjusting attraction points");
    place_additional_attraction_points(&mut grid, &mut groups, 3, &mut rng);

    //place_dynamic_attraction_points(&mut grid, &mut groups, 10, &mut rng, "winter");
    log::info!("Removing attraction points with quality 0");
    remove_ap_on_cells_with_quality_0(&mut grid);
    
    
    //create vector for dispersing individuals using the struct in dispersal.rs
    let disperser_vector: &mut Vec<DispersingIndividual> = &mut Vec::new();
    let dispersing_groups_vector: &mut Vec<DispersingFemaleGroup> = &mut Vec::new();

    //create vector for roaming individuals using the struct in roamers.rs
    let roamer_vector: &mut Vec<RoamingIndividual> = &mut Vec::new();

    //create a vector for the carcasses
    let carcass_vector: &mut Vec<Carcass> = &mut Vec::new();

    
   // place_new_attraction_points(&mut grid, &mut groups, 5);

    // Vector to store grid states for all iterations
    let mut all_grid_states: Vec<(usize, Vec<Vec<Cell>>)> = Vec::new();

    // Vector to store individual states for all iterations
    let mut all_group_states: Vec<(usize, Vec<Groups>)> = Vec::new();

    // Vector to store disperser states for all iterations
    let mut all_disperser_states: Vec<(usize, Vec<DispersingFemaleGroup>)> = Vec::new();

    // Vector to store roamer states for all iterations
    let mut all_roamer_states: Vec<(usize, Vec<RoamingIndividual>)> = Vec::new();

    // Vector to store global variables for all iterations
    let mut all_global_variables: Vec<GlobalVariables> = Vec::new();

    // Vector to store interaction layer for all iterations
    let mut all_interaction_layers: Vec<(usize, InteractionLayer)> = Vec::new();

    // Vector to store carcass states for all iterations
    let mut all_carcass_states: Vec<(usize, Vec<Carcass>)> = Vec::new();

    
       let global_variables = GlobalVariables {
        age_mortality: 0,
        random_mortality: 0,
        overcapacity_mortality: 0,
        n_individuals: groups.iter().map(|group| group.group_members.len()).sum(),
        day: 1,   // Initialize with 1
        month: 1, // Initialize with 1
        year: 1,  // Initialize with 1
        n_groups: 0,
        n_dispersers: 0,
        n_roamers: 0,
        good_year: false,
        current_time: 0,
        // Add more variables as needed here
    };

    //let interaction_layer_tmp = create_interaction_layer();
    //let interaction_layer_tmp = 0;
    
     // Create an instance of InteractionLayer
     let interaction_layer_tmp = InteractionLayer::new();

     // create the model
     let mut model = Model {
        grid: grid,
        groups: groups,
        dispersers: dispersing_groups_vector.clone(),
        roamers: roamer_vector.clone(),
        global_variables: global_variables,
        interaction_layer: interaction_layer_tmp,
        carcasses: carcass_vector.clone(),
    };
    

    // Allocate survival probabilities
    let survival_prob = SurvivalProbability {
        adult: ADULT_SURVIVAL_DAY,
        piglet: PIGLET_SURVIVAL_DAY,
    };
    
   // place_attraction_points(&mut grid, 3,6,1600);

    //Debug print:
    println!("Setup complete -> starting iteration");

    // Simulate and save the grid state and individual state for each iteration
    for iteration in 1..= RUNTIME {

        if model.global_variables.day == 1 && model.global_variables.month == 1 {
            good_year_check(&mut model, &mut rng); // check if it is a good year
            roamer_density_dependent_removal(&mut model); //roamers leave the area i.e. are removed when there are more males then females
        }
        

        check_for_empty_groups(&mut model.groups);
        check_and_remove_empty_dispersal_groups(dispersing_groups_vector);
        log::info!("Checking for empty groups: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        free_cells_of_empty_groups(&model.groups, &mut model.grid);
        log::info!("Freeing cells of empty groups and deleting group: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        delete_groups_without_members(&mut model.groups);
        check_for_empty_groups(&mut model.groups);

        
       // if iteration > 25{
       //     log::info!("Checking for empty groups: year {}, month {}, day {}, iteration {}", global_variables.year, global_variables.month, global_variables.day, iteration);
       //     handle_empty_groups(&mut groups, &mut grid);
       // }

        //dispersal
        if iteration > 100 {

            //DEBUG FIX me: those 2 functions are breaking the simulation
        //free_group_cells(&mut groups, &mut grid);
        //delete_groups_without_members(&mut groups);
            
        //println!("Dispersal triggered");
        if model.global_variables.day == 1 {
           // println!("Dispersal triggered: year {}, month {}, day {}", global_variables.year, global_variables.month, global_variables.day);
            log::info!("Dispersal triggered: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            dispersal_assignment(&mut model.groups, disperser_vector, &mut model.dispersers);
            //assign_dispersal_targets_individuals( disperser_vector, &groups);
            log::info!("Assigning dispersal targets to individuals: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            assign_dispersal_targets_groups(&mut model.dispersers, &mut model.groups, &mut model.grid, &mut rng);
            //assign male individuals as roamers
            log::info!("Assigning roamer targets to individuals: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            roamer_assignemnt(&mut model.roamers,&mut model.groups);
        }
       // move_female_disperser(disperser_vector, &mut grid, &mut groups);
       check_empty_disperser_group(dispersing_groups_vector);
            log::info!("Moving dispersers: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            check_and_remove_empty_dispersal_groups(dispersing_groups_vector);
            move_female_disperser_group(&mut model.dispersers, &mut model.grid, &mut model.groups, &mut rng, model.global_variables.month, &mut model.interaction_layer, iteration);

        }
        log::info!("Initial roamer target assignment: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        initial_roamer_dispersal_target(&mut model.roamers,  &mut model.grid, &mut rng);
        log::info!("Initial roamer movement: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        initial_roamer_dispersal_movement(&mut model.roamers, &mut model.grid, &mut model.groups, &mut rng, &mut model.interaction_layer, iteration);
        // Free territory of groups with no members
        if model.global_variables.day == 1 {
          //  free_group_cells(&mut groups, &mut grid);
          //  remove_ap_from_freed_cells(&mut grid);
        }

       
        delete_groups_without_members(&mut model.groups);

        // Simulate movement of individuals
        log::info!("AP dynamic: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        dynamic_ap(&mut model.grid, &mut model.groups, &mut rng, &mut model.global_variables);
        log::info!("Check AP of groups: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        check_attraction_points_in_territory(&mut model.grid, &mut model.groups, 3, &mut rng);
        log::info!("Roaming movement: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        execute_roaming(&mut model.roamers, &mut model.groups, &mut model.grid, &mut rng, &mut model.interaction_layer, iteration);
        log::info!("Group movement: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        delete_groups_without_members(&mut model.groups);
        move_groups(&model.grid, &mut model.groups, &mut rng, &mut model.interaction_layer, iteration);

        //check dispersers if their target cell == none


        if model.global_variables.day == 5 {
            //debug print REMOVE ME
            //print!("reproduction is triggered");
            log::info!("Reproduction triggered: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
          reproduction(model.global_variables.month, &mut model.groups, iteration, model.global_variables.good_year);  // Adjust num_new_individuals               //   <-----------------temp OFF
        }

        if model.global_variables.day == 15 {

         //mortality(&survival_prob, &mut groups, &mut global_variables.random_mortality);                    //   <-----------------temp OFF
         log::info!("Mortality triggered: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
           // combined_mortality(&survival_prob, &mut groups, &mut global_variables.random_mortality, &mut global_variables.overcapacity_mortality);
            execute_mortality(&mut model, &survival_prob)
        }

        //age individuals by one day
        log::info!("Ageing triggered: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        ageing(&mut model);                                         //   <-----------------temp OFF
        

        //Updating various counters such as number of individuals
        update_counter(&mut model.global_variables, &mut model.groups, &disperser_vector, &roamer_vector);

        // Update group memory
        //update_group_memory(&mut individuals); // turned off for speed

        // Update the interaction layer to remove single individual instances
        log::info!("Deleting single individual instances: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
       // delete_single_individual_instances(&mut model.interaction_layer);

       if iteration == (6000) {  // DEBUG TESTER REMOVE ME
        remove_half_of_all_groups(&mut model);
       }

        if iteration == (RUNTIME) {
            // Save the grid state for the current (last) iteration
            //println!("its happening");
            all_grid_states.push((iteration, model.grid.clone()));
            }
    
            // Save the individual state for the current iteration
            all_group_states.push((iteration, model.groups.clone()));

            // Save the disperser state for the current iteration
            all_disperser_states.push((iteration, model.dispersers.clone()));

            // Save the roamer state for the current iteration
            all_roamer_states.push((iteration, model.roamers.clone()));

            // save the interaction layer for the current iteration
            all_interaction_layers.push((iteration, model.interaction_layer.clone()));

            // Save the carcass state for the current iteration
            all_carcass_states.push((iteration, model.carcasses.clone()));

           // purge_interaction_layer( &mut model.interaction_layer);

        // Stop the sim when all individuals are dead

        if model.global_variables.n_individuals == 0 {
            println!("Simulation terminated: No individuals remaining.");
            println!("Simulation terminated at timeindex: {}", iteration);
            all_grid_states.push((iteration, model.grid.clone())); // update gridstates wen simulation finished
            break;
        }

         all_global_variables.push(GlobalVariables {
            age_mortality: model.global_variables.age_mortality,
            random_mortality: model.global_variables.random_mortality,
            overcapacity_mortality: model.global_variables.overcapacity_mortality,
            n_individuals: model.global_variables.n_individuals,
            day: model.global_variables.day,
            month: model.global_variables.month,
            year: model.global_variables.year,
            n_groups: model.global_variables.n_groups,
            n_dispersers: model.global_variables.n_dispersers,
            n_roamers: model.global_variables.n_roamers,
            good_year: model.global_variables.good_year,
            current_time: iteration,

        });


        // Debug print time

        //print!("Day:{}, Month:{}, Year:{}, Individuals:{}\n", global_variables.day, global_variables.month, global_variables.year, global_variables.n_individuals);
        if model.global_variables.month == 1 && model.global_variables.day == 1{
            let perc = (iteration as f64 / RUNTIME as f64 * 100.0).round();
            let elapsed_time = start_time.elapsed().as_secs();
        println!("Simulation {}% complete! - Elapsed time: {}s", perc, elapsed_time);
        }
        // Progress time 
        
        progress_time(&mut model.global_variables);

        model.interaction_layer.clear_interaction_layer(); // clear the interaction layer for the next iteration
 

    }
    println!("Simulation complete, saving output\n");

    //variable that is set to the system time when the simulation ends
    let end_time = Instant::now();
    //variable showing the difference between the start and end time
    let time_taken = end_time.duration_since(start_time);
    println!("Time taken to run simulation: {:?}", time_taken);
    log::info!("Time taken to run simulation: {:?}", time_taken);

    // Save all grid states to a single CSV file
    save_grid_as_csv("output/all_grid_states.csv", &all_grid_states).expect("Failed to save grid states as CSV");                //   <-----------------temp OFF

    // Save all individual states to a single CSV file
    save_groups_as_csv("output/all_groups.csv", &all_group_states).expect("Failed to save groups as CSV");

    // Save all global variables to a single CSV file
    save_global_variables_as_csv("output/all_global_variables.csv", &all_global_variables).expect("Failed to save global variables as CSV");

    save_disperser_group_as_csv("output/all_dispersers.csv", &all_disperser_states).expect("Failed to save disperser as CSV");

    save_roamers_as_csv("output/all_roamers.csv", &all_roamer_states).expect("Failed to save roamer as CSV");

    //save_interaction_layer_as_csv("output/all_interaction_layer.csv", &all_interaction_layers).expect("Failed to save interaction layer as CSV");

    save_carcasses_as_csv("output/all_carcasses.csv", &all_carcass_states).expect("Failed to save carcasses as CSV");

   // save_interaction_layer_as_bson("output/all_interaction_layer.bson", &all_interaction_layers).expect("Failed to save interaction layer as BSON");

    // variable that is set to the system time when the save is complete
    //let save_time = Local::now();
    let save_time = Instant::now();
    //variable showing the difference between the end time and the save time
    let time_taken_save = save_time.duration_since(end_time);
    println!("Time taken to save output: {:?}", time_taken_save);
    log::info!("Time taken to save output: {:?}", time_taken_save);
    

    //rename the log file to include date and time to the minute
    let now = Local::now();
    log::info!("--------------------------->>> Simulation complete at time: {:?}", now);
    let log_file = format!("logs/log_{}.log", now.format("%Y-%m-%d_%H-%M"));
    fs::rename("logs/outputLog.log", log_file).expect("Failed to rename log file");


    // check the logs folder, if there is 10 ore more files in there zip them and move them to the archive folder
    //let log_folder = Path::new("logs");
    //let archive_folder = Path::new("logs/archive");
    //let log_files = fs::read_dir(log_folder).unwrap();
    //let mut log_files_count = 0;
    //for _ in log_files {
    //    log_files_count += 1;
    //}
//
    // if log_files_count >= 10 {
    //    let now = Local::now();
    //    let zip_name = format!("log_archive_{}_{}_{}_{}_{}.zip", now.year(), now.month(), now.day(), now.hour(), now.minute());
    //    let zip_path = archive_folder.join(zip_name);
    //    let mut zip = ZipWriter::new(fs::File::create(zip_path).unwrap());
    //    let options = FileOptions::default().compression_method(CompressionMethod::Stored);
    //    let log_files = fs::read_dir(log_folder).unwrap();
    //    for file in log_files {
    //        let file = file.unwrap();
    //        let path = file.path();
    //        let file_name = path.file_name().unwrap().to_str().unwrap();
    //        zip.start_file(file_name, options).unwrap();
    //        let mut file = fs::File::open(path).unwrap();
    //        io::copy(&mut file, &mut zip).unwrap();
    //    }
    //}

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
