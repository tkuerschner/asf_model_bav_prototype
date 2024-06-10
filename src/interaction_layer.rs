use crate::*;
use std::collections::HashMap;

//Interaction layer:
//1. The x and y coordinates of the individual
//2. The group id of the individual
//3. The type of individual (group, roamer, disperser)
//4. The time the individual was at the x and y coordinates
//5. The time the individual left the x and y coordinates

//The interaction layer should be able to return the following information:
//1. The number of individuals at a specific x and y coordinate
//2. The number of groups at a specific x and y coordinate
//3. The number of roamer at a specific x and y coordinate
//4. The number of disperser at a specific x and y coordinate
//5. The number of individuals that have been at a specific x and y coordinate
//6. The number of groups that have been at a specific x and y coordinate

#[derive(Debug, Clone, Default)]
pub struct InteractionCell {
    pub entities: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub group_id: usize,
    pub individual_type: String,
    pub time: usize,
    pub time_left: usize,
    pub duration: usize,
    pub individual_id: usize,
    pub interaction_strength: f64,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            group_id: 0,
            individual_type: "".to_string(),
            time: 0,
            time_left: 0,
            duration: 0,
            individual_id: 0,
            interaction_strength: 0.0,
        }
    }
}


// A key to represent the position and time
type PositionTimeKey = (usize, usize, usize); // (x, y, time)

pub fn create_interaction_layer() -> HashMap<PositionTimeKey, InteractionCell> {
    HashMap::new()
}

// Function to record movement in the interaction layer
pub fn record_movement_in_interaction_layer(interaction_layer: &mut HashMap<PositionTimeKey, InteractionCell>, x: usize, y: usize, time: usize, group_id: usize, individual_type: &str, individual_id: usize) {
    let directions = [
        (0, 0, 1.0), // Target cell
        (0, 1, 0.8), (1, 0, 0.8), (0, -1, 0.8), (-1, 0, 0.8), // Immediate neighbors
        (1, 1, 0.8), (1, -1, 0.8), (-1, 1, 0.8), (-1, -1, 0.8), // Diagonal neighbors
        (0, 2, 0.5), (2, 0, 0.5), (0, -2, 0.5), (-2, 0, 0.5), // Next distance neighbors
        (1, 2, 0.5), (2, 1, 0.5), (2, -1, 0.5), (1, -2, 0.5), // Mixed neighbors
        (-1, 2, 0.5), (-2, 1, 0.5), (-2, -1, 0.5), (-1, -2, 0.5)//,
        //(0, 3, 0.1), (3, 0, 0.1), (0, -3, 0.1), (-3, 0, 0.1), // Further neighbors
        //(1, 3, 0.1), (3, 1, 0.1), (3, -1, 0.1), (1, -3, 0.1),
        //(-1, 3, 0.1), (-3, 1, 0.1), (-3, -1, 0.1), (-1, -3, 0.1),
        //(2, 2, 0.1), (2, -2, 0.1), (-2, 2, 0.1), (-2, -2, 0.1)
    ];

    let mut added_positions = std::collections::HashSet::new();

    for &(dx, dy, strength) in &directions {
        let nx = x.wrapping_add(dx as usize);
        let ny = y.wrapping_add(dy as usize);
        
        let key = (nx, ny, time);
        if added_positions.insert(key) {
            let entity = Entity {
                group_id,
                individual_type: individual_type.to_string(),
                time,
                time_left: 0,
                duration: 0,
                individual_id,
                interaction_strength: strength,
            };

            let cell = interaction_layer.entry(key).or_insert_with(InteractionCell::default);

            // Check for duplicates
            if !cell.entities.iter().any(|e| ((e.individual_id == individual_id) || (e.group_id == group_id)) && e.time == time) {
                cell.entities.push(entity);
            }
        }
    }
}

// Function to record movement in the interaction layer for roamers only in the cell that they are in
pub fn record_movement_in_interaction_layer_for_roamers(interaction_layer: &mut HashMap<PositionTimeKey, InteractionCell>, x: usize, y: usize, time: usize, group_id: usize, individual_type: &str, individual_id: usize) {
    let directions = [
        (0, 0, 1.0), // Target cell
    ];

    let mut added_positions = std::collections::HashSet::new();

    for &(dx, dy, strength) in &directions {
        let nx = x.wrapping_add(dx as usize);
        let ny = y.wrapping_add(dy as usize);
        
        let key = (nx, ny, time);
        if added_positions.insert(key) {
            let entity = Entity {
                group_id,
                individual_type: individual_type.to_string(),
                time,
                time_left: 0,
                duration: 0,
                individual_id,
                interaction_strength: strength,
            };

            let cell = interaction_layer.entry(key).or_insert_with(InteractionCell::default);

            // Check for duplicates
            if !cell.entities.iter().any(|e| ((e.individual_id == individual_id) || (e.group_id == group_id)) && e.time == time) {
                cell.entities.push(entity);
            }
        }
    }
}

