
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader, Error, ErrorKind, Result};
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

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


// Define a struct to represent a group
#[derive(Debug, Clone)]
pub struct Groups {
    //id: usize,
    group_id: usize,
    x: usize,
    y: usize,
   // age: u32,
   // sex: Sex,
   // has_reproduced: bool,
    time_of_reproduction: usize,
    core_cell:Option<(usize,usize)>,
    target_cell:Option<(usize,usize)>,
    remaining_stay_time: usize,
   // age_class: AgeClass, 
    memory: GroupMemory,
    group_members: Vec<GroupMember>,
    // add reset for reproduction
    movement: MovementMode,
    daily_movement_distance: usize,
}

impl Groups {
    // Function to set a core cell
    fn set_core_cell(&mut self, core_cell: (usize, usize)) {
        self.core_cell = Some(core_cell);
    }

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

    pub fn distance_to_target (&self) -> i32 {
        (self.x as i32 - self.target_cell.unwrap().0 as i32).abs() + (self.y as i32 - self.target_cell.unwrap().1 as i32).abs() // manhattan distance
    }




   pub fn create_new_initial_group_member(&mut self) -> Result<GroupMember> {
    let mut rng = rand::thread_rng();
    let rand: f64 = rng.gen_range(0.0..1.0);

    let var_value = 0; // FIX me add age blur variance

    let individual_id = generate_individual_id(); // 1; // FIX ME rolling ID

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

    //set age class according to age in weeks (SwifCoIBMove)
    let age_class = if tmp_age <= 21 {
        AgeClass::Piglet
    } else if tmp_age <= 104 {
        AgeClass::Yearling
    } else {
        AgeClass::Adult
    };

    let sex = if rand::thread_rng().gen_bool(0.5) {
        Sex::Female
    } else {
        Sex::Male
    };

    let health_status = HealthStatus::Susceptible;
    let time_of_birth = 0;
    let  has_reproduced = false;
    let  time_of_reproduction = 0;

    let new_member = GroupMember {
        individual_id,
        age,
        age_class,
        sex,
        health_status,
        time_of_birth,
        has_reproduced,
        time_of_reproduction,
    };

    self.group_members.push(new_member.clone());
    Ok(new_member)
}

}

#[derive(Debug, Clone, PartialEq)]
pub enum MovementMode{
    ApTransition,
    Foraging,
    NotSet,
}

impl fmt::Display for MovementMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MovementMode::ApTransition => write!(f, "apTransition"),
            MovementMode::Foraging => write!(f, "foraging"),
            MovementMode::NotSet => write!(f, "not set"),
        }
    }
}


// Static counter for individual_id
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

