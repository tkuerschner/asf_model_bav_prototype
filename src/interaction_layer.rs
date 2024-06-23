
use crate::*;

use kdtree::distance::squared_euclidean;
use kdtree::KdTree;

#[derive(Debug, Clone)]
pub struct Entity {
    pub group_id: usize,
    pub individual_type: String,
    pub time: usize,
    pub time_left: usize,
    pub duration: usize,
    pub individual_id: usize,
    pub interaction_strength: f64,
    pub x: f64,  // Float x coordinate
    pub y: f64,  // Float y coordinate
}

impl Entity {
    pub fn new(group_id: usize, individual_type: &str, time: usize, time_left: usize, duration: usize, individual_id: usize, interaction_strength: f64, x: f64, y: f64) -> Self {
        Entity {
            group_id,
            individual_type: individual_type.to_string(),
            time,
            time_left,
            duration,
            individual_id,
            interaction_strength,
            x,
            y,
        }
    }

    // Convert coordinates to [f64; 2] for kd-tree
    pub fn as_point(&self) -> [f64; 2] {
        [self.x, self.y]
    }
}
#[derive(Debug)]
pub struct InteractionLayer {
    entities: Vec<Entity>,
    kd_tree: KdTree<f64, usize, [f64; 2]>, // Specify all three generic parameters
}

impl InteractionLayer {
    pub fn new() -> Self {
        InteractionLayer {
            entities: Vec::new(),
            kd_tree: KdTree::new(2), // 2-dimensional space
        }
    }

    pub fn record_movement_in_interaction_layer(
        &mut self,
        entity: Entity,
    ) {
        // Add the entity to the list (optional, for reference)
        self.entities.push(entity.clone());

        // Add or update entity in kd-tree
        self.kd_tree.add(entity.as_point(), entity.individual_id).unwrap();
    }

    pub fn add_entity_and_record_movement(
        &mut self,
        group_id: usize,
        individual_type: &str,
        time: usize,
        time_left: usize,
        duration: usize,
        individual_id: usize,
        interaction_strength: f64,
        x: f64,
        y: f64,
    ) {
        // Create new entity
        let entity = Entity::new(group_id, individual_type, time, time_left, duration, individual_id, interaction_strength, x, y);

        // Record movement in interaction layer
        self.record_movement_in_interaction_layer(entity);
    }

   // pub fn calculate_interactions(&self, query_x: f64, query_y: f64) {
   //     let query_point = [query_x, query_y];
   //     let nearest_neighbors = self.kd_tree.nearest(&query_point, 10, &squared_euclidean).unwrap();
//
   //     // Process results (entities closest to query_point)
   //     for neighbor in nearest_neighbors {
   //         let entity_id = neighbor.1;
   //         let entity = &self.entities[*entity_id];
   //       //  println!("Entity ID: {}, Coordinates: ({}, {})", entity.individual_id, entity.x, entity.y);
   //     }
   // }
//
    pub fn clear_interaction_layer(&mut self) {
        self.entities.clear();
        self.kd_tree = KdTree::new(2); // Reinitialize the KD-tree
    }

   // Method to iterate over entities in the KD-tree
   pub fn iter_entities<'a>(&'a self) -> impl Iterator<Item = &'a Entity> {
    (0..self.entities.len()).map(move |entity_idx| &self.entities[entity_idx])
}

    /// Query entities within radius `radius` from point `(query_x, query_y)`
    pub fn query_other_entities_in_radius(&self, query_x: f64, query_y: f64, radius: f64, entity_id_to_exclude: usize) -> Vec<(usize, f64)> {
        let query_point = [query_x, query_y];
        let mut results = Vec::new();

        // Iterate over all entities and filter those within radius, excluding self
        for (_, entity_id) in self.kd_tree.iter_nearest(&query_point, &squared_euclidean).unwrap() {
            let entity = &self.entities[*entity_id];
            let distance_squared = squared_euclidean(&entity.as_point(), &query_point);
            let distance = distance_squared.sqrt();
            
            if distance <= radius && entity.individual_id != entity_id_to_exclude {
                results.push((*entity_id, distance));
            }
        }

        results
    }

    /// Remove all entries in the interaction layer where there is no other entity within radius 4
    pub fn remove_entities_without_neighbors(&mut self) {
        let radius = 4.0;
        let mut to_remove = Vec::new();

        for entity in &self.entities {
            let neighbors = self.query_other_entities_in_radius(entity.x, entity.y, radius, entity.individual_id);

            // If no neighbors found within radius, mark for removal
            if neighbors.is_empty() {
                to_remove.push(entity.individual_id);
            }
        }

        // Remove entities from both entities vector and kd-tree
        let mut new_entities = Vec::new();
        for entity in self.entities.drain(..) {
            if !to_remove.contains(&entity.individual_id) {
                new_entities.push(entity.clone());
            }
        }
        self.entities = new_entities;

        // Rebuild kd-tree with remaining entities
        self.kd_tree = KdTree::new(2); // Reinitialize the KD-tree

        for entity in &self.entities {
            self.kd_tree.add(entity.as_point(), entity.individual_id).unwrap();
        }
    }
}




impl Clone for InteractionLayer {
    fn clone(&self) -> Self {
        let mut new_kd_tree = KdTree::new(2);
        for entity in &self.entities {
            new_kd_tree.add(entity.as_point(), entity.individual_id).unwrap();
        }
        InteractionLayer {
            entities: self.entities.clone(),
            kd_tree: new_kd_tree,
        }
    }
}






