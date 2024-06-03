use crate::*;

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
    pub individuals: Vec<Individual>,
}

#[derive(Debug, Clone)]
pub struct Individual {
    pub group_id: usize,
    pub individual_type: String,
    pub time: usize,
    pub time_left: usize,
    pub duration: usize,
}

impl Default for Individual {
    fn default() -> Self {
        Individual {
            group_id: 0,
            individual_type: "".to_string(),
            time: 0,
            time_left: 0,
            duration: 0,
        }
    }
}


//function to create the interaction layer
pub fn create_interaction_layer(grid: &Vec<Vec<Cell>>) -> Vec<Vec<InteractionCell>> {
    let mut interaction_layer = Vec::new();

    for _ in 0..grid.len() {
        let mut row = Vec::new();
        for _ in 0..grid[0].len() {
            row.push(InteractionCell::default());
        }
        interaction_layer.push(row);
    }

    interaction_layer
}

//function to record the movement of an individual in the interaction layer
pub fn record_movement_in_interaction_layer(interaction_layer: &mut Vec<Vec<InteractionCell>>, x: usize, y: usize, group_id: usize, individual_type: &str, time: usize) {
    interaction_layer[x][y].individuals.push(Individual {
        group_id,
        individual_type: individual_type.to_string(),
        time,
    });
}

//function to record the time an individual left a specific x and y coordinate
pub fn record_time_left_in_interaction_layer(interaction_layer: &mut Vec<Vec<InteractionCell>>, x: usize, y: usize, group_id: usize, individual_type: &str, time: usize) {
    for individual in interaction_layer[x][y].individuals.iter_mut() {
        if individual.group_id == group_id && individual.individual_type == individual_type {
            individual.time_left = time;
            individual.duration = individual.time_left - individual.time;
        }
    }
}

//function to get the number of individuals at a specific x and y coordinate
pub fn get_number_of_individuals_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].individuals.len()
}

//function to get the number of groups at a specific x and y coordinate
pub fn get_number_of_groups_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].individuals.iter().filter(|individual| individual.individual_type == "group").count()
}

//function to get the number of roamer at a specific x and y coordinate

pub fn get_number_of_roamers_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].individuals.iter().filter(|individual| individual.individual_type == "roamer").count()
}

//function to get the number of disperser at a specific x and y coordinate
pub fn get_number_of_dispersers_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].individuals.iter().filter(|individual| individual.individual_type == "disperser").count()
}

//function to get the number of individuals that have been at a specific x and y coordinate
pub fn get_number_of_individuals_that_have_been_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].individuals.iter().count()
}

//function to get the number of groups that have been at a specific x and y coordinate
pub fn get_number_of_groups_that_have_been_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].individuals.iter().filter(|individual| individual.individual_type == "group").count()
}

//function to get the number of roamer that have been at a specific x and y coordinate
pub fn get_number_of_roamers_that_have_been_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].individuals.iter().filter(|individual| individual.individual_type == "roamer").count()
}

//function to get the number of disperser that have been at a specific x and y coordinate
pub fn get_number_of_dispersers_that_have_been_at_coordinate(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize) -> usize {
    interaction_layer[x][y].individuals.iter().filter(|individual| individual.individual_type == "disperser").count()
}

//function to get the number of individuals that have been at a specific x and y coordinate at a specific time
pub fn get_number_of_individuals_that_have_been_at_coordinate_at_time(interaction_layer: &Vec<Vec<InteractionCell>>, x: usize, y: usize, time: usize) -> usize {
    interaction_layer[x][y].individuals.iter().filter(|individual| individual.time <= time && individual.time_left >= time).count()
}