// Function to generate a unique individual_id
fn generate_group_id() -> usize {
    unsafe {
        GROUP_COUNTER += 1;
        GROUP_COUNTER
    }
}

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
enum HealthStatus {
    Susceptible,
    Infected,
    Immune,
}

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
enum Sex {
    Male,
    Female,
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sex::Male => write!(f, "male"),
            Sex::Female => write!(f, "female"),
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
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    quality: f64,
    counter: usize,
    x_grid: usize,
    y_grid: usize,
    territory: AreaSeparation
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaSeparation {
    is_ap: bool,
    is_taken:bool,
    taken_by_group: usize,
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


struct Input {

max_age: u32,
resence_time_limit: usize,
//move_chance_percentage: usize,
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





//const MAX_AGE: u32 = 365 * 12;
//const PRESENCE_TIME_LIMIT: usize = 5;
//const MOVE_CHANCE_PERCENTAGE: usize = 5;
//const MAX_KNOWN_CELLS: usize = 20;
//const MAX_LAST_VISITED_CELLS: usize = 3;
//const RUNTIME: usize = 365;
//const ADULT_SURVIVAL: f64 = 0.65;
//const PIGLET_SURVIVAL: f64 = 0.5;
//const ADULT_SURVIVAL_DAY: f64 =  0.9647;
//const PIGLET_SURVIVAL_DAY: f64 = 0.9438;
//const MIN_STAY_TIME: usize = 1;
//const MAX_STAY_TIME: usize = 14;
//const DEFAULT_DAILY_MOVEMENT_DISTANCE: usize = 20;

fn assign_to_constants(input_struct: &Input) {
    // Assign values to constants

    //static MAX_AGE: u32 = input_struct.max_age;
    static PRESENCE_TIME_LIMIT1: usize = 5;
    static MOVE_CHANCE_PERCENTAGE: usize = 5;
    static MAX_KNOWN_CELLS: usize = 20;
    static MAX_LAST_VISITED_CELLS: usize = 3;
    static RUNTIME: usize = 365;
    static ADULT_SURVIVAL: f64 = 0.65;
    static PIGLET_SURVIVAL: f64 = 0.5;
    static ADULT_SURVIVAL_DAY: f64 =  0.9647;
    static PIGLET_SURVIVAL_DAY: f64 = 0.9438;
    static MIN_STAY_TIME: usize = 1;
    static MAX_STAY_TIME: usize = 14;
    static DEFAULT_DAILY_MOVEMENT_DISTANCE: usize = 20;

    // Print the assigned values for control
    //println!("Constant 1: {}", CONSTANT1);
    //println!("Constant 2: {}", CONSTANT2);
    //println!("Constant 3: {}", CONSTANT3);
}


//Constants / inputs
const MAX_AGE: u32 = 365 * 12;
const PRESENCE_TIME_LIMIT: usize = 5;
const MOVE_CHANCE_PERCENTAGE: usize = 5;
const MAX_KNOWN_CELLS: usize = 20;
const MAX_LAST_VISITED_CELLS: usize = 3;
const RUNTIME: usize = 365;// 365 * 10; //<---------------------------FIX ME DEBUG TO 1 year
const ADULT_SURVIVAL: f64 = 0.65; //annual
const PIGLET_SURVIVAL: f64 = 0.5; //annual
const ADULT_SURVIVAL_DAY: f64 =  0.9647;//daily //0.9647381; // monthly
const PIGLET_SURVIVAL_DAY: f64 = 0.9438;//daily //0.9438743;// monthly
const MIN_STAY_TIME: usize = 1; // days //<---------------------------------------------FIX ME randomly selected
const MAX_STAY_TIME: usize = 14; // days
const DEFAULT_DAILY_MOVEMENT_DISTANCE: usize = 20;
// Individuals related functions

// Function to perform circular BFS from the core cell

fn circular_bfs(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize, group_id: usize, desired_total_cells: usize) {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    queue.push_back((x, y));
    visited[x][y] = true;

    let mut count = 0;

    while let Some((cx, cy)) = queue.pop_front() {
        occupy_this_cell(&mut grid[cx][cy], group_id);
        count += 1;

        if count >= desired_total_cells {
            break;
        }

        // Explore neighbors in a circular fashion
        let radius = 5.0;                  
        let mut angle = 0.0;           

        while angle <= 2.0 * std::f64::consts::PI {
            let nx = (cx as f64 + (radius * angle.cos()).round()) as usize;
            let ny = (cy as f64 + (radius * angle.sin()).round()) as usize;


            if nx < grid.len() && ny < grid[0].len() && !visited[nx][ny] {
                if grid[nx][ny].quality > 0.0 && !grid[nx][ny].territory.is_taken { // changed quality check
                    queue.push_back((nx, ny));
                    visited[nx][ny] = true;
                }
            }

            angle += std::f64::consts::PI / 180.0; //12.0;      
        }
    }
}

pub fn group_setup(cell_info_list: &Vec<CellInfo>,  grid: &mut Vec<Vec<Cell>>, num_groups: usize) -> Vec<Groups> {

    // Create individuals with unique IDs, group IDs, and memory
    let mut group: Vec<Groups> = Vec::with_capacity(num_groups);
    let grid_size = grid.len();  

   // let tmp_Grid = grid.iter().iter().filter(|cell| cell.quality > 0.0);

    for group_id in 0..num_groups {

        // Select an free attraction point as territory coe cell
        let free_ap = get_free_attraction_points(&grid);
        if free_ap.is_empty(){

        println!("No more free space for additional groups, group creation halted at {}/{} groups!", group_id,num_groups);
        break;
        }else{
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..free_ap.len());
        let random_ap = free_ap[random_index];
        
        let x = random_ap.0;
        let y = random_ap.1;

        let group_id = generate_group_id(); // id; //rand::thread_rng().gen_range(1..=2);

        // Make this cell the core cell / the core Ap
        occupy_this_cell(&mut grid[x][y], group_id);
        //occupy_this_cell(&mut grid, x, y, group_id);
        let core_cell = (x, y);

        // Take the surrounding cells as territory
        // Total Cells=Width×Height
        //

        let desired_total_cells = 1600;
       // let range = ((desired_total_cells as f64).sqrt() - 1.0) / 2.0;

       // for i in (x.saturating_sub(range as usize))..=(x + range as usize) {
       //     for j in (y.saturating_sub(range as usize))..=(y + range as usize) {
       //         if i < grid.len() && j < grid[0].len() {
       //             if grid[i][j].quality > 0.0 && grid[i][j].territory.is_taken == false {
//
       //                // occupy_this_cell(&mut grid, x, y, group_id);
       //                 occupy_this_cell(&mut grid[i][j], group_id);
//
       //             }
       //         }
       //     }
       // }

       circular_bfs(grid, x, y, group_id, desired_total_cells);

        //if my_vector.is_empty() {
        //    println!("The vector is empty!");
        //} else {
        //    println!("The vector is not empty!");
        //}

    // Random position
       // let (x, y) = loop {
       //     let x_candidate = rand::thread_rng().gen_range(0..grid_size);
       //     let y_candidate = rand::thread_rng().gen_range(0..grid_size);

       //     if grid[x_candidate][y_candidate].quality > 0.0 {
       //         break (x_candidate, y_candidate);
       //     }
       // };

        //let age = 730 + rand::thread_rng().gen_range(1..=1825);
        
        let presence_timer = 0;
        
        //let sex;
        //if rand::thread_rng().gen_bool(0.5) == true {
        //     sex = Sex::Female;
        //}else{
        //     sex = Sex::Male;
        //}

        let time_of_reproduction = 0;

       // let age_class = AgeClass::Adult;

       // let has_reproduced = false;
        let memory = GroupMemory {
            known_cells: HashSet::new(),
            group_member_ids: Vec::new(),
            //last_visited_cells: HashSet::new(),
            known_cells_order: Vec::new(),
            //last_visited_cells_order: Vec::new(),
            presence_timer,
        };

        //let core_cell = None;
        let target_cell = None;
        let remaining_stay_time = 0;
        let movement = MovementMode::Foraging;
        let group_members = vec![];
        let daily_movement_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE; //<--------------------DEBUG FIX ME with actual values
        

        group.push(Groups {
            group_id,
            x,
            y,
           //age,
           //sex,
           //age_class,
           //has_reproduced,
            time_of_reproduction,
            core_cell: Some(core_cell),
            target_cell,
            remaining_stay_time,
            memory,
            movement,
            group_members,
            daily_movement_distance,
        });

     }
    }

    group
}



// use the mean habitat suitability of the surrounding cells (territory) to calculate number of breeding females and create groups accordingly
//fn fill_initial_groups(group: &mut Vec<Groups> , grid: &Vec<Vec<Cell>>){
//
//
// for group in group.iter_mut() {
//
//    let breed_cap = calculate_mean_quality_for_group(grid, group.group_id).round();
//
//    // group size estimator from SwiCoIBMove 4.5
//
//    let tmp_size = (4.5 * breed_cap - 1.0).round() as u32; 
//
//    for i in 0..tmp_size {
//
//        group.create_new_initial_group_member();
//
//    }
//
//
// }
//}

fn fill_initial_groups(groups: &mut Vec<Groups>, grid: &Vec<Vec<Cell>>) {
    for group in groups.iter_mut() {
        let breed_cap = calculate_mean_quality_for_group(grid, group.group_id).round();

        // group size estimator from SwiCoIBMove 4.5
        let tmp_size = (4.5 * breed_cap - 1.0).round() as u32;
        //println!("grpsize {}", tmp_size); FIX ME  DEBUG PRINT

        for _ in 0..tmp_size {
            group.create_new_initial_group_member();
        }
    }
}


fn calculate_mean_quality_for_group(grid: &Vec<Vec<Cell>>, group_id: usize) -> f64 {
    let mut total_quality = 0.0;
    let mut num_cells = 0;

    for row in grid {
        for cell in row {
            if cell.territory.is_taken && cell.territory.taken_by_group == group_id {
                total_quality += cell.quality * 10.0; //FIX ME adjust for the raster
                num_cells += 1;
            }
        }
    }

    if num_cells == 0 {
        return 0.0; // Avoid division by zero
    }

    //println!("qual: {}, ncell: {}", total_quality, num_cells );

    total_quality / (num_cells as f64)
}


//EXPERMINETS

// Function to choose a core cell (within 1600 cells)
fn choose_core_cell(grid: &Vec<Vec<Cell>>, group: &Groups, rng: &mut impl Rng) -> (usize, usize) {
    // Get the position of the individual as the core cell
    (group.x, group.y)
}

//// Function to choose a target cell (90% within 1600 cells, 10% within 3200 cells)
//fn choose_target_cell(grid: &Vec<Vec<Cell>>, individual: &Individual, rng: &mut impl Rng) -> (usize, usize) {
//    let core_cell = individual.core_cell.unwrap_or((0, 0));
//    if rng.gen_bool(0.9) {
//        // Choose a target cell within 1600 cells
//        let target_cell = find_cell_within_range(grid, core_cell, 1600, rng);
//        target_cell.unwrap_or_else(|| random_cell_within_range(grid.len(), grid[0].len(), core_cell, 1600, rng))
//    } else {
//        // Choose a target cell within 3200 cells (avoiding other individuals)
//        let target_cell = find_cell_within_range(grid, core_cell, 3200, rng);
//        target_cell.unwrap_or_else(|| random_cell_within_range(grid.len(), grid[0].len(), core_cell, 3200, rng))
//    }
//}
//
//// Function to find a random cell within a specified range (avoiding other individuals)
//fn find_cell_within_range(grid: &Vec<Vec<Cell>>, center_cell: (usize, usize), range: usize, rng: &mut impl Rng) -> Option<(usize, usize)> {

//    random_cell_within_range(grid.len(), grid[0].len(), center_cell, range, rng)
//}
//
//// Function to find a random cell within a specified range (excluding the center cell)
//fn random_cell_within_range(
//    grid_size_x: usize,
//    grid_size_y: usize,
//    center_cell: (usize, usize),
//    range: usize,
//    rng: &mut impl Rng,
//) -> (usize, usize) {

//    random_cell(grid_size_x, grid_size_y)
//}
//
//
////

// Mortality

//fn mortality(surv_prob: &SurvivalProbability, group: &mut Vec<Groups>, random_mortality: &mut u32){
//
//    let retained_groups: Vec<Groups> = group
//    .drain(..)
//    .filter(|ind| {
//       if ind.age_class != AgeClass::Piglet {
//
//        let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0); // random floating point number
//        let rounded_number = (random_number * 1e4).round() / 1e4; // rounded to 4 digits
//
//        if rounded_number < surv_prob.adult 
//         {true} else {
//            *random_mortality += 1;
//            false
//        }
//       }else{
//
//        let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0); // random floating point number
//        let rounded_number = (random_number * 1e4).round() / 1e4; // rounded to 4 digits
//
//        if rounded_number < surv_prob.piglet
//         {true} else {
//            
//            *random_mortality += 1;
//            false
//        }
//       }
//    })
//    .collect();
//
//    // Clear the original vector and insert retained individuals
//    group.clear();
//    group.extend_from_slice(&retained_groups);
//}

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

pub fn update_group_memory(group: &mut Vec<Groups>) {
    // Get the indices of individuals
   // let indices: Vec<usize> = (0..group.len()).collect();
//
   // // Iterate through indices to update group memory
   // for &index in &indices {
   //     let group_id = group[index].group_id;
//
   //     // Find indices of group members with the same group_id
   //     let group_members_ids: Vec<usize> = indices
   //         .iter()
   //         .filter(|&&i| group[i].group_id == group_id)
   //         .map(|&i| group[i].id)
   //         .collect();
//
   //     // Update group memory with the IDs of group members
   //     group[index].memory.group_member_ids = group_members_ids;
//
   //     // Print debug information
   //     //println!(
   //     //    "Individual {}: Group ID: {}, Group members: {:?}",
   //     //    index, group_id, individuals[index].memory.group_member_ids
   //     //);
   // }
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

pub fn move_to_closest_adjacent_cell_to_target(grid: &Vec<Vec<Cell>>, group: &mut Groups) {
    // Find the closest adjacent cell to the target
    if let Some((new_x, new_y)) = find_closest_adjacent_cell_to_target(group) {
        // Update known cells
        update_memory(&mut group.memory.known_cells, &mut group.memory.known_cells_order, (group.x, group.y), MAX_KNOWN_CELLS);

        // Update individual's position
        group.x = new_x;
        group.y = new_y;
    }
}



pub fn move_individuals<R: Rng>(grid: &Vec<Vec<Cell>>, group: &mut Vec<Groups>, rng: &mut R) {
    for group in group.iter_mut() {

        //println!("Movement called"); //<------ DEBUG print

        let mut realign_time = 3; //number of steps before realigning towards the target

        while group.daily_movement_distance > 0  {

            

            //check if a target cell is needed and assign a stay time for the ap
            if group.target_cell.is_none() {
                let territory_ap = get_attraction_points_in_territory(grid, group.group_id);
                let new_target_cell = territory_ap
                    .choose(rng)
                    .cloned()
                    .expect("No attraction points in territory");
            
                group.set_target_cell(new_target_cell);
                group.remaining_stay_time = rng.gen_range(MIN_STAY_TIME..MAX_STAY_TIME);
            }

            // Steps
            // 25% chance to move randomly
            if rng.gen_range(0..100) < 25 { // <-----------------------------------------------DEBUG FIX ME percentage
                //move_to_random_adjacent_cells(grid.len(), individual, rng);
                move_to_random_adjacent_cells_2(grid, group, rng);
                group.daily_movement_distance -= 1;
            } else {
                // Move towards the cell with the highest quality
               // move_towards_highest_quality(grid, group, rng);
               //move_within_territory(grid, group, rng);

               // move_to_random_adjacent_cells_2(grid, group, rng);
                if group.movement == MovementMode::ApTransition {

                    if group.x == group.target_cell.unwrap().0 && group.y == group.target_cell.unwrap().1 {
                        group.movement = MovementMode::Foraging;

                        break; // if target location reached flit to foraging
                    }
                    
                   // if realign_time > 0 { // every 3rd step we realign to the target
                   // correlated_random_walk_towards_target(grid, group, rng);
                   // realign_time -= 1;
                   // }
                   // if realign_time == 0 {
                   //     move_to_closest_adjacent_cell_to_target(grid, group);
                   //     realign_time = 3;
                   // }

                  // move_to_closest_adjacent_cell_to_target(grid, group);

                  //move_towards_target_cell(group);

                  //move_one_step_towards_target_cell(group);
                  move_one_step_towards_target_cell_with_random(group,rng,grid);

                    group.daily_movement_distance -= 1;

                    if group.distance_to_target() <= 3 {

                        group.movement = MovementMode::Foraging;
                        //print!("Engage forage mode"); // DEBUG

                    }

                } else if group.movement == MovementMode::Foraging {
                    
                    if group.remaining_stay_time <= 0 { //if stay time around ap is used up get a new ap to move towards

                        let territory_ap = get_attraction_points_in_territory(grid, group.group_id);
                        let closest_ap = get_closest_attraction_point(group, &territory_ap);
                        let other_aps: Vec<(usize, usize)> = territory_ap
                            .into_iter()
                            .filter(|&ap| ap != closest_ap)
                            .collect();

                        // Choose a random target cell from the remaining attraction points
                        let new_target_cell = other_aps
                            .choose(rng)
                            .cloned()
                            .expect("No other attraction points in territory");

                        group.set_target_cell(new_target_cell);
                        group.remaining_stay_time = rng.gen_range(MIN_STAY_TIME..MAX_STAY_TIME);

                        group.movement = MovementMode::ApTransition;
                       // group.target_cell = None;
                        
                        break;
                    }

                    
                    // if distance to current ap is more than 3 cells individuals more back towards the ap
                    if ((group.x as isize) - (group.target_cell.unwrap().0 as isize)).abs() <= 3
                    && ((group.y as isize) - (group.target_cell.unwrap().1 as isize)).abs() <= 3
                    {
                        move_towards_highest_quality(grid, group, rng);
                        group.daily_movement_distance -= 1;
                    }else {
                        
                       // correlated_random_walk_towards_target(grid, group, rng);
                        //move_one_step_towards_target_cell(group);
                        move_one_step_towards_target_cell_with_random(group,rng,grid);
                        group.daily_movement_distance -= 1;
                    }
                    
                    
                   // println!("Movement left: {}", group.daily_movement_distance); // DEBUG PRINT

                      // Update presence timer
                    //group.memory.presence_timer += 1;
                    //
                    //// Check if presence time limit is reached or 5% chance to move
                    //if group.memory.presence_timer >= PRESENCE_TIME_LIMIT || rng.gen_range(0..100) < MOVE_CHANCE_PERCENTAGE {
                    //    // Reset presence timer and force movement to a random cell
                    //    group.memory.presence_timer = 0;
                    //    //move_to_random_adjacent_cells(grid.len(), group, rng);
                    //
                    //    group.movement = MovementMode::ApTransition;
                    //}
                }
            }
        }
        // Reset movement distance
        group.daily_movement_distance =  DEFAULT_DAILY_MOVEMENT_DISTANCE;

        // update the stay time around the ap
        group.update_remaining_stay_time();
    }

}

pub fn move_to_new_ap(grid: &Vec<Vec<Cell>>, group: &mut Groups, rng: &mut impl Rng) { // UNSUSED
    // If remaining_stay_time is 0 or there is no target_cell, select a new target_cell from attraction points
    if group.remaining_stay_time == 0 || group.target_cell.is_none() {
        let territory_ap = get_attraction_points_in_territory(grid, group.group_id);
        let new_target_cell = territory_ap
            .choose(rng)
            .cloned()
            .expect("No attraction points in territory");

        group.set_target_cell(new_target_cell);
        group.remaining_stay_time = rng.gen_range(MIN_STAY_TIME..MAX_STAY_TIME);
    }

    // Move towards the target_cell using move_towards_highest_quality
    //move_towards_highest_quality(grid, group, rng);

    // Use CRW to move to target
    correlated_random_walk_towards_target(grid, group, rng);


    // Decrement remaining_stay_time
    group.update_remaining_stay_time();
}


pub fn move_towards_highest_quality(grid: &Vec<Vec<Cell>>, group: &mut Groups, rng: &mut impl Rng) {
    // Generate a list of adjacent cells
    let adjacent_cells = vec![
        (group.x.saturating_sub(1), group.y),
        (group.x.saturating_add(1), group.y),
        (group.x, group.y.saturating_sub(1)),
        (group.x, group.y.saturating_add(1)),
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

    update_memory(&mut group.memory.known_cells, &mut group.memory.known_cells_order, (group.x, group.y), MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);


    // Update individual's position
    group.x = new_x;
    group.y = new_y;
}

//pub fn move_towards_target_cell(group: &mut Groups) {
//    // Check if there is a target cell set
//    if let Some(target_cell) = group.target_cell {
//        // Calculate the movement direction towards the target cell
//        let direction = (
//            target_cell.0 as isize - group.x as isize,
//            target_cell.1 as isize - group.y as isize,
//        );
//
//        // Update individual's position
//        group.x = (group.x as isize + direction.0).max(0) as usize;
//        group.y = (group.y as isize + direction.1).max(0) as usize;
//
//        // Update known cells
//        update_memory(
//            &mut group.memory.known_cells,
//            &mut group.memory.known_cells_order,
//            (group.x, group.y),
//            MAX_KNOWN_CELLS,
//        );
//    }
//}

pub fn move_one_step_towards_target_cell(group: &mut Groups) {
    // Check if there is a target cell set
    if let Some(target_cell) = group.target_cell {
        // Calculate the movement direction towards the target cell
        let direction = (
            target_cell.0 as isize - group.x as isize,
            target_cell.1 as isize - group.y as isize,
        );

        // Update individual's position by one step
        group.x = (group.x as isize + direction.0.signum()).max(0) as usize;
        group.y = (group.y as isize + direction.1.signum()).max(0) as usize;

        // Update known cells
        update_memory(
            &mut group.memory.known_cells,
            &mut group.memory.known_cells_order,
            (group.x, group.y),
            MAX_KNOWN_CELLS,
        );
    }
}


pub fn move_one_step_towards_target_cell_with_random(
    group: &mut Groups,
    rng: &mut impl Rng,
    grid: &Vec<Vec<Cell>>,
) {
    // Check if there is a target cell set
    if let Some(target_cell) = group.target_cell {
        // Randomly decide whether to move towards the target or move randomly
        if rng.gen_range(0..100) < 90 {
            // Calculate the movement direction towards the target cell
            let direction = (
                target_cell.0 as isize - group.x as isize,
                target_cell.1 as isize - group.y as isize,
            );

            // Update individual's position by one step
            group.x = (group.x as isize + direction.0.signum()).max(0) as usize;
            group.y = (group.y as isize + direction.1.signum()).max(0) as usize;

            // Update known cells
            update_memory(
                &mut group.memory.known_cells,
                &mut group.memory.known_cells_order,
                (group.x, group.y),
                MAX_KNOWN_CELLS,
            );
        } else {
            // Move randomly
            move_to_random_adjacent_cell(group, rng, grid);
        }
    }
}


// Function for correlated random walk towards the target // NOT WORKING
fn correlated_random_walk_towards_target(grid: &Vec<Vec<Cell>>, group: &mut Groups, rng: &mut impl Rng) {
    // Autoregressive parameters 
    let alpha = 0.5; // Persistence parameter

    // Placeholder for storing the last movement direction
    let mut last_direction = (0, 0);

    // Generate a list of adjacent cells
    let adjacent_cells = vec![
        (group.x.saturating_sub(1), group.y),
        (group.x.saturating_add(1), group.y),
        (group.x, group.y.saturating_sub(1)),
        (group.x, group.y.saturating_add(1)),
    ];

    // Calculate the quality score for each adjacent cell
    let quality_scores: Vec<_> = adjacent_cells.iter()
        .filter(|&&(x, y)| x < grid.len() && y < grid[0].len())
        .map(|&(x, y)| (x, y, calculate_quality_score(grid, x, y)))
        .collect();

    // Sort cells by quality in descending order
    let sorted_cells: Vec<_> = quality_scores.iter()
        .cloned()
        .filter(|&(x, y, _)| x < grid.len() && y < grid[0].len())
        .collect();
    let sorted_cells: Vec<_> = sorted_cells.iter()
        .cloned()
        .filter(|&(x, y, _)| x < grid.len() && y < grid[0].len())
        .collect();
    let sorted_cells: Vec<_> = sorted_cells.iter()
        .cloned()
        .filter(|&(x, y, _)| x < grid.len() && y < grid[0].len())
        .collect();

    // Select the first cell with quality > 0
    let target_cell = sorted_cells
        .first()
        .map(|&(x, y, _)| (x, y))
        .unwrap_or_else(|| random_cell(grid.len(), rng));

    // Update known cells
    update_memory(&mut group.memory.known_cells, &mut group.memory.known_cells_order, (group.x, group.y), MAX_KNOWN_CELLS);

    // Calculate the movement direction towards the target cell
    let direction = (target_cell.0.saturating_sub(group.x), target_cell.1.saturating_sub(group.y));

    // Update the movement direction using autoregressive model
    let correlated_direction = (
        (alpha * direction.0 as f64 + (1.0 - alpha) * last_direction.0 as f64).round() as isize,
        (alpha * direction.1 as f64 + (1.0 - alpha) * last_direction.1 as f64).round() as isize,
    );

    // Update individual's position
    group.x = (group.x as isize + correlated_direction.0).max(0) as usize;
    group.y = (group.y as isize + correlated_direction.1).max(0) as usize;

    // Update last_direction for the next iteration
    last_direction = correlated_direction;
}



//TEST
pub fn move_to_random_adjacent_cells_2(grid: &Vec<Vec<Cell>>, group: &mut Groups, rng: &mut impl Rng){
    // Get the current position of the individual
    let current_x = group.x;
    let current_y = group.y;

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

    update_memory(&mut group.memory.known_cells, &mut group.memory.known_cells_order, (group.x, group.y), MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);

    // Update individual's position
    group.x = target_cell.0;
    group.y = target_cell.1;
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


// Function to move to a random adjacent cell
fn move_to_random_adjacent_cell(group: &mut Groups, rng: &mut impl Rng, grid: &Vec<Vec<Cell>>) {
    let adjacent_cells = vec![
        (group.x.saturating_sub(1), group.y),
        (group.x.saturating_add(1), group.y),
        (group.x, group.y.saturating_sub(1)),
        (group.x, group.y.saturating_add(1)),
    ];

    // Shuffle the list of adjacent cells
    let mut shuffled_cells = adjacent_cells.clone();
    shuffled_cells.shuffle(rng);

    // Select the first cell (randomized) with quality > 0
    if let Some(target_cell) = shuffled_cells
        .into_iter()
        .filter(|&(x, y)| x < grid.len() && y < grid[0].len() && grid[x][y].quality > 0.0)
        .next()
    {
        // Update individual's position
        group.x = target_cell.0;
        group.y = target_cell.1;

        // Update known cells
        update_memory(
            &mut group.memory.known_cells,
            &mut group.memory.known_cells_order,
            (group.x, group.y),
            MAX_KNOWN_CELLS,
        );
    }
}

pub fn move_to_random_adjacent_cells3(grid_size: usize, group: &mut Groups, rng: &mut impl Rng) {
    // Get the current position of the individual
    let current_x = group.x;
    let current_y = group.y;

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

    update_memory(&mut group.memory.known_cells, &mut group.memory.known_cells_order, (group.x, group.y), MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);

    // Update individual's position
    group.x = target_cell.0;
    group.y = target_cell.1;
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

pub fn update_counter(n_groups: &mut usize,group: &mut Vec<Groups>){

    *n_groups = group.len();
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
    flip_grid(&mut grid);

    // Setup the individuals
    let mut groups = group_setup(&cell_info_list, &mut grid, num_groups);

      // Check if any individual is outside the bounds
      if groups.iter().any(|ind| ind.x >= grid.len() || ind.y >= grid[0].len()) {
        println!("Some individuals are outside the bounds of the grid.");
    }

    fill_initial_groups(&mut groups, &grid);

    (grid, groups)
}

// Main model

fn main() {
    // Define grid dimensions
    //let grid_size = 25;

    //assign_to_constants(&Input);

    let num_groups = 1; // FIX ME DEBUG CHANGE TO 1

    let file_path = "input/landscape/redDeer_global_50m.asc";
   
    // Setup the landscape and individuals

    let (mut grid, mut groups) = setup(file_path, num_groups);

    // Vector to store grid states for all iterations
    let mut all_grid_states: Vec<(usize, Vec<Vec<Cell>>)> = Vec::new();

    // Vector to store individual states for all iterations
    let mut all_group_states: Vec<(usize, Vec<Groups>)> = Vec::new();

    // Vector to store global variables for all iterations
    let mut all_global_variables: Vec<GlobalVariables> = Vec::new();

       let mut global_variables = GlobalVariables {
        age_mortality: 0,
        random_mortality: 0,
        n_individuals: groups.len(), // FIX ME
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

   // place_attraction_points(&mut grid, 3,6,1600);

    //Debug print:
    println!("Setup complete -> starting iteration");

    // Simulate and save the grid state and individual state for each iteration
    for iteration in 1..= RUNTIME {

        // Simulate movement of individuals
        let mut rng = rand::thread_rng();
        move_individuals(&grid, &mut groups, &mut rng);

   

        if global_variables.month == 5 {
            //debug print REMOVE ME
            //print!("reproduction is triggered");

          reproduction(global_variables.month, &mut groups, iteration);  // Adjust num_new_individuals               //   <-----------------temp OFF
        }

        if global_variables.day == 15 {

        //  mortality(&survival_prob, &mut individuals, &mut global_variables.random_mortality);                    //   <-----------------temp OFF
        }

        //age individuals by one day
        ageing(&mut groups, &mut global_variables.age_mortality);                                         //   <-----------------temp OFF

        //Updating various counters such as number of individuals
        update_counter(&mut global_variables.n_individuals, &mut groups);

        // Update group memory
        //update_group_memory(&mut individuals); // turned off for speed

        if iteration == (RUNTIME) {
            // Save the grid state for the current (last) iteration
            //println!("its happening");
            all_grid_states.push((iteration, grid.clone()));
            }
    
            // Save the individual state for the current iteration
            all_group_states.push((iteration, groups.clone()));

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
    save_groups_as_csv("output/all_groups.csv", &all_group_states).expect("Failed to save groups as CSV");

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