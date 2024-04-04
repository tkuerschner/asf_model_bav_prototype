use crate::*;
use crate::group_functions::*;
use std::collections::HashMap;

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
#[derive(Debug, Clone, PartialEq)]
pub struct DispersingFemaleGroup {
    pub dispersing_individuals: Vec<DispersingIndividual>,
    pub disp_grp_id: usize,
    pub target_cell: Option<(usize, usize)>,
    pub daily_distance: usize,
    pub disp_grp_x: usize,
    pub disp_grp_y: usize,
}


//pub fn dispersal_assignment(groups: &mut Vec<Groups>, dispersing_individuals: &mut Vec<DispersingIndividual>, dispersing_groups: &mut Vec<DispersingFemaleGroup>) {
//    // Iterate over groups and their members to find individuals to disperse
//    for group in groups.iter_mut() {
//        // Collect indices of group members that need to disperse
//        let members_to_disperse_indices: Vec<usize> = group
//            .group_members
//            .iter()
//            .enumerate()
//            .filter(|(_, mem)| mem.age_class == AgeClass::Yearling && mem.sex == Sex::Female && !mem.has_dispersed)
//            .map(|(i, _)| i)
//            .collect();
//
//        // Iterate over indices in reverse order to remove elements safely
//        for &index in members_to_disperse_indices.iter().rev() {
//            // Remove the member from the group and collect it as a dispersing individual
//            let member = group.group_members.remove(index);
//            let dispersing_individual = DispersingIndividual {
//                //ageClass: member.age_class.clone(),
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
//            // Add the dispersing individual to the dispersing_individuals vector
//            dispersing_individuals.push(dispersing_individual);
//
//            // if the number of instances in dispersing_individuals is above 2 then create a new instance of DispersingFemaleGroup and put those in there
//            if dispersing_individuals.len() >= 2 {
//                let dispersing_group = DispersingFemaleGroup {
//                    dispersing_individuals: dispersing_individuals.clone(),
//                    disp_grp_id: generate_disperser_id(),
//                    target_cell: None,
//                    daily_distance: DEFAULT_DAILY_MOVEMENT_DISTANCE,
//                    disp_grp_x: group.x,
//                    disp_grp_y: group.y,
//                };
//                dispersing_groups.push(dispersing_group);
//
//                // Clear the dispersing_individuals vector
//                dispersing_individuals.clear();
//            } 
//        }
//    }
//}

pub fn dispersal_assignment(groups: &mut Vec<Groups>, dispersing_individuals: &mut Vec<DispersingIndividual>, dispersing_groups: &mut Vec<DispersingFemaleGroup>) {
    let mut remaining_individuals = Vec::new(); // Store individuals that couldn't be dispersed

    // Create a map to store dispersing individuals by their origin group ID
    let mut dispersing_by_group: HashMap<u64, Vec<DispersingIndividual>> = HashMap::new();

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

        //print the number of dispersers if there is more then 0
        //if members_to_disperse_indices.len() > 0 {
           // println!("Number of dispersers: {}", members_to_disperse_indices.len());
       // }

        

        // Iterate over indices in reverse order to remove elements safely
        for &index in members_to_disperse_indices.iter().rev() {
            // Remove the member from the group and collect it as a dispersing individual
            let member = group.group_members.remove(index);
            let dispersing_individual = DispersingIndividual {
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
            // Add the dispersing individual to the dispersing_by_group map
            let dispersing_group = dispersing_by_group.entry(group.group_id as u64).or_insert_with(Vec::new);
            dispersing_group.push(dispersing_individual.clone());
        }
    }

    // Iterate over dispersing_by_group to create DispersingFemaleGroup instances as needed
    for (group_id, dispersing_individuals) in dispersing_by_group {
        // If there are at least two individuals, create a dispersing group
        if dispersing_individuals.len() >= 2 {
            let dispersing_group = DispersingFemaleGroup {
                dispersing_individuals: dispersing_individuals.clone(),
                disp_grp_id: generate_disperser_id(),
                target_cell: None,
                daily_distance: DEFAULT_DAILY_MOVEMENT_DISTANCE,
                disp_grp_x: dispersing_individuals[0].x, // Use x of the first individual
                disp_grp_y: dispersing_individuals[0].y, // Use y of the first individual
            };
            dispersing_groups.push(dispersing_group);
           // println!("Dispersing group created");
        } else {
             // If there's only one dispersing individual, merge it back into its original group
    if let Some(group) = groups.iter_mut().find(|g| g.group_id == group_id as usize) {
        for dispersing_individual in dispersing_individuals {
            // Convert dispersing individual back to GroupMember
            let group_member = GroupMember {
                individual_id: dispersing_individual.individual_id,
                age: dispersing_individual.age,
                age_class: dispersing_individual.age_class.clone(),
                sex: dispersing_individual.sex.clone(),
                health_status: dispersing_individual.health_status.clone(),
                time_of_birth: dispersing_individual.time_of_birth,
                has_reproduced: dispersing_individual.has_reproduced,
                time_of_reproduction: dispersing_individual.time_of_reproduction,
                origin_group_id: dispersing_individual.origin_group_id,
                has_dispersed: false, // Mark as dispersed
                current_group_id: group_id as usize, // Update current group ID
            };
            group.group_members.push(group_member);
           // println!("Dispersing individual merged back into original group");
        }
    } else {
        // Couldn't find the original group, add to remaining_individuals
        remaining_individuals.extend(dispersing_individuals);
    }
        }
    }

    // Update dispersing_individuals with the remaining individuals
    dispersing_individuals.extend(remaining_individuals);
}

