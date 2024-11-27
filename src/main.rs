
use rand::Rng;
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::iter;
//use rayon::iter;
//use rayon::iter;
//use rand_distr::num_traits::int;
//use core::time;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Result,Write};
//use std::path::Path;
//use std::collections::VecDeque;
use std::time::Instant;
//use serde::{de, Deserialize, Serialize};
//use chrono::Datelike;
//use chrono::Timelike;
use std::fs;
use chrono::Local;
//use zip::{ZipWriter, write::FileOptions, CompressionMethod};
//use std::thread;
//use std::time::Duration;
use std::collections::HashMap;
use std::path::Path;


use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::VecDeque;

use serde::Deserialize;
use lazy_static::lazy_static;


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
use ageing::*;

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

mod hunting;
use hunting::*;

mod behaviour;
//type InteractionLayer = HashMap<(usize, usize, usize), InteractionCell>;






// Define a struct to represent a group
#[derive(Debug, Clone)]
pub struct Model {
    pub groups: Vec<Groups>,
    pub grid: Vec<Vec<Cell>>,
    pub global_variables: GlobalVariables,
    pub roamers: Vec<RoamingIndividual>,
    pub dispersers: Vec<DispersingFemaleGroup>,
    pub interaction_layer: InteractionLayer,
    pub carcasses: Vec<Carcass>,
    pub high_seats: Vec<HighSeat>,
    pub hunting_statistics: HuntingStatistics,
    pub metadata: SimMetaData,
    pub attract_points: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub struct SimMetaData {
    pub iteration_output: Vec<Vec<String>>,
    pub simulation_id: String,
}

impl SimMetaData {
    pub fn new() -> SimMetaData {
        SimMetaData {
            iteration_output: Vec::new(),
            simulation_id: "NA".to_string(),
        }
    }
    pub fn clear(&mut self) {
        self.iteration_output.clear();
    }
    
}

   

#[derive(Debug, PartialEq)]
struct HeapElement {
    priority: f64,
    coordinates: (usize, usize),
}

impl Eq for HeapElement {}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse to make higher priority values appear first in the heap
        other.priority.partial_cmp(&self.priority)
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Wrapper struct for f32 to implement Ord
#[derive(Debug, PartialEq, PartialOrd)]
struct OrdFloat(f32);

impl Eq for OrdFloat {}

impl Ord for OrdFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
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
    mfd: bool,
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

    pub fn remove_random_member(&mut self) {
        if let Some(index) = self.group_members.choose(&mut rand::thread_rng()).map(|member| self.group_members.iter().position(|m| *m == *member).unwrap()) {
            self.group_members.remove(index);
        }
    }

    pub fn get_id_random_group_member(&self) -> usize {
        if let Some(member) = self.group_members.choose(&mut rand::thread_rng()) {
            member.individual_id
        } else {
            0
        }
    }

    pub fn remove_group_member(&mut self, member_id: usize) {
        if let Some(index) = self.group_members.iter().position(|member| member.individual_id == member_id) {
            self.group_members.remove(index);
        }
    }

    // Method to perform logic on each group member
    //pub fn process_group_members(&self) {
    //    for member in &self.group_members {
    //        // Perform logic on each group member
    //        // Example: println!("{:?}", member);
    //    }
    //}

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
    let time_of_infection = None;
    let infection_stage = InfectionStage::NotInfected;

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
        time_of_infection,
        infection_stage,

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

