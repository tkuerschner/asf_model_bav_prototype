use crate::*;
use crate::group_functions::*;


//struct for dispersal with all fields of the group_member struct and new fields x and y coordinates

// Static counter for disperser_id
static mut DISPERSER_COUNTER: usize = 0;

// Function to generate a unique individual_id
pub fn generate_disperser_id() -> usize {
    unsafe {
        DISPERSER_COUNTER += 1;
        DISPERSER_COUNTER
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DispersingIndividual {
    //pub ageclass: AgeClass,
    pub x: usize,
    pub y: usize,
    pub individual_id: usize,
    pub age: u32,
    pub age_class: AgeClass,
    pub sex: Sex,
    pub health_status: HealthStatus, 
    pub time_of_birth: usize,
    pub has_reproduced: bool,
    pub time_of_reproduction: usize,
    pub origin_group_id: usize,
    pub disperser_id: usize,
    pub target_cell:Option<(usize,usize)>,
    pub daily_distance: usize,
}


// dispersal check function, once a female group members ageclass switches to yearling it creates a new instance of 
// dispersing individual with the same fields as the group member and the x and y coordinates of the group the member is then removed from the group
//
//pub fn dispersal_assignment(groups: &mut Vec<Groups>, dispersing_individuals: &mut Vec<DispersingIndividual>) {
//    for group in groups.iter_mut() {
//        let members_to_disperse_indices: Vec<usize> = group
//            .group_members
//            .iter()
//            .enumerate()
//            .filter(|(_, mem)| mem.age_class == AgeClass::Yearling && mem.sex == Sex::Female)
//            .map(|(i, _)| i)
//            .collect();
//
//        for index in members_to_disperse_indices.into_iter() { 
//            let member = &mut group.group_members[index];
//            let dispersing_individual = DispersingIndividual { //create new dispersing individual
//                ageclass: member.age_class.clone(),
//                x: group.x,
//                y: group.y,
//                individual_id: member.individual_id,
//                age: member.age,
//                age_class: member.age_class.clone(),
//                sex: member.sex.clone(),
//                health_status: member.health_status.clone(),
//                time_of_birth: member.time_of_birth,
//                has_reproduced: member.has_reproduced,
//                time_of_reproduction: member.time_of_reproduction,
//                origin_group_id: group.group_id,
//                disperser_id: generate_disperser_id(),
//                target_cell: None,
//                daily_distance: DEFAULT_DAILY_MOVEMENT_DISTANCE,
//            };
//
//            // Remove the member from the group
//            group.group_members.remove(index);
//
//            // Add the dispersing individual to the vector
//            dispersing_individuals.push(dispersing_individual);
//        }
//    }
//}
//
//pub fn assign_dispersal_targets (dispersing_individuals: &mut Vec<DispersingIndividual>, groups: &Vec<Groups>) {
//    let selected_groups = select_dispersal_target(dispersing_individuals, groups);
//    for (i, disperser) in dispersing_individuals.iter_mut().enumerate() {
//        let target_group = groups.iter().find(|group| group.group_id == selected_groups[i]).unwrap();
//        disperser.target_cell = Some((target_group.x, target_group.y));
//    }
//}
//
//
//// function for dispersers to find the closest 4 groups that are not the origin group and return their ids
//pub fn find_closest_groups(dispersing_individuals: &Vec<DispersingIndividual>, groups: &Vec<Groups>) -> Vec<usize> {
//    let mut closest_groups: Vec<usize> = Vec::new();
//    for disperser in dispersing_individuals.iter() {
//        let mut distances: Vec<(usize, usize, usize)> = Vec::new();
//        for group in groups.iter() {
//            if group.group_id != disperser.origin_group_id {
//                let distance = ((disperser.x as isize - group.x as isize).pow(2) + (disperser.y as isize - group.y as isize).pow(2)) as f64;
//                distances.push((group.group_id, disperser.x, disperser.y));
//            }
//        }
//        distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
//        for i in 0..4 {
//            closest_groups.push(distances[i].0);
//        }
//    }
//    closest_groups
//}
//
//// function for disperser to randomly select a group from the closest groups 
//pub fn select_dispersal_target(dispersing_individuals: &Vec<DispersingIndividual>, groups: &Vec<Groups>) -> Vec<usize> {
//    let closest_groups = find_closest_groups(dispersing_individuals, groups);
//    let mut selected_groups: Vec<usize> = Vec::new();
//    for disperser in dispersing_individuals.iter() {
//        let selected_group = rand::thread_rng().gen_range(0..4);
//        selected_groups.push(closest_groups[selected_group]);
//    }
//    selected_groups
//}
//
//// function to to assign the selected groups coordinates as target_cell for the disperser
//pub fn assign_dispersal_target_cell(dispersing_individuals: &mut Vec<DispersingIndividual>, groups: &Vec<Groups>) {
//    let selected_groups = select_dispersal_target(dispersing_individuals, groups);
//    for (i, disperser) in dispersing_individuals.iter_mut().enumerate() {
//        let target_group = groups.iter().find(|group| group.group_id == selected_groups[i]).unwrap();
//        disperser.target_cell = Some((target_group.x, target_group.y));
//    }
//}
//
//// function to move the disperser to the target cell wit 25% chance of moving randomly one cell per step until daily distance is 0
//pub fn move_female_disperser(dispersing_individuals: &mut Vec<DispersingIndividual>, grid: &mut Vec<Vec<Cell>>, groups: &mut Vec<Groups>) { 
//    let grid_width = grid[0].len();
//    let grid_height = grid.len();
//
//    for disperser in dispersing_individuals.iter_mut() {
//        while disperser.daily_distance > 0 {
//            let mut rng = rand::thread_rng();
//            let random_number: f64 = rng.gen();
//            if random_number < 0.25 {
//                let mut new_x = disperser.x;
//                let mut new_y = disperser.y;
//                let random_number: f64 = rng.gen();
//                if random_number < 0.25 {
//                    new_x = new_x + 1;
//                } else if random_number < 0.5 {
//                    new_x = new_x - 1;
//                } else if random_number < 0.75 {
//                    new_y = new_y + 1;
//                } else {
//                    new_y = new_y - 1;
//                }
//                if new_x < grid_width && new_y < grid_height {
//                    disperser.x = new_x;
//                    disperser.y = new_y;
//                    disperser.daily_distance -= 1;
//                }
//            } else {
//                if let Some((target_x, target_y)) = disperser.target_cell {
//                    let mut new_x = disperser.x;
//                    let mut new_y = disperser.y;
//                    if new_x < target_x {
//                        new_x = new_x + 1;
//                    } else if new_x > target_x {
//                        new_x = new_x - 1;
//                    }
//                    if new_y < target_y {
//                        new_y = new_y + 1;
//                    } else if new_y > target_y {
//                        new_y = new_y - 1;
//                    }
//                    if new_x < grid_width && new_y < grid_height {
//                        disperser.x = new_x;
//                        disperser.y = new_y;
//                        disperser.daily_distance -= 1;
//                    }
//                }
//            }
//        }
//    }
//    add_disperser_to_group(dispersing_individuals, groups);
//}
//
//
//
////function to add the disperser to the group at the target cell and remove it from the dispersing individuals vector
//pub fn add_disperser_to_group(dispersing_individuals: &mut Vec<DispersingIndividual>, groups: &mut Vec<Groups>) {
//   for disperser in dispersing_individuals.iter() {
//       let group = groups.iter_mut().find(|group| group.x == disperser.x && group.y == disperser.y).unwrap();
//       group.group_members.push(GroupMember {
//           individual_id: disperser.individual_id,
//           age: disperser.age,
//           age_class: disperser.age_class.clone(),
//           sex: disperser.sex.clone(),
//           health_status: disperser.health_status.clone(),
//           time_of_birth: disperser.time_of_birth,
//           has_reproduced: disperser.has_reproduced,
//           time_of_reproduction: disperser.time_of_reproduction,
//           origin_group_id: disperser.origin_group_id,
//       });
//   }
//   //dispersing_individuals.retain(|disperser| disperser.x != disperser.target_cell.unwrap().0 && disperser.y != disperser.target_cell.unwrap().1);
//   dispersing_individuals.retain(|disperser| {
//    if let Some((target_x, target_y)) = disperser.target_cell {
//        disperser.x != target_x || disperser.y != target_y
//    } else {
//        true // Retain the disperser if target_cell is None
//    }
//});
//}
//
//
////pub fn add_disperser_to_group(disperser: &DispersingIndividual, groups: &mut Vec<Groups>) {
////    let group = groups.iter_mut().find(|group| group.x == disperser.x && group.y == disperser.y).unwrap();
////    group.group_members.push(GroupMember {
////        individual_id: disperser.individual_id,
////        age: disperser.age,
////        age_class: disperser.age_class.clone(),
////        sex: disperser.sex.clone(),
////        health_status: disperser.health_status.clone(),
////        time_of_birth: disperser.time_of_birth,
////        has_reproduced: disperser.has_reproduced,
////        time_of_reproduction: disperser.time_of_reproduction,
////        origin_group_id: disperser.origin_group_id,
////    });
////}   
////


pub fn dispersal_assignment(groups: &mut Vec<Groups>, dispersing_individuals: &mut Vec<DispersingIndividual>) {
    // Iterate over groups and their members to find individuals to disperse
    for group in groups.iter_mut() {
        // Collect indices of group members that need to disperse
        let members_to_disperse_indices: Vec<usize> = group
            .group_members
            .iter()
            .enumerate()
            .filter(|(_, mem)| mem.age_class == AgeClass::Yearling && mem.sex == Sex::Female && !mem.has_dispersed)
            .map(|(i, _)| i)
            .collect();

        // Iterate over indices in reverse order to remove elements safely
        for &index in members_to_disperse_indices.iter().rev() {
            // Remove the member from the group and collect it as a dispersing individual
            let member = group.group_members.remove(index);
            let dispersing_individual = DispersingIndividual {
                //ageClass: member.age_class.clone(),
                x: group.x,
                y: group.y,
                individual_id: member.individual_id,
                age: member.age,
                age_class: member.age_class.clone(),
                sex: member.sex.clone(),
                health_status: member.health_status.clone(),
                time_of_birth: member.time_of_birth,
                has_reproduced: member.has_reproduced,
                time_of_reproduction: member.time_of_reproduction,
                origin_group_id: group.group_id,
                disperser_id: generate_disperser_id(),
                target_cell: None,
                daily_distance: DEFAULT_DAILY_MOVEMENT_DISTANCE,
            };
            // Add the dispersing individual to the dispersing_individuals vector
            dispersing_individuals.push(dispersing_individual);
        }
    }
}



pub fn assign_dispersal_targets(dispersing_individuals: &mut Vec<DispersingIndividual>, groups: &Vec<Groups>) {
    for disperser in dispersing_individuals.iter_mut() {
        // Find the closest groups that are not the origin group
        let closest_groups = find_closest_groups(disperser, groups);

        // Randomly select a target group from the closest groups
        let selected_group_index = rand::thread_rng().gen_range(0..closest_groups.len());
        let selected_group_id = closest_groups[selected_group_index];

        // Find the coordinates of the selected group
        if let Some(selected_group) = groups.iter().find(|group| group.group_id == selected_group_id) {
            // Assign the coordinates of the selected group as the target cell for the disperser
            disperser.target_cell = Some((selected_group.core_cell.unwrap().0, selected_group.core_cell.unwrap().1));
        } else {
            // Handle the case where the selected group cannot be found
            println!("Error: Selected group not found!");
        }
    }
}

// Function to find the closest groups that are not the origin group
fn find_closest_groups(disperser: &DispersingIndividual, groups: &Vec<Groups>) -> Vec<usize> {
    let mut distances: Vec<(usize, usize)> = Vec::new();

    // Calculate the distance to each group that is not the origin group
    for group in groups.iter() {
        if group.group_id != disperser.origin_group_id {
            let distance = calculate_distance(disperser.x, disperser.y, group.x, group.y);
            distances.push((group.group_id, distance));
        }
    }

    // Sort the distances in ascending order
    distances.sort_by_key(|&(_, distance)| distance);

    // Extract the group IDs from the sorted distances
    let closest_groups: Vec<usize> = distances.iter().map(|&(group_id, _)| group_id).collect();

    // Return the closest group IDs
    closest_groups
}

// Function to calculate the Euclidean distance between two points
fn calculate_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let distance_squared = (x2 as isize - x1 as isize).pow(2) + (y2 as isize - y1 as isize).pow(2);
    let distance = (distance_squared as f64).sqrt() as usize;
    distance
}

// Function to move dispersers towards their target cell
pub fn move_female_disperser(dispersing_individuals: &mut Vec<DispersingIndividual>, grid: &mut Vec<Vec<Cell>>, groups: &mut Vec<Groups>) {
    add_dispersers_to_groups(dispersing_individuals, groups);
    for disperser in dispersing_individuals.iter_mut() {
        while disperser.daily_distance > 0 {
            // Randomly decide whether to move towards the target or move randomly
            let move_towards_target = rand::thread_rng().gen_bool(0.25);

            if move_towards_target {
                move_towards_target_cell(disperser, grid);
                if disperser.x == disperser.target_cell.unwrap().0 && disperser.y == disperser.target_cell.unwrap().1 {
                    disperser.daily_distance = 0;
                }
            } else {
                move_randomly(disperser, grid);
                if disperser.x == disperser.target_cell.unwrap().0 && disperser.y == disperser.target_cell.unwrap().1 {
                    disperser.daily_distance = 0;
                }
            }
        }
        disperser.daily_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE;
    }

    // Add dispersers to groups if they have reached their target cell
    
    add_dispersers_to_groups(dispersing_individuals, groups);
}

// Function to move disperser towards its target cell
fn move_towards_target_cell(disperser: &mut DispersingIndividual, grid: &Vec<Vec<Cell>>) {
    if let Some((target_x, target_y)) = disperser.target_cell {
        let dx = (target_x as isize - disperser.x as isize).signum();
        let dy = (target_y as isize - disperser.y as isize).signum();

        let new_x = (disperser.x as isize + dx) as usize;
        let new_y = (disperser.y as isize + dy) as usize;

        // Update disperser's position if within grid boundaries
        if new_x < grid.len() && new_y < grid[0].len() && is_valid_cell(grid, new_x, new_y) {
            disperser.x = new_x;
            disperser.y = new_y;
            disperser.daily_distance -= 1;
        }
    }
}

// Function to move disperser randomly within the grid
fn move_randomly(disperser: &mut DispersingIndividual, grid: &Vec<Vec<Cell>>) {
    let dx = rand::thread_rng().gen_range(-1..=1);
    let dy = rand::thread_rng().gen_range(-1..=1);

    let new_x = (disperser.x as isize + dx) as usize;
    let new_y = (disperser.y as isize + dy) as usize;

    // Update disperser's position if within grid boundaries
    if new_x < grid.len() && new_y < grid[0].len() && is_valid_cell(grid, new_x, new_y) {
        disperser.x = new_x;
        disperser.y = new_y;
        disperser.daily_distance -= 1;
    }
}

// Function to add dispersers to groups if they have reached their target cell
fn add_dispersers_to_groups(dispersing_individuals: &mut Vec<DispersingIndividual>, groups: &mut Vec<Groups>) {
    let mut index = 0;
    while index < dispersing_individuals.len() {
        let disperser = &dispersing_individuals[index];

        // Check if disperser has reached its target cell
        if let Some((target_x, target_y)) = disperser.target_cell {
            if disperser.x == target_x && disperser.y == target_y {
                // Find the group corresponding to the target cell
                if let Some(target_group) = groups.iter_mut().find(|group| group.core_cell.unwrap().0 == target_x && group.core_cell.unwrap().1 == target_y) {
                    // Add disperser as a group member
                    //println!("Adding disperser to group");
                    let new_group_member = GroupMember {
                        individual_id: disperser.individual_id,
                        age: disperser.age,
                        age_class: disperser.age_class.clone(),
                        sex: disperser.sex.clone(),
                        health_status: disperser.health_status.clone(),
                        time_of_birth: disperser.time_of_birth,
                        has_reproduced: disperser.has_reproduced,
                        time_of_reproduction: disperser.time_of_reproduction,
                        origin_group_id: disperser.origin_group_id,
                        has_dispersed: true,
                        current_group_id: target_group.group_id,
                    };
                    target_group.group_members.push(new_group_member);

                    // Remove disperser from dispersing individuals
                    dispersing_individuals.remove(index);
                    continue; // Continue to next disperser without incrementing index
                }
            }
        }
        // Move to next disperser
        index += 1;
    }
}

pub fn form_new_group(grid: &mut Vec<Vec<Cell>>,x: usize, y: usize, groups: &mut Vec<Groups>) {
    //form a new group at the disperser's current location

   add_new_group_at_location(groups, grid, x, y);


}
