/*
use crate::*;
use std::collections::HashMap;
use sif_kdtree::KdTree;

//[dependencies]
//sif-kdtree = "0.6.0"

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


type Position = [usize; 3]; // (x, y, time)
type InteractionLayer = KdTree<[usize; 3], PositionedEntity>;

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

#[derive(Debug, Clone)]
pub struct PositionedEntity {
    pub position: Position,
    pub entity: Entity,
}

// Implement AsRef for PositionedEntity to provide the correct type for KdTree
impl AsRef<[usize; 3]> for PositionedEntity {
    fn as_ref(&self) -> &[usize; 3] {
        &self.position
    }
}

// Function to rebuild the KdTree from scratch with all existing entities plus a new one.
fn rebuild_kdtree(existing_entities: &[PositionedEntity], new_entity: PositionedEntity) -> InteractionLayer {
    let mut kdtree = KdTree::new(existing_entities.len() + 1);

    // Add all existing entities to the KdTree
    for entity in existing_entities {
        kdtree.add(entity.as_ref(), entity.clone()).unwrap();
    }

    // Add the new entity to the KdTree
    kdtree.add(new_entity.as_ref(), new_entity).unwrap();

    kdtree
}

// Function to record movement in the interaction layer
pub fn record_movement_in_interaction_layer(
    interaction_layer: &mut InteractionLayer,
    x: usize,
    y: usize,
    time: usize,
    group_id: usize,
    individual_type: &str,
    individual_id: usize,
) {
    let directions = [
        (0, 0, 1.0), // Target cell
        (0, 1, 0.8), (1, 0, 0.8), (0, -1, 0.8), (-1, 0, 0.8), // Immediate neighbors
        (1, 1, 0.8), (1, -1, 0.8), (-1, 1, 0.8), (-1, -1, 0.8), // Diagonal neighbors
        (0, 2, 0.5), (2, 0, 0.5), (0, -2, 0.5), (-2, 0, 0.5), // Next distance neighbors
        (1, 2, 0.5), (2, 1, 0.5), (2, -1, 0.5), (1, -2, 0.5), // Mixed neighbors
        (-1, 2, 0.5), (-2, 1, 0.5), (-2, -1, 0.5), (-1, -2, 0.5), // More mixed neighbors
    ];

    let mut existing_entities: Vec<_> = interaction_layer.iter().map(|(_, v)| v.clone()).collect();

    let mut added_positions = HashSet::new();

    for &(dx, dy, strength) in &directions {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        let position = [nx, ny, time];
        if added_positions.insert(position) {
            let entity = Entity {
                group_id,
                individual_type: individual_type.to_string(),
                time,
                time_left: 0,
                duration: 0,
                individual_id,
                interaction_strength: strength,
            };

            let positioned_entity = PositionedEntity { position, entity };

            // Rebuild the KdTree with all existing entities plus the new one
            *interaction_layer = rebuild_kdtree(&existing_entities, positioned_entity.clone());

            // Update the list of existing entities for the next iteration
            existing_entities.push(positioned_entity);
        }
    }
}

// Function to delete the kdtree
pub fn delete_kdtree(interaction_layer: &mut InteractionLayer) {
    interaction_layer.clear();
}


// A key to represent the position and time
type PositionTimeKey = (usize, usize, usize); // (x, y, time)

// Define your interaction layer type
type InteractionLayer = KdTree<f64, Entity, PositionTimeKey>;

pub fn record_movement_in_interaction_layer(
    interaction_layer: &mut InteractionLayer,
    x: usize,
    y: usize,
    time: usize,
    group_id: usize,
    individual_type: &str,
    individual_id: usize,
) {
    let directions = [
        (0, 0, 1.0), // Target cell
        (0, 1, 0.8), (1, 0, 0.8), (0, -1, 0.8), (-1, 0, 0.8), // Immediate neighbors
        (1, 1, 0.8), (1, -1, 0.8), (-1, 1, 0.8), (-1, -1, 0.8), // Diagonal neighbors
        (0, 2, 0.5), (2, 0, 0.5), (0, -2, 0.5), (-2, 0, 0.5), // Next distance neighbors
        (1, 2, 0.5), (2, 1, 0.5), (2, -1, 0.5), (1, -2, 0.5), // Mixed neighbors
        (-1, 2, 0.5), (-2, 1, 0.5), (-2, -1, 0.5), (-1, -2, 0.5), // More mixed neighbors
        //(0, 3, 0.1), (3, 0, 0.1), (0, -3, 0.1), (-3, 0, 0.1), // Further neighbors
        //(1, 3, 0.1), (3, 1, 0.1), (3, -1, 0.1), (1, -3, 0.1),
        //(-1, 3, 0.1), (-3, 1, 0.1), (-3, -1, 0.1), (-1, -3, 0.1),
        //(2, 2, 0.1), (2, -2, 0.1), (-2, 2, 0.1), (-2, -2, 0.1)
    ];

    let mut added_positions = std::collections::HashSet::new();

    for &(dx, dy, strength) in &directions {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        let key = (nx as f64, ny as f64, time as f64);
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

            // Insert the entity into the k-d tree, avoiding duplicates
            let existing_entities = interaction_layer.within(&key, 0.0);
            if !existing_entities.iter().any(|(_, e)| e.individual_id == individual_id) {
                interaction_layer.insert(key, entity);
            }
        }
    }
}

pub fn remove_entity_from_interaction_layer(
    interaction_layer: &mut InteractionLayer,
    x: usize,
    y: usize,
    time: usize,
    individual_id: usize,
) {
    let key = (x as f64, y as f64, time as f64);

    interaction_layer.remove(&key, |_, entity| entity.individual_id == individual_id);
}

pub fn get_entities_at_position_and_time(
    interaction_layer: &InteractionLayer,
    x: usize,
    y: usize,
    time: usize,
) -> Vec<&Entity> {
    let key = (x as f64, y as f64, time as f64);

    interaction_layer
        .within(&key, 0.0)
        .map(|(_, entity)| entity)
        .collect()
}

pub fn find_nearby_individuals(
    interaction_layer: &InteractionLayer,
    x: f64,
    y: f64,
    radius: f64,
) -> Vec<&Entity> {
    let search_point = (x, y, 0.0); // We're only interested in the spatial dimensions (x, y)

    // Use the `within` method to find entities within the given radius of the search point
    interaction_layer
        .within(&search_point, radius)
        .map(|(_, entity)| entity)
        .collect()
}

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


// function to fully purge the interaction layer
pub fn purge_interaction_layer(interaction_layer: &mut InteractionLayer) {
    interaction_layer.clear();
}




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