    pub fn expand_territory_with_natural_shape2(&mut self, grid: &mut Vec<Vec<Cell>>) {
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




    pub fn expand_territory_with_natural_shape3(
        &mut self,
        grid: &mut Vec<Vec<Cell>>
    ) {

        let quality_threshold = 0.3;
        let min_cells = 1000;
        let max_cells = 1600;
      
           
        let mut territory_cells = HashSet::new();

        if let Some((x, y)) = self.core_cell {
            // Start with the core cell
            territory_cells.insert((x, y));
    
            let mut to_explore = BinaryHeap::new();
            to_explore.push(HeapElement {
                priority: 0.0, // Initial priority
                coordinates: (x, y),
            });
            let mut iterations = 0;
            while !to_explore.is_empty() && territory_cells.len() < max_cells && iterations < 10000 {
                let HeapElement { coordinates: (cx, cy), .. } = to_explore.pop().unwrap();
    
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
    
                        let nx = cx as isize + dx;
                        let ny = cy as isize + dy;
    
                        if nx >= 0
                            && ny >= 0
                            && nx < grid.len() as isize
                            && ny < grid[0].len() as isize
                        {
                            let nx = nx as usize;
                            let ny = ny as usize;
    
                            let cell = &grid[nx][ny];
    
                            if !cell.territory.is_taken && cell.quality > 0.3 {
                                territory_cells.insert((nx, ny));
    
                                to_explore.push(HeapElement {
                                    priority: cell.quality,
                                    coordinates: (nx, ny),
                                });
                            }
                        }
                    }
                }
                iterations += 1;
            }
    
            // Check if the territory is smaller than the minimum required cells
            if territory_cells.len() < min_cells {
                eprintln!(
                    "Group {} did not meet the required minimum cells. Expanding search...",
                    self.group_id
                );
            
                // Secondary expansion logic: Force-claim nearby eligible cells
                for x in 0..grid.len() {
                    for y in 0..grid[0].len() {
                        if grid[x][y].quality > 0.3 && !grid[x][y].territory.is_taken {
                            territory_cells.insert((x, y));
                            grid[x][y].territory.is_taken = true;
                            grid[x][y].territory.taken_by_group = self.group_id;
            
                            if territory_cells.len() >= min_cells {
                                break;
                            }
                        }
                    }
                    if territory_cells.len() >= min_cells {
                        break;
                    }
                }
            }
    
            // Assign cells to the group's territory
            for (x, y) in &territory_cells {
                let cell = &mut grid[*x][*y];
                cell.territory.is_taken = true;
                cell.territory.taken_by_group = self.group_id;
            }
        } else {
            eprintln!(
                "Error: Group {} does not have a core cell assigned. Skipping territory expansion.",
                self.group_id
            );
        }
    }
    
    pub fn expand_territory_with_natural_shape4(&mut self, grid: &mut Vec<Vec<Cell>>) {
        // Constants for desired number of cells
        let min_cells = 1000;
        let max_cells = 1600;
    
        // If no core cell exists, return early
        if let Some((core_x, core_y)) = self.core_cell {
            let mut territory_cells: HashSet<(usize, usize)> = HashSet::new();
            let mut to_explore: VecDeque<(usize, usize)> = VecDeque::new();
            let mut iterations = 0;
    
            // Start with the core cell
            to_explore.push_back((core_x, core_y));
            territory_cells.insert((core_x, core_y));
    
            // Expand the territory
            while !to_explore.is_empty() && territory_cells.len() < max_cells && iterations < 10000 {
                iterations += 1;
    
                let (x, y) = to_explore.pop_front().unwrap();
    
                // Explore neighboring cells
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
    
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;
    
                        // Ensure the cell is within bounds
                        if new_x >= 0
                            && new_x < grid.len() as isize
                            && new_y >= 0
                            && new_y < grid[0].len() as isize
                        {
                            let new_x = new_x as usize;
                            let new_y = new_y as usize;
    
                            // Check eligibility of the cell
                            if !grid[new_x][new_y].territory.is_taken
                                && grid[new_x][new_y].quality > 0.3
                                && !territory_cells.contains(&(new_x, new_y))
                            {
                                // Add the cell to the territory
                                territory_cells.insert((new_x, new_y));
                                to_explore.push_back((new_x, new_y));
                            }
                        }
                    }
                }
            }
    
            // Check if the territory meets the minimum size
            if territory_cells.len() < min_cells {
                eprintln!(
                    "Group {} did not meet the required minimum cells. Expanding search...",
                    self.group_id
                );
    
                // Try to force-claim nearby eligible cells
                for x in 0..grid.len() {
                    for y in 0..grid[0].len() {
                        if !grid[x][y].territory.is_taken
                            && grid[x][y].quality > 0.3
                            && territory_cells.len() < min_cells
                        {
                            territory_cells.insert((x, y));
                            grid[x][y].territory.is_taken = true;
                            grid[x][y].territory.taken_by_group = self.group_id;
                        }
                    }
                }
            }
    
            // Finalize the territory claim
            for (x, y) in &territory_cells {
                grid[*x][*y].territory.is_taken = true;
                grid[*x][*y].territory.taken_by_group = self.group_id;
            }
    
            // Log the result
            //println!(
            //    "Group {} finalized territory with {} cells.",
            //    self.group_id,
            //    territory_cells.len()
            //);
        }
    }
 
    pub fn expand_territory_with_natural_shape5(&mut self, grid: &mut Vec<Vec<Cell>>) {
        // Constants for desired number of cells
        let min_cells = 1000;
        let max_cells = 1600;
    
        // Default radii for the ellipsoid
        let radius_x: f32 = 50.0;
        let radius_y: f32 = 50.0;
    
        // Ensure the core cell exists
        if let Some((core_x, core_y)) = self.core_cell {
            let mut territory_cells: HashSet<(usize, usize)> = HashSet::new();
            let mut to_explore: BinaryHeap<(OrdFloat, (usize, usize))> = BinaryHeap::new();
            let mut iterations = 0;
    
            // Start with the core cell
            to_explore.push((OrdFloat(0.0), (core_x, core_y)));
            territory_cells.insert((core_x, core_y));
    
            // Expand the territory while keeping it within the ellipsoid
            while !to_explore.is_empty() && territory_cells.len() < max_cells && iterations < 10000 {
                iterations += 1;
    
                let (_, (x, y)) = to_explore.pop().unwrap();
    
                // Explore neighboring cells with a weighted priority
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue; // Skip the current cell itself
                        }
    
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;
    
                        // Ensure the new cell is within grid bounds
                        if new_x >= 0
                            && new_x < grid.len() as isize
                            && new_y >= 0
                            && new_y < grid[0].len() as isize
                        {
                            let new_x = new_x as usize;
                            let new_y = new_y as usize;
    
                            // Calculate the ellipsoid distance for the candidate cell
                            let distance_sq = (new_x as f32 - core_x as f32).powi(2) / radius_x.powi(2)
                                + (new_y as f32 - core_y as f32).powi(2) / radius_y.powi(2);
    
                            // Only add cells inside the ellipsoid and not already explored
                            if distance_sq <= 1.0
                                && !grid[new_x][new_y].territory.is_taken
                                && grid[new_x][new_y].quality > 0.3
                                && !territory_cells.contains(&(new_x, new_y))
                            {
                                // Use distance to the core as a priority (closer cells have higher priority)
                                to_explore.push((OrdFloat(-distance_sq), (new_x, new_y)));
    
                                // Insert the cell into the territory
                                territory_cells.insert((new_x, new_y));
                            }
                        }
                    }
                }
            }
    
            // Finalize the territory claim
            for (x, y) in territory_cells {
                grid[x][y].territory.is_taken = true;
                grid[x][y].territory.taken_by_group = self.group_id;
            }
    
            // Log the result
            //println!(
            //    "Group {} finalized territory with {} cells.",
            //    self.group_id,
            //    territory_cells.len()
            //);
        }
    }
    
    pub fn expand_territory_with_natural_shape(&mut self, grid: &mut Vec<Vec<Cell>>) {
        // Constants for desired number of cells
        let min_cells = CONFIG.min_hr_cells;
        let max_cells = CONFIG.max_hr_cells;
    
        // Default radii for the ellipsoid
        let radius_x: f32 = 50.0;
        let radius_y: f32 = 50.0;
    
        // Ensure the core cell exists
        if let Some((core_x, core_y)) = self.core_cell {
            let mut territory_cells: HashSet<(usize, usize)> = HashSet::new();
            let mut to_explore: BinaryHeap<(OrdFloat, (usize, usize))> = BinaryHeap::new();
            let mut iterations = 0;
    
            // Start with the core cell
            to_explore.push((OrdFloat(0.0), (core_x, core_y)));
            territory_cells.insert((core_x, core_y));
    
            // Expand the territory while keeping it within the ellipsoid
            while !to_explore.is_empty() && territory_cells.len() < max_cells && iterations < 10000 {
                iterations += 1;
    
                let (_, (x, y)) = to_explore.pop().unwrap();
    
                // Explore neighboring cells with a weighted priority
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue; // Skip the current cell itself
                        }
    
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;
    
                        // Ensure the new cell is within grid bounds
                        if new_x >= 0
                            && new_x < grid.len() as isize
                            && new_y >= 0
                            && new_y < grid[0].len() as isize
                        {
                            let new_x = new_x as usize;
                            let new_y = new_y as usize;
    
                            // Calculate the ellipsoid distance for the candidate cell

                            let mut rng = rand::thread_rng();
                            let noise_factor = CONFIG.hr_border_fuzzy; // Low border fuzzyness
                            
                            let distance_sq = (new_x as f32 - core_x as f32).powi(2) / radius_x.powi(2)
                                + (new_y as f32 - core_y as f32).powi(2) / radius_y.powi(2)
                                + rng.gen_range(-noise_factor..noise_factor); // Add randomness


                                if distance_sq <= 1.0
                                && !grid[new_x][new_y].territory.is_taken
                                && grid[new_x][new_y].quality > 0.3
                                && !territory_cells.contains(&(new_x, new_y))
                            {
                                // Use distance to the core as a priority (closer cells have higher priority)
                                to_explore.push((OrdFloat(-distance_sq), (new_x, new_y)));
    
                                // Insert the cell into the territory
                                territory_cells.insert((new_x, new_y));
                            }
                        }
                    }
                }
            }
    
            // Finalize the territory claim
            for (x, y) in &territory_cells {
                grid[*x][*y].territory.is_taken = true;
                grid[*x][*y].territory.taken_by_group = self.group_id;
            }
    
            // Log the result
            //println!(
            //    "Group {} finalized territory with {} cells.",
            //    self.group_id,
            //    territory_cells.len()
            //);
        }
    }
    
   
    pub fn expand_territory_with_natural_shape_and_radius(&mut self, grid: &mut Vec<Vec<Cell>>) {
    // Constants for desired number of cells, shape, and radius
    let min_desired_cells = CONFIG.min_hr_cells;
    let max_desired_cells = CONFIG.max_hr_cells;
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

    pub fn symptomatic_member_present(&self)->bool{
        let mut symptomatic = false;
        for member in &self.group_members {
            if member.health_status == HealthStatus::Infected && member.infection_stage == InfectionStage::Symptomatic || member.infection_stage == InfectionStage::HighlyInfectious {
                symptomatic = true;
                break;
            }
        }
        symptomatic
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
#[derive(Debug, Clone, PartialEq)]
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
    time_of_infection: Option<usize>,
    infection_stage: InfectionStage,
}


// Define a struct to represent a groups's memory
#[derive(Debug, Clone)]
struct GroupMemory {
    known_cells: HashSet<(usize, usize)>,
    known_cells_order: Vec<(usize, usize)>,
    //last_visited_cells: HashSet<(usize, usize)>,
    //last_visited_cells_order: Vec<(usize, usize)>,
    //group_member_ids: Vec<usize>,
    //presence_timer: usize,
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
}//

// Define a struct to represent the output
#[derive(Debug)]
struct Output {

}
//
//// Define a struct to represent the input
//#[derive(Debug)]
//struct Input {
//max_age: u32,
//presence_time_limit: usize,
//move_chance_percentage: usize,
//max_known_cells: usize,
//runtime: usize,
//adult_survival: f64,
//piglet_survival: f64,
//adult_survival_day: f64,
//piglet_survival_day: f64,
//min_stay_time:usize,
//max_stay_time:usize,
//default_daily_movement_distance:usize,
//
//}


#[derive(Deserialize)]
struct Config {
    max_age: u32,
    max_known_cells: usize,
    runtime: usize,
    adult_survival_day: f64,
    piglet_survival_day: f64,
    min_stay_time: usize,
    max_stay_time: usize,
    default_daily_movement_distance: usize,
    good_year_chance: usize,
    burn_in_period: usize,
    beta_w: f64,
    beta_b: f64,
    beta_c: f64,
    carcass_contact_prob: f64,
    p_symptomatic: f64,
    n_starting_groups: usize,
    seed: u64,
    min_hr_cells: usize,
    max_hr_cells: usize,
    hr_border_fuzzy: f32,
    ap_max_jitter: isize,
    ap_jitter_factor: isize,
    min_ap: i32

}

lazy_static! {
    static ref CONFIG: Config = read_config("config.json");
}


//// consants
//const MAX_AGE: u32 = 365 * 12;
////const PRESENCE_TIME_LIMIT: usize = 5;
////const MOVE_CHANCE_PERCENTAGE: usize = 5;
//const MAX_KNOWN_CELLS: usize = 60; // DEBUG FIX ME with actual values
////const MAX_LAST_VISITED_CELLS: usize = 3;
//const RUNTIME: usize = 365 * 3; 
////const ADULT_SURVIVAL: f64 = 0.65;
////const PIGLET_SURVIVAL: f64 = 0.5;
//const ADULT_SURVIVAL_DAY: f64 =  0.9647;
//const PIGLET_SURVIVAL_DAY: f64 = 0.9438;
//const MIN_STAY_TIME: usize = 1;
//const MAX_STAY_TIME: usize = 14;
//const DEFAULT_DAILY_MOVEMENT_DISTANCE: usize = 20;
//const GOOD_YEAR_CHANCE: usize = 15; // 15% chance of a good year
//const BURN_IN_PERIOD: usize = 0; // 365 * 2; // 2 years burn in period
//const BETA_W: f64 = 0.05; // within group transmission rate // FIX ME
//const BETA_B: f64 = 0.001; // between group transmission rate // FIX ME
//const BETA_C: f64 = 0.6; // carcass transmission rate // FIX ME
//const CARCASS_CONTACT_PROB : f64 = 0.10; // carcass contact probability // FIX ME
//const P_SYMPTOMATIC: f64 = 0.5; // probability of being symptomatic // FIX ME



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
 

// General setup

pub fn setup(file_path: &str, num_groups: usize) -> (Vec<Vec<Cell>>, Vec<Groups>) {
    // Setup the landscape (grid)
    let (mut grid, _metadata) = match landscape_setup_from_ascii(file_path) {
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
    let sim_id = generate_unique_simulation_id();


     // Initialize the logger
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    
    // variable that is set to the system time when the simulation starts
    
    let start_time = Instant::now();


    log::info!("--------------------------->>> Starting simulation {} at time: {:?}", sim_id, start_time);

    //let mut rng = rand::thread_rng();
    let seed = CONFIG.seed;//[0u8; 32]; 
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    let num_groups = CONFIG.n_starting_groups; // FIX ME DEBUG CHANGE TO 1

    //let file_path = "input/landscape/redDeer_global_50m.asc";
  //let file_path = "input/landscape/test.asc";
  //let file_path = "input/landscape/wb_50x50_prob_pred_s18.asc";
    //let file_path = "input/oct24_bavariaMaps/hs_bavaria_10_24.asc";
    let file_path = "input/oct24_bavariaMaps/hs_bavaria_chunk_11.asc";

    // Setup the landscape and individuals
    log::info!("Setting up the landscape and individuals");
    let (mut grid, mut groups) = setup(file_path, num_groups); 

    // adjust attraction points
    log::info!("Adjusting attraction points");
    place_additional_attraction_points(&mut grid, &mut groups, 3, &mut rng);

    //place_dynamic_attraction_points(&mut grid, &mut groups, 10, &mut rng, "winter");
    log::info!("Removing attraction points with quality 0");
    remove_ap_on_cells_with_quality_0(&mut grid);
    
    log::info!("Initializing individual vectors");
    //create vector for dispersing individuals using the struct in dispersal.rs
    let disperser_vector: &mut Vec<DispersingIndividual> = &mut Vec::new();
    let dispersing_groups_vector: &mut Vec<DispersingFemaleGroup> = &mut Vec::new();

    //create vector for roaming individuals using the struct in roamers.rs
    let roamer_vector: &mut Vec<RoamingIndividual> = &mut Vec::new();

    //create a vector for the carcasses
    let carcass_vector: &mut Vec<Carcass> = &mut Vec::new();

    //high seat vector
    let high_seat_vector: &mut Vec<HighSeat> = &mut Vec::new();
    
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

    // Vector to store high seat states for all iterations
    let mut all_high_seat_states: Vec<(usize, Vec<HighSeat>)> = Vec::new();

    // Vector to store hunting statistics for all iterations
    let mut all_hunting_statistics: Vec<(usize, HuntingStatistics)> = Vec::new();

    let mut all_sim_meta_data:  Vec<(usize, SimMetaData)> = Vec::new();

    let all_ap: Vec<(usize, usize)> = Vec::new();

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
    };

     // Create an instance of InteractionLayer
     let interaction_layer_tmp = InteractionLayer::new();

     // Create an instance of Hunting statistics
     let hunting_statistics = HuntingStatistics::new();

     // Create an instance of SimMetaData
        let sim_meta_data = SimMetaData::new();

     // create the model
     log::info!("Creating the model");
     let mut model = Model {
        grid: grid,
        groups: groups,
        dispersers: dispersing_groups_vector.clone(),
        roamers: roamer_vector.clone(),
        global_variables: global_variables,
        interaction_layer: interaction_layer_tmp,
        carcasses: carcass_vector.clone(),
        high_seats: high_seat_vector.clone(),
        hunting_statistics: hunting_statistics,
        metadata: sim_meta_data,
        attract_points: all_ap.clone(),
    };

    model.metadata.simulation_id = sim_id.clone();

    
    // Allocate survival probabilitiesall_grom1
    let survival_prob = SurvivalProbability {
        adult: CONFIG.adult_survival_day,
        piglet: CONFIG.piglet_survival_day,
    };

    //Debug print:
    println!("Setup complete -> starting iteration");

    // Simulate and save the grid state and individual state for each iteration
    for iteration in 1..= CONFIG.runtime {

        remove_dead_individuals(&mut model);
        check_group_territory_size_and_ap(&mut model);
        model.global_variables.current_time = iteration;

        log::info!("Starting iteration: {}", iteration);
        if model.global_variables.day == 1 && model.global_variables.month == 1 {
            log::info!("good year check: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            good_year_check(&mut model, &mut rng); // check if it is a good year
            roamer_density_dependent_removal(&mut model); //roamers leave the area i.e. are removed when there are more males then females
        }
        
        log::info!("Checking and removing empty groups: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        check_for_empty_groups(&mut model.groups);
        check_and_remove_empty_dispersal_groups(dispersing_groups_vector);
        log::info!("Freeing cells of empty groups and deleting group: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);

        handle_empty_groups(&mut model.groups, &mut model.grid);
        check_for_empty_groups(&mut model.groups);

        //test outsource into file TODO
        let hunting_per_month_list = vec![0.3,0.2,0.2,0.1,0.0,0.0,0.0,0.0,0.0,0.1,0.1,0.2];
        //high seat occupancy
        
      if iteration > CONFIG.burn_in_period {
        //get the current month
        let current_month = model.global_variables.month;
        //use the current month to get the position in the hunting list vector
        let hunting_per_month = hunting_per_month_list[current_month as usize - 1];

        if iteration == 1 {

               //place high seats
                log::info!("Placing high seats");
                handle_high_seats_initial(&mut model, &mut rng, hunting_per_month);
                log::info!("Placing high seats done");
        }

        if model.global_variables.day == 28 {
            if hunting_per_month > 0.0 {
                log::info!("Shuffling high seat occupancy: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            shuffle_high_seat_occupancy(&mut model, &mut rng, hunting_per_month)
            } else {
                log::info!("Removing all high seats and hunting zones: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
                leave_all_high_seats(&mut model);
                remove_all_hunting_zones(&mut model.grid);
                // DEBUG REMOVE ME check if there is any cell with hunting zone true left
                if model.grid.iter().any(|row| row.iter().any(|cell| cell.hunting_zone)) {
                    println!("There are still cells with hunting zone true left");
                }
            }
        }
        log::info!("Within group pathogen transmission: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        pathogen_transmission_within_groups(&mut model, &mut rng);


        pathogen_progression(&mut model, &mut rng);

      }

        //dispersal
        if iteration > 100 {

        if model.global_variables.day == 1 {
           // println!("Dispersal triggered: year {}, month {}, day {}", global_variables.year, global_variables.month, global_variables.day);
            log::info!("Dispersal: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            dispersal_assignment(&mut model.groups, disperser_vector, &mut model.dispersers);
            log::info!("Assigning dispersal targets to individuals: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            assign_dispersal_targets_groups(&mut model.dispersers, &mut model.groups, &mut model.grid, &mut rng);
            //assign male individuals as roamers
            log::info!("Assigning roamer targets to individuals: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            roamer_assignemnt(&mut model.roamers,&mut model.groups);
        }
    
            check_empty_disperser_group(dispersing_groups_vector);
            log::info!("Moving dispersers: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            check_and_remove_empty_dispersal_groups(dispersing_groups_vector);
            move_female_disperser_group(&mut model, &mut rng, iteration);

        }
        log::info!("Initial roamer target assignment: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        initial_roamer_dispersal_target(&mut model.roamers,  &mut model.grid, &mut rng);
        log::info!("Initial roamer movement: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        initial_roamer_dispersal_movement(&mut model, &mut rng,  iteration);
        // Free territory of groups with no members
        if model.global_variables.day == 1 {
          //  free_group_cells(&mut groups, &mut grid);
          //  remove_ap_from_freed_cells(&mut grid);
        }

        log::info!("Removing groups without members: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        delete_groups_without_members(&mut model.groups);

        // Simulate movement of individuals
        log::info!("AP dynamic: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        dynamic_ap(&mut model.grid, &mut model.groups, &mut rng, &mut model.global_variables);
        list_all_attraction_points(&mut model.grid, &mut model.attract_points);
        log::info!("Check AP of groups: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        check_attraction_points_in_territory(&mut model.grid, &mut model.groups, 3, &mut rng);
        log::info!("Roaming movement: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        execute_roaming(&mut model.roamers, &mut model.groups, &mut model.grid, &mut rng, &mut model.interaction_layer, iteration, &mut model.high_seats, &mut model.hunting_statistics);
        log::info!("Group movement: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        delete_groups_without_members(&mut model.groups);
        move_groups(&mut rng,   iteration, &mut model);

        //check dispersers if their target cell == none


        if model.global_variables.day == 5 {
            //debug print REMOVE ME
            //print!("reproduction is triggered");
            log::info!("Reproduction: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
            reproduction(model.global_variables.month, &mut model.groups, iteration, model.global_variables.good_year);  // Adjust num_new_individuals               //   <-----------------temp OFF
        }

        if model.global_variables.day == 15 {

         //mortality(&survival_prob, &mut groups, &mut global_variables.random_mortality);                    //   <-----------------temp OFF
         log::info!("Mortality: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
           // combined_mortality(&survival_prob, &mut groups, &mut global_variables.random_mortality, &mut global_variables.overcapacity_mortality);
            execute_mortality(&mut model, &survival_prob);
        }

        log::info!("Carcass handling: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        handle_carcasses(&mut model);

        //age individuals by one day
        log::info!("Ageing: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        ageing(&mut model);                                         //   <-----------------temp OFF
        

        //Updating various counters such as number of individuals
        log::info!("Updating counters: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        update_counter(&mut model.global_variables, &mut model.groups, &disperser_vector, &roamer_vector);

        // Update group memory
        //update_group_memory(&mut individuals); // turned off for speed

        // Update the interaction layer to remove single individual instances
        log::info!("Deleting single individual instances: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
       // delete_single_individual_instances(&mut model.interaction_layer);

      // if iteration == (6000) {  // DEBUG TESTER REMOVE ME
      //  remove_half_of_all_groups(&mut model);
      // }

        if iteration == (CONFIG.runtime) {
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

            // Save the high seat state for the current iteration
            all_high_seat_states.push((iteration, model.high_seats.clone()));

            // Save the hunting statistics for the current iteration
            all_hunting_statistics.push((iteration, model.hunting_statistics.clone()));

            // clear the hunting statistics for the next iteration
            model.hunting_statistics.clear_hunting_statistics();

            all_sim_meta_data.push((iteration, model.metadata.clone()));

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


        // generate a sim output row
        log::info!("Generating iteration sim output row: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
        generate_iteration_sim_output_row(&mut model);

        all_sim_meta_data.push((iteration, model.metadata.clone()));

        // purge sim_output_row
        model.metadata.clear();

        // Debug print time

        //print!("Day:{}, Month:{}, Year:{}, Individuals:{}\n", global_variables.day, global_variables.month, global_variables.year, global_variables.n_individuals);
        if model.global_variables.month == 1 && model.global_variables.day == 1{
            let perc = (iteration as f64 / CONFIG.runtime as f64 * 100.0).round();
            let elapsed_time = start_time.elapsed().as_secs();
        println!("Simulation {}% complete! - Elapsed time: {}s", perc, elapsed_time);
        }
        // Progress time 
        
        progress_time(&mut model.global_variables);

        model.interaction_layer.clear_interaction_layer(); // clear the interaction layer for the next iteration
 
        log::info!("Iteration complete: year {}, month {}, day {}, iteration {}", model.global_variables.year, model.global_variables.month, model.global_variables.day, iteration);
    }
    println!("Simulation complete, saving output\n");

    //variable that is set to the system time when the simulation ends
    let end_time = Instant::now();
    //variable showing the difference between the start and end time
    let time_taken = end_time.duration_since(start_time);
    println!("Time taken to run simulation: {:?}", time_taken);
    log::info!("Time taken to run simulation: {:?}", time_taken);

    let time_rn = Local::now();
    let folder_name = format!("simulation_{}_t_{}", sim_id, time_rn.format("%Y_%m_%d_%H_%M"));
    let folder_path = format!("output/{}", folder_name);

    // Create the directory
    println!("Creating directory: {}", folder_path);
    match fs::create_dir_all(&folder_path) {
        Ok(_) => println!("Directory created successfully: {}", folder_path),
        Err(e) => {
            println!("Failed to create directory: {}. Error: {:?}", folder_path, e);
            return;
        }
    }

    // Ensure directory creation is flushed to stdout
    std::io::stdout().flush().unwrap();

    save_outputs(&folder_name, all_grid_states, all_group_states, all_global_variables, all_disperser_states, all_roamer_states, all_carcass_states, all_high_seat_states, all_hunting_statistics, all_interaction_layers, folder_path.clone(), all_sim_meta_data);

    let save_time = Instant::now();
    let time_taken_save = save_time.duration_since(end_time);
    println!("Time taken to save output: {:?}", time_taken_save);
    log::info!("Time taken to save output: {:?}", time_taken_save);
    
    //rename the log file to include date and time to the minute
    let now = Local::now();
    log::info!("--------------------------->>> Simulation complete at time: {:?}", now);
    let log_file = format!("logs/log_{}_{}.log",sim_id, now.format("%Y_%m_%d_%H_%M"));
    fs::rename("logs/outputLog.log", log_file.clone()).expect("Failed to rename log file");
    //copy log file to output folder
    let log_file_output = format!("{}/log_{}_{}.log", folder_path, sim_id, now.format("%Y_%m_%d_%H_%M"));
    fs::copy(log_file.clone(), log_file_output).expect("Failed to copy log file to output folder");
    //remove the original log file
    fs::remove_file(log_file).expect("Failed to remove original log file");

}

/*
 This module represents the main file of the ASF Bavaria prototype.
 It contains various imports, module declarations, and struct definitions.
 The `Model` struct represents the main simulation model, containing information about groups, grid, variables, and other entities.
 The `SimMetaData` struct represents metadata for the simulation, including iteration output and simulation ID.
 The `Groups` struct represents a group of individuals, with properties such as group ID, position, members, and movement.
 The `GroupMember` struct represents an individual within a group, with properties such as age, sex, health status, and infection status.
 The module also includes various sub-modules for specific functionalities such as grid functions, saving to CSV, ageing, reproduction, and more.
 The code also includes various helper functions for manipulating groups, territories, and other entities.
 Overall, this code serves as the foundation for running a simulation of ASF (African Swine Fever) in Bavaria.

*/