pub fn assign_dispersal_targets_individuals(dispersing_individuals: &mut Vec<DispersingIndividual>, groups: &Vec<Groups>) {
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

pub fn assign_dispersal_targets_groups(dispersing_groups: &mut Vec<DispersingFemaleGroup>, groups: &mut Vec<Groups>, grid: &Vec<Vec<Cell>>, rng: &mut impl Rng) {
   // let mut groups_to_remove = Vec::new(); // Vector to store indices of groups to remove
    let mut indices_to_remove = Vec::new();
    for (index, dispersing_group) in dispersing_groups.iter_mut().enumerate() {
        // Find the closest groups that are not the origin group
     //   let closest_groups = find_closest_groups(&dispersing_group.dispersing_individuals[0], groups);
//
     //   // Randomly select a target group from the closest groups
     //   let selected_group_index = rand::thread_rng().gen_range(0..closest_groups.len());
     //   let selected_group_id = closest_groups[selected_group_index];
//
     //   // Find the coordinates of the selected group
     //   if let Some(selected_group) = groups.iter().find(|group| group.group_id == selected_group_id) {
     //       // Assign the coordinates of the selected group as the target cell for each dispersing individual in the group
     //       for disperser in &mut dispersing_group.dispersing_individuals {
     //           disperser.target_cell = Some((selected_group.core_cell.unwrap().0, selected_group.core_cell.unwrap().1));
     //       }
     //   } else {
     //       // Handle the case where the selected group cannot be found
     //       println!("Error: Selected group not found!");
     //   }

        if dispersing_group.target_cell.is_none() {
            let mut target_cell = select_random_free_cell_in_range(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, rng, groups);
            
            let mut ptc = 0;
            while !check_surrounding(grid, target_cell.0, target_cell.1, 100) && ptc < 10{ // check 100 cells around the target cell if they are taken
                //println!("Target cell is isolated, looking for new target cell");
                //println!("Target cell: {:?}", target_cell);
                //while check_surrounding(grid, target_cell.0, target_cell.1, 100) {
                //    target_cell = select_random_free_cell_in_range(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, rng, groups);
                //}

                

                target_cell = select_random_free_cell_in_range(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, rng, groups);
                ptc += 1;
            }
           // if check_if_cell_is_isolated(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, dispersing_group.disp_grp_id){
           //     println!("Cell is isolated, looking for new target cell");
           //     while check_if_cell_is_isolated(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, dispersing_group.disp_grp_id) {
           //       //  println!("Cell is isolated");
           //         target_cell = select_random_free_cell_in_range(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, rng, groups);
           //     }
           // }
           //

           if !check_surrounding(grid, target_cell.0, target_cell.1, 100)
           {
                println!("Unable to find a suitable target cell, merging dispersing group back");
                // function that tries to merge a dispersing group back into its oringal group if the original groups groups size is below the maximum group size, otherwise the individuals in the dispersing group die
                merge_dispersing_group_back_to_origin(dispersing_group, groups);
                indices_to_remove.push(index);
               // groups_to_remove.push(dispersing_group);
               // continue;
                //delete this group
               //let index = groups.iter().position(|group| group.group_id == dispersing_group.disp_grp_id);
               //if let Some(index) = index {
               //    groups.remove(index);
               //}
            

                
           }else{
           
           // for disperser in &mut dispersing_group.dispersing_individuals {
           //     disperser.target_cell = Some(target_cell);
           // }
            dispersing_group.target_cell = Some(target_cell);
           }

          // if circular_bfs_dummy(grid, target_cell.0, target_cell.1, 1600) < 1000 {
          //  //println!("number of possible cells to low");
          //  merge_dispersing_group_back_to_origin(dispersing_group, groups);
          //  return;
          //  }


        }
    }

    // delete all entries of groups_to_remove from dispersing_groups: &mut Vec<DispersingFemaleGroup>
    // Delete dispersing groups based on disp_grp_id
    
    for &index in indices_to_remove.iter().rev() {
        dispersing_groups.remove(index);
    }
   
  }

fn merge_dispersing_group_back_to_origin(dispersing_group: &mut DispersingFemaleGroup, groups: &mut Vec<Groups>) {
    let origin_group_id = dispersing_group.dispersing_individuals[0].origin_group_id;
    if let Some(origin_group) = groups.iter_mut().find(|group| group.group_id == origin_group_id) {
        // Check if the origin group's size is below the maximum group size
       // if origin_group.max_size < count_group_members(origin_group) + count_dispersers_in_disperser_group(dispersing_group) {
            // Merge the dispersing group back into the origin group
            for disperser in &mut dispersing_group.dispersing_individuals {
                let group_member = GroupMember {
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
                        current_group_id: origin_group_id,
                };
                origin_group.group_members.push(group_member);

            }
      //  } else {
      //      // Handle the case where the origin group's size is already at the maximum
      //      println!("Error: Origin group size is already at the maximum! Individuals in the dispersing group will die!");
      //  }
    } else {
        // Handle the case where the origin group cannot be found
        println!("Error: Origin group not found!");
    }

    // delete the dispersing group
  //  let index = groups.iter().position(|group| group.group_id == dispersing_group.disp_grp_id);
  //  if let Some(index) = index {
  //      groups.remove(index);
  //  }
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

//pub fn move_female_disperser_group2(dispersing_group: &mut Vec<DispersingFemaleGroup>, grid: &mut Vec<Vec<Cell>>, groups: &mut Vec<Groups>) {
//  
//    for disperser_group in dispersing_group.iter_mut() {
//
//        let new_group_id: usize;
//      
//                // check if grid cell at location of individual 1 in female disperser group == target cell
//                if let Some((target_x, target_y)) = disperser_group.target_cell {
//                    if grid[target_x][target_y].x_grid == disperser_group.target_cell.unwrap().0 && grid[target_x][target_y].y_grid == disperser_group.target_cell.unwrap().1{
//                        // call add_new_group_at_location
//                        add_new_group_at_location(groups, grid, target_x, target_y);
//                        // get the group id of the just created group
//                        new_group_id = groups.last().unwrap().group_id;
//                    
//                
//
//                // for each individual in dispersing group create a group member and add to group with the values copied over from the dispersing individual
//                for disperser in &mut disperser_group.dispersing_individuals {
//                    let new_group_member = GroupMember {
//                        individual_id: disperser.individual_id,
//                        age: disperser.age,
//                        age_class: disperser.age_class.clone(),
//                        sex: disperser.sex.clone(),
//                        health_status: disperser.health_status.clone(),
//                        time_of_birth: disperser.time_of_birth,
//                        has_reproduced: disperser.has_reproduced,
//                        time_of_reproduction: disperser.time_of_reproduction,
//                        origin_group_id: disperser.origin_group_id,
//                        has_dispersed: true,
//                        current_group_id: disperser.origin_group_id,
//                    };
//                    
//                    groups[new_group_id].group_members.push(new_group_member);
//                    }
//                   
//                 }         
//
//            }
//
//        }
//
//
//
//        //while disperser.daily_distance > 0 {
//        //    // Randomly decide whether to move towards the target or move randomly
//        //    let move_towards_target = rand::thread_rng().gen_bool(0.25);
////
//        //    if move_towards_target {
//        //        move_towards_target_cell(disperser, grid);
//        //        if disperser.x == disperser.target_cell.unwrap().0 && disperser.y == disperser.target_cell.unwrap().1 {
//        //            disperser.daily_distance = 0;
//        //        }
//        //    } else {
//        //        move_randomly(disperser, grid);
//        //        if disperser.x == disperser.target_cell.unwrap().0 && disperser.y == disperser.target_cell.unwrap().1 {
//        //            disperser.daily_distance = 0;
//        //        }
//        //    }
//        //}
//        //disperser.daily_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE;
//    }

    pub fn move_female_disperser_group(dispersing_groups: &mut Vec<DispersingFemaleGroup>, grid: &mut Vec<Vec<Cell>>, groups: &mut Vec<Groups>, rng: &mut impl Rng) {
        let mut groups_to_remove = Vec::new(); // Vector to store indices of groups to remove
        for (index, disperser_group) in dispersing_groups.iter_mut().enumerate() {
            let mut reached_target = false;

            while disperser_group.daily_distance > 0 && !reached_target {
                // Randomly decide whether to move towards the target or move randomly
                let move_towards_target = rand::thread_rng().gen_bool(0.25);

                if move_towards_target {
                    move_towards_target_cell_group(disperser_group, grid);
                    if disperser_group.disp_grp_x == disperser_group.target_cell.unwrap().0 && disperser_group.disp_grp_y == disperser_group.target_cell.unwrap().1 {
                        reached_target = true;
                       // println!("disperser reached target tw");
                        break;
                    }
                } else {
                    move_randomly_group(disperser_group, grid);
                    if disperser_group.disp_grp_x == disperser_group.target_cell.unwrap().0 && disperser_group.disp_grp_y == disperser_group.target_cell.unwrap().1 {
                        reached_target = true;
                       // println!("disperser reached target rw");
                        break;
                    }
                }

                if disperser_group.disp_grp_x == disperser_group.target_cell.unwrap().0 && disperser_group.disp_grp_y== disperser_group.target_cell.unwrap().1 {
                    reached_target = true;
                  //  println!("disperser reached target");
                    break; // Exit the loop if one disperser reached the target
                }

                 // Decrement daily distance
                 if disperser_group.daily_distance > 0 {
                    disperser_group.daily_distance -= 1;
                }
            }
            
            if !reached_target {
                disperser_group.daily_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE;
            }

            if reached_target {

                if !is_valid_territory(grid, disperser_group.target_cell.unwrap().0, disperser_group.target_cell.unwrap().1, 1600) {
                    reached_target = false;
                    redraw_dispersal_target(disperser_group, grid, rng,groups );
                    
                    break;
                }

                let (target_x, target_y) = disperser_group.target_cell.unwrap();
                // Call add_new_group_at_location
                let new_group_id: usize;

                add_new_group_at_location(groups, grid, target_x, target_y);
                new_group_id = groups.last().unwrap().group_id;
                make_core_cell_an_ap(grid, groups.last().unwrap().core_cell.unwrap().0, groups.last().unwrap().core_cell.unwrap().1);
                place_attraction_points_in_territory(grid, new_group_id, 8, rng);
                remove_ap_on_cells_with_quality_0(grid);
                // For each individual in dispersing group, create a group member and add to group
                for disperser in &mut disperser_group.dispersing_individuals {
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
                        current_group_id: new_group_id,
                    };
                    groups[new_group_id - 1].group_members.push(new_group_member);
                }
                // Add index to groups_to_remove vector
                 groups_to_remove.push(index);
            }
        }
      //println!("Groups to remove: {:?}", groups_to_remove);
      groups_to_remove.sort_unstable_by(|a, b| b.cmp(a));
      for &index in groups_to_remove.iter().rev() {
        //println!("Removing group at index {}", index);
        if index < dispersing_groups.len() {
            dispersing_groups.remove(index);
        } else {
            println!("Index {} is out of bounds (length: {})", index , dispersing_groups.len());
        }
    }

       // println!("Remaining dispersing groups: {:?}", dispersing_groups);
    }

    fn move_towards_target_cell_group(disperser_group: &mut DispersingFemaleGroup, grid: &Vec<Vec<Cell>>) {
        if let Some((target_x, target_y)) = disperser_group.target_cell {
            let dx = (target_x as isize - disperser_group.disp_grp_x as isize).signum();
            let dy = (target_y as isize - disperser_group.disp_grp_y as isize).signum();

            let new_x = (disperser_group.disp_grp_x as isize + dx) as usize;
            let new_y = (disperser_group.disp_grp_y as isize + dy) as usize;

            // Update disperser's position if within grid boundaries
            if new_x < grid.len() && new_y < grid[0].len() && is_valid_cell(grid, new_x, new_y) {
                disperser_group.disp_grp_x = new_x;
                disperser_group.disp_grp_y = new_y;
                disperser_group.daily_distance -= 1;
            }
        }
    }

    fn move_randomly_group(disperser_group: &mut DispersingFemaleGroup, grid: &Vec<Vec<Cell>>) {
        let dx = rand::thread_rng().gen_range(-1..=1);
        let dy = rand::thread_rng().gen_range(-1..=1);

        let new_x = (disperser_group.disp_grp_x as isize + dx) as usize;
        let new_y = (disperser_group.disp_grp_y as isize + dy) as usize;

        // Update disperser's position if within grid boundaries
        if new_x < grid.len() && new_y < grid[0].len() && is_valid_cell(grid, new_x, new_y) {
            disperser_group.disp_grp_x = new_x;
            disperser_group.disp_grp_y = new_y;
            disperser_group.daily_distance -= 1;
        }
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


pub fn redraw_dispersal_target(dispersing_group: &mut DispersingFemaleGroup, grid: &mut Vec<Vec<Cell>>, rng: &mut impl Rng, groups: &mut Vec<Groups>) {

    let mut target_cell = select_random_free_cell_in_range(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, rng, groups);
    let mut ptc = 0;
    while !check_surrounding(grid, target_cell.0, target_cell.1, 100) && ptc < 10{ // check 100 cells around the target cell if they are taken
        //println!("Target cell is isolated, looking for new target cell");
        //println!("Target cell: {:?}", target_cell);
        target_cell = select_random_free_cell_in_range(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, rng, groups);
        ptc += 1;
    }
    if !check_surrounding(grid, target_cell.0, target_cell.1, 100)
    {
        println!("Unable to find a suitable target cell, merging dispersing group");
        // function that tries to merge a dispersing group back into its oringal group if the original groups groups size is below the maximum group size, otherwise the individuals in the dispersing group die
        merge_dispersing_group_back_to_origin(dispersing_group, groups);
        return;
       //  println!("Unable to find a suitable target cell, killing dispersing group");
      // // delete this disperser group without a merge attempt
      // let index = groups.iter().position(|group| group.group_id == dispersing_group.disp_grp_id);
      // if let Some(index) = index {
      //     groups.remove(index);
      // }
       // return;

    }else{
     dispersing_group.target_cell = Some(target_cell);
    }
}




pub fn redraw_and_remove_dispersal_groups(dispersing_groups: &mut Vec<DispersingFemaleGroup>, grid: &mut Vec<Vec<Cell>>, rng: &mut impl Rng, groups: &mut Vec<Groups>) {
    let mut indices_to_remove = Vec::new(); // Vector to store indices of groups to remove
    for (index, dispersing_group) in dispersing_groups.iter_mut().enumerate() {
        redraw_dispersal_target(dispersing_group, grid, rng, groups);

        if dispersing_group.target_cell.is_none() {
            indices_to_remove.push(index);
        }
    }

    // Remove dispersing groups with no target cell assigned
    for &index in indices_to_remove.iter().rev() {
        dispersing_groups.remove(index);
    }
}