pub fn delete_single_individual_instances(interaction_layer: &mut InteractionLayer) {
    interaction_layer.retain(|_, cell| {
        let mut time_map: HashMap<usize, Vec<&Entity>> = HashMap::new();

        for entity in &cell.entities {
            time_map.entry(entity.time).or_insert_with(Vec::new).push(entity);
        }

        // Retain only those cells where there's at least one time slot with more than one entity
        time_map.values().any(|entities| entities.len() > 1)
    });
}

/* 


//function to record the time an individual left a specific x and y coordinate
pub fn record_time_left_in_interaction_layer(interaction_layer: &mut Vec<Vec<InteractionCell>>, x: usize, y: usize, group_id: usize, individual_type: &str, time: usize) {
    for entity in interaction_layer[x][y].entities.iter_mut() {
        if entity.group_id == group_id && entity.individual_type == individual_type {
            entity.time_left = time;
            entity.duration = entity.time_left - entity.time;
        }
    }
}

// Function to get ids of all the individuals that are here with me
pub fn get_individuals_here_with_me(interaction_layer: &HashMap<PositionTimeKey, InteractionCell>, x: usize, y: usize, time: usize, group_id: usize) -> Vec<usize> {
    let key = (x, y, time);
    if let Some(cell) = interaction_layer.get(&key) {
        cell.entities.iter().filter(|&individual| individual.group_id != group_id).map(|individual| individual.group_id).collect()
    } else {
        Vec::new()
    }
}

//function to get the number of individuals at a specific x and y coordinate
pub fn get_number_of_individuals_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].entities.len()
}

//function to get the number of groups at a specific x and y coordinate
pub fn get_number_of_groups_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].entities.iter().filter(|individual| individual.individual_type == "group").count()
}

//function to get the number of roamer at a specific x and y coordinate

pub fn get_number_of_roamers_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].entities.iter().filter(|individual| individual.individual_type == "roamer").count()
}

//function to get the number of disperser at a specific x and y coordinate
pub fn get_number_of_dispersers_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].entities.iter().filter(|individual| individual.individual_type == "disperser").count()
}

//function to get the number of individuals that have been at a specific x and y coordinate
pub fn get_number_of_individuals_that_have_been_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].entities.iter().count()
}

//function to get the number of groups that have been at a specific x and y coordinate
pub fn get_number_of_groups_that_have_been_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].entities.iter().filter(|individual| individual.individual_type == "group").count()
}

//function to get the number of roamer that have been at a specific x and y coordinate
pub fn get_number_of_roamers_that_have_been_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].entities.iter().filter(|individual| individual.individual_type == "roamer").count()
}

//function to get the number of disperser that have been at a specific x and y coordinate
pub fn get_number_of_dispersers_that_have_been_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].entities.iter().filter(|individual| individual.individual_type == "disperser").count()
}

//function to get the number of individuals that have been at a specific x and y coordinate at a specific time
pub fn get_number_of_individuals_that_have_been_at_coordinate_at_time(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize, time: usize) -> usize {
    interaction_layer[x][y].entities.iter().filter(|individual| individual.time <= time && individual.time_left >= time).count()
}

// function to check if any individual is here with me that is not of my group
pub fn any_other_individual_here(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize, group_id: usize) -> bool {
    interaction_layer[x][y].entities.iter().any(|individual| individual.group_id != group_id)
}


// function to check if any individual is within a radius of 5 cells with me that is not of my group
pub fn any_other_individual_within_radius(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize, group_id: usize) -> bool {
    let mut result = false;
    for i in 0..5 {
        for j in 0..5 {
            if x + i < interaction_layer.len() && y + j < interaction_layer[0].len() {
                result = result || any_other_individual_here(interaction_layer, x + i, y + j, group_id);
            }
            if x + i < interaction_layer.len() && y as i32 - j as i32 >= 0 {
                result = result || any_other_individual_here(interaction_layer, x + i, y - j, group_id);
            }
            if x as i32 - i as i32 >= 0 && y + j < interaction_layer[0].len() {
                result = result || any_other_individual_here(interaction_layer, x - i, y + j, group_id);
            }
            if x as i32 - i as i32 >= 0 && y as i32 - j as i32 >= 0 {
                result = result || any_other_individual_here(interaction_layer, x - i, y - j, group_id);
            }
        }
    }
    result
}
 */






