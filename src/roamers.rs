//use std::vec;

use crate::*;

// Static counter for roamer_id
static mut ROAMER_COUNTER: usize = 0;

// Function to generate a unique individual_id
pub fn generate_roamer_id() -> usize {
    unsafe {
        ROAMER_COUNTER += 1;
        ROAMER_COUNTER
    }
}
// Roamer struct
#[derive(Debug, Clone, PartialEq)]
pub struct RoamingIndividual {
    pub roamer_id: usize,
    pub roamer_y: usize,
    pub roamer_x: usize,
    pub individual_id: usize,
    pub age: u32,
    pub age_class: AgeClass,
    pub sex: Sex,
    pub health_status: HealthStatus, 
    pub time_of_birth: usize,
    pub has_reproduced: bool,
    pub time_of_reproduction: usize,
    pub origin_group_id: usize,
    pub target_cell:Option<(usize,usize)>,
    pub daily_distance: usize,
    pub target_group: Option<usize>,
    pub known_groups: Vec<usize>,
    pub initial_dispersal: bool,
    pub target_group_id: Option<usize>,
    pub reached_target: bool,
    pub stay_time: usize,
    pub staying_with_target_group: bool,
    pub infection_stage: InfectionStage,
    pub time_of_infection: Option<usize>,
}

impl RoamingIndividual {

    pub fn roamer_is_infected(&self) -> bool {
        self.health_status == HealthStatus::Infected
    }


}

// Function to assign dispersers to groups
pub fn roamer_assignemnt(roamers: &mut Vec<RoamingIndividual>, groups: &mut Vec<Groups>) {
    
    for group in groups.iter_mut() {

        // Collect indices of members to be dispersed
        let members_to_roam_indices: Vec<usize> = group
            .group_members
            .iter()
            .enumerate()
            .filter(|(_, mem)| (mem.age_class == AgeClass::Yearling || mem.age_class == AgeClass::Adult) && mem.sex == Sex::Male && !mem.has_dispersed)
            .map(|(i, _)| i)
            .collect();

          // Iterate over indices in reverse order to remove elements safely
          for &index in members_to_roam_indices.iter().rev() {
            // Remove the member from the group and collect it as a dispersing individual
            let member = group.group_members.remove(index);
            let roamer = RoamingIndividual {
                roamer_id: generate_roamer_id(),
                roamer_x: group.x,
                roamer_y: group.y,
                individual_id: member.individual_id,
                age: member.age,
                age_class: member.age_class.clone(),
                sex: member.sex.clone(),
                health_status: member.health_status.clone(),
                time_of_birth: member.time_of_birth,
                has_reproduced: member.has_reproduced,
                time_of_reproduction: member.time_of_reproduction,
                origin_group_id: group.group_id,
                target_cell: None,
                daily_distance: DEFAULT_DAILY_MOVEMENT_DISTANCE,
                target_group: None,
                known_groups: Vec::new(),
                initial_dispersal: true,
                target_group_id: None,
                reached_target: false,
                stay_time: 5 + rand::thread_rng().gen_range(0..10), // Random stay time between 5 and 15 days
                staying_with_target_group: false,
                infection_stage: InfectionStage::NotInfected,
                time_of_infection: None,
            };
           
              roamers.push(roamer);
        }
    }
    
    // if there are more then 2 roamers in a group, add each roamer into the roamers vector and remove theese individuals from their group
    //for (group_id, roamers) in roaming_by_group {
    //    if roamers.len() > 2 {
    //        // Clone roamers to avoid borrowing issues
    //        let mut cloned_roamers = roamers.clone();
    //        for roamer in cloned_roamers.iter_mut() {
    //            roamers.push(roamer.clone());
    //            let group = groups.iter_mut().find(|g| g.group_id == roamer.origin_group_id).unwrap();
    //            let index = group.group_members.iter().position(|m| m.individual_id == roamer.individual_id).unwrap();
    //            group.group_members.remove(index);
    //        }
    //    }
    //    // if there is only one individual, merge it back into its origin group
    //    else if roamers.len() == 1 {
    //        let roamer = roamers.pop().unwrap();
    //        let group = groups.iter_mut().find(|g| g.group_id == roamer.origin_group_id).unwrap();
    //        group.group_members.push(GroupMember {
    //            individual_id: roamer.individual_id,
    //            age: roamer.age,
    //            age_class: roamer.age_class.clone(),
    //            sex: roamer.sex.clone(),
    //            health_status: roamer.health_status.clone(),
    //            time_of_birth: roamer.time_of_birth,
    //            has_reproduced: roamer.has_reproduced,
    //            time_of_reproduction: roamer.time_of_reproduction,
    //            origin_group_id: roamer.origin_group_id,
    //            has_dispersed: false, // Mark as dispersed
    //            current_group_id: group_id as usize, // Update current group ID
    //        });
    //    }
    //}
}

pub fn initial_roamer_dispersal_target(roamers: &mut Vec<RoamingIndividual>, grid: &Vec<Vec<Cell>>, rng: &mut impl Rng) {
    for roamer in roamers.iter_mut().filter(|roamer| roamer.initial_dispersal == true){
       // if roamer.initial_dispersal == true {
       //select a target cell that is at least 100 cells away from any cell belonging to this roamers origin group and is_valid_cell using the grid_functions.rs
       // let distance = 0;
        while roamer.target_cell.is_none() {
            //log::info!("Roamer {:?} has no target cell, assigning target", roamer.roamer_id);
         //   //use the random valid cell function to get a random cell
         //   let mut r_cell = random_valid_cell(grid, rng);
         //   let my_group = roamer.origin_group_id;

         //   while grid[r_cell.0][r_cell.1].territory.taken_by_group != my_group {
         //       r_cell = random_valid_cell(grid, rng);
         //   }
         //   
         //   roamer.target_cell = Some(r_cell);
         //   //roamer.target_cell.unwrap().0 = r_cell.0;
         //   //roamer.target_cell.unwrap().1 = r_cell.1;
         //   //check if the cell is at least 100 cells away from any cell belonging to this roamers origin group
         //   //let mut valid = true;

         //   //put all cells around the target cell in a 100 cell radius into a vector
         //   let mut cells_in_radius = Vec::new();
         //   for i in 0..100 {
         //       for j in 0..100 {
         //           cells_in_radius.push((r_cell.0 + i, r_cell.1 + j));
         //           cells_in_radius.push((r_cell.0 - i, r_cell.1 - j));
         //           cells_in_radius.push((r_cell.0 + i, r_cell.1 - j));
         //           cells_in_radius.push((r_cell.0 - i, r_cell.1 + j));
         //       }
         //   }
         //            
         //   // as long as there is a cell in the vector taken by the same group as the roamer, generate a new target cell
         //   while cells_in_radius.iter().any(|(x, y)| grid[*x][*y].territory.taken_by_group == my_group) {
         //       let new_target_cell = random_valid_cell(grid, rng);
         //       roamer.target_cell = Some(new_target_cell);
            // select a random valid cell that is not owned by the origin group
            let target_cell = random_valid_cell(grid, rng);
            roamer.target_cell = Some(target_cell);
            //log::info!("Roamer {:?} has target cell {:?}", roamer.roamer_id, roamer.target_cell.unwrap());
            break;
            }
           
       // }
    }
 }
//}

pub fn initial_roamer_dispersal_movement(model: &mut Model ,  rng: &mut impl Rng, time: usize) {

    let mut vec_ids_hunted_roamers: Vec<u32> = Vec::new(); 
    for roamer in model.roamers.iter_mut().filter(|roamer| roamer.initial_dispersal == true) {
        let mut ptt = 0;
        let this_roamer_id = roamer.roamer_id;
        let mut hunted = false;
        while roamer.daily_distance > 0 && roamer.initial_dispersal == true && ptt < 1000{
            let move_towards_target = rand::thread_rng().gen_bool(0.25);

            //log::info!("Dispersing roamer {:?} is moving towards target: {:?}", roamer.roamer_id, move_towards_target);

            if move_towards_target {
                move_towards_target_cell_roamer(roamer, &model.grid, &mut model.interaction_layer);

                if hunting_check(&mut model.grid,&mut model.high_seats, rng, roamer.roamer_x, roamer.roamer_y) {
                    
                    model.hunting_statistics.add_hunted_individual(roamer.roamer_x, roamer.roamer_y, roamer.sex.clone(), roamer.age, roamer.age_class, roamer.individual_id, Some(roamer.origin_group_id), IndividualType::Roamer, model.global_variables.current_time);
                    hunted = true;
                    //prepare this roamer for removal and end movement loop
                    break;

                }



              //  record_movement_in_interaction_layer_for_roamers(i_layer, roamer.roamer_x, roamer.roamer_y, time, roamer.origin_group_id,  "roamer", roamer.roamer_id);
              model.interaction_layer.add_entity_and_record_movement(
                roamer.origin_group_id,
                "roamer",
                time,
                0, // Assuming time_left is not used
                0, // Assuming duration is not used
                roamer.individual_id,
                1.0, // Assuming interaction_strength is default
                roamer.roamer_x as f64, // Convert coordinates to f64 if necessary
                roamer.roamer_y as f64,  // Convert coordinates to f64 if necessary
                roamer.roamer_is_infected(),
                roamer.infection_stage.clone(),
            );
                if let Some((target_x, target_y)) = roamer.target_cell {
                    if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                        // Roamer reached target
                       // log::info!("Roamer {:?} reached target", roamer.roamer_id);                        
                        roamer.initial_dispersal = false;
                        //set_list_of_target_groups(roamer, groups);
                        get_3_groups_in_range(roamer, &model.groups);
                        select_target_group(roamer, rng);
                        roamer.target_group_id = roamer.target_group;
                        evaluate_and_set_target_cell(roamer, &model.groups);
                        break;
                    }
                }
            } else {
                move_randomly_roamer(roamer, &model.grid);
                if hunting_check(&mut model.grid,&mut model.high_seats, rng, roamer.roamer_x, roamer.roamer_y) {
                    
                    model.hunting_statistics.add_hunted_individual(roamer.roamer_x, roamer.roamer_y, roamer.sex.clone(), roamer.age, roamer.age_class, roamer.individual_id, Some(roamer.origin_group_id), IndividualType::Roamer, model.global_variables.current_time);
                    hunted = true;
                    //prepare this roamer for removal and end movement loop
                    break;

                }
              //  record_movement_in_interaction_layer_for_roamers(i_layer, roamer.roamer_x, roamer.roamer_y, time, roamer.origin_group_id,  "roamer", roamer.roamer_id);
              model.interaction_layer.add_entity_and_record_movement(
                roamer.origin_group_id,
                "roamer",
                time,
                0, // Assuming time_left is not used
                0, // Assuming duration is not used
                roamer.individual_id,
                1.0, // Assuming interaction_strength is default
                roamer.roamer_x as f64, // Convert coordinates to f64 if necessary
                roamer.roamer_y as f64,  // Convert coordinates to f64 if necessary
                roamer.roamer_is_infected(),
                roamer.infection_stage.clone(),
            );
                if let Some((target_x, target_y)) = roamer.target_cell {
                    if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                        // Roamer reached target
                       // log::info!("Roamer {:?} reached target", roamer.roamer_id);
                        roamer.initial_dispersal = false;
                         //set_list_of_target_groups(roamer, groups);
                         get_3_groups_in_range(roamer, &model.groups);
                        select_target_group(roamer, rng);
                        roamer.target_group_id = roamer.target_group;
                        evaluate_and_set_target_cell(roamer, &model.groups);
                        break;
                    }
                }
            }
            ptt += 1;
        }

        if hunted {
            vec_ids_hunted_roamers.push(this_roamer_id as u32);
        }



        if ptt == 1000 {
            log::info!("Roamer {:?} movement loop timeout", roamer.roamer_id);
        }
        //log::info!("Dispersing roamer {:?} finished moving towards target for today", roamer.roamer_id);
        if roamer.initial_dispersal == true {
            roamer.daily_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE;
        }
    }
    // remove hunted roamers
    // take all the roamer id from the vec and remove them from the roamers vector
    for id in vec_ids_hunted_roamers.iter() {
        let index = model.roamers.iter().position(|r| r.roamer_id == *id as usize).unwrap();
        model.roamers.remove(index);
    }
}

//fn move_towards_target_cell_roamer(roamer: &mut RoamingIndividual, grid: &Vec<Vec<Cell>>, //i_layer: &mut InteractionLayer) {
//    if let Some((target_x, target_y)) = roamer.target_cell {
//        let dx = (target_x as isize - roamer.roamer_x as isize).signum();
//        let dy = (target_y as isize - roamer.roamer_y as isize).signum();
//        let new_x = (roamer.roamer_x as isize + dx) as usize;
//        let new_y = (roamer.roamer_y as isize + dy) as usize;
//
//        if any_other_roamer_close(i_layer, new_x as f64, //new_y as f64, roamer.individual_id){
//
//            // find a neighboring cell with any other roamer false
//            let mut new_x = roamer.roamer_x;
//            let mut new_y = roamer.roamer_y;
//
//            let mut found = false;
//
//            for i in -1..=1 {
//                for j in -1..=1 {
//                    let x = (roamer.roamer_x as isize + i) as usize;
//                    let y = (roamer.roamer_y as isize + j) as usize;
//                    if x < grid.len() && y < grid[0].len() && !any_other_roamer_close//(i_layer, x as f64, y as f64, //roamer.individual_id) {
//                        new_x = x;
//                        new_y = y;
//                        found = true;
//                        roamer.roamer_x = new_x;
//                        roamer.roamer_y = new_y;
//                       
//                        break;
//                    }
//                }
//            }
//
//
//            if found == false { 
//                // random neighboring cell
//                let dx = rand::thread_rng().gen_range(-1..=1);
//                let dy = rand::thread_rng().gen_range(-1..=1);
//
//                let new_x = (roamer.roamer_x as isize + dx) as usize;
//                let new_y = (roamer.roamer_y as isize + dy) as usize;
//
//                // Update roamer's position if within grid boundaries
//                if new_x < grid.len() && new_y < grid[0].len()
//                //&& is_valid_cell(grid, new_x, new_y)
//                {
//                    roamer.roamer_x = new_x;
//                    roamer.roamer_y = new_y;
//                    roamer.daily_distance -= 1;
//                }
//            } 
//
//        } else {
//        // Update roamer's position if within grid boundaries
//        if new_x < grid.len() && new_y < grid[0].len() 
//        //&& is_valid_cell(grid, new_x, new_y) 
//        {
//            roamer.roamer_x = new_x;
//            roamer.roamer_y = new_y;
//            roamer.daily_distance -= 1;
//        }
//     }
//    }
//}

fn move_towards_target_cell_roamer(roamer: &mut RoamingIndividual, grid: &Vec<Vec<Cell>>, i_layer: &mut InteractionLayer) {
    if let Some((target_x, target_y)) = roamer.target_cell {
        let dx = (target_x as isize - roamer.roamer_x as isize).signum();
        let dy = (target_y as isize - roamer.roamer_y as isize).signum();
        let new_x = (roamer.roamer_x as isize + dx) as usize;
        let new_y = (roamer.roamer_y as isize + dy) as usize;

        if any_other_roamer_close(i_layer, new_x as f64, new_y as f64, roamer.individual_id) {
            let mut new_x = roamer.roamer_x;
            let mut new_y = roamer.roamer_y;
            let mut found = false;

            for i in -1..=1 {
                for j in -1..=1 {
                    let x = (roamer.roamer_x as isize + i) as usize;
                    let y = (roamer.roamer_y as isize + j) as usize;
                    if x < grid.len() && y < grid[0].len() && !any_other_roamer_close(i_layer, x as f64, y as f64, roamer.individual_id) {
                        new_x = x;
                        new_y = y;
                        found = true;
                        break;
                    }
                }
                if found {
                    roamer.roamer_x = new_x;
                    roamer.roamer_y = new_y;
                    if roamer.daily_distance > 0 {
                        roamer.daily_distance -= 1;
                    }
                    break;
                }
            }

            if !found {
                let dx = rand::thread_rng().gen_range(-1..=1);
                let dy = rand::thread_rng().gen_range(-1..=1);
                let new_x = (roamer.roamer_x as isize + dx) as usize;
                let new_y = (roamer.roamer_y as isize + dy) as usize;

                if new_x < grid.len() && new_y < grid[0].len() {
                    roamer.roamer_x = new_x;
                    roamer.roamer_y = new_y;
                    
                    // Ensure daily_distance does not underflow
                    if roamer.daily_distance > 0 {
                        roamer.daily_distance -= 1;
                    }
                }
            }
        } else {
            if new_x < grid.len() && new_y < grid[0].len() {
                roamer.roamer_x = new_x;
                roamer.roamer_y = new_y;
                
                // Ensure daily_distance does not underflow
                if roamer.daily_distance > 0 {
                    roamer.daily_distance -= 1;
                }
            }
        }
    }
}


fn move_randomly_roamer(roamer: &mut RoamingIndividual, grid: &Vec<Vec<Cell>>) {
    let dx = rand::thread_rng().gen_range(-1..=1);
    let dy = rand::thread_rng().gen_range(-1..=1);

    let new_x = (roamer.roamer_x as isize + dx) as usize;
    let new_y = (roamer.roamer_y as isize + dy) as usize;

    // Update roamer's position if within grid boundaries
    if new_x < grid.len() && new_y < grid[0].len() && is_valid_cell(grid, new_x, new_y) {
        roamer.roamer_x = new_x;
        roamer.roamer_y = new_y;
        roamer.daily_distance -= 1;
    }
}

pub fn move_roamer(roamer: &mut RoamingIndividual, grid: &mut Vec<Vec<Cell>>, i_layer: &mut InteractionLayer, time: usize, rng: &mut impl Rng, high_seats: &mut Vec<HighSeat>, hunting_statistics: &mut HuntingStatistics) -> bool {
    let mut to_be_removed = false;

    while roamer.daily_distance > 0 && !roamer.staying_with_target_group && !roamer.initial_dispersal && !roamer.reached_target {
        let move_towards_target = rand::thread_rng().gen_bool(0.25);
        
        if move_towards_target {
            move_towards_target_cell_roamer(roamer, grid, i_layer);
        } else {
            move_randomly_roamer(roamer, grid);
        }

        if hunting_check(grid, high_seats, rng, roamer.roamer_x, roamer.roamer_y) {
           
        hunting_statistics.add_hunted_individual(roamer.roamer_x, roamer.roamer_y, roamer.sex.clone(), roamer.age, roamer.age_class, roamer.individual_id, Some(roamer.origin_group_id) , IndividualType::GroupMember, time);

            to_be_removed = true;
            break;

           }

        i_layer.add_entity_and_record_movement(
            roamer.origin_group_id,
            "roamer",
            time,
            0, // Assuming time_left is not used
            0, // Assuming duration is not used
            roamer.individual_id,
            1.0, // Assuming interaction_strength is default
            roamer.roamer_x as f64, // Convert coordinates to f64 if necessary
            roamer.roamer_y as f64,  // Convert coordinates to f64 if necessary
            roamer.roamer_is_infected(),
            roamer.infection_stage.clone(),
        );

        if let Some((target_x, target_y)) = roamer.target_cell {
            if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                roamer.reached_target = true;
                roamer.staying_with_target_group = true;
                roamer.daily_distance = 0;
                break;
            }
        }
    }

    if to_be_removed {
       return true
    }

    if !roamer.reached_target {
        roamer.daily_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE;
    }

    return false
}


fn set_list_of_target_groups(roamer: &mut RoamingIndividual, groups: &Vec<Groups>) {
 // take the 5 closet groups to the current x/y and write their group_id into known_groups
    let mut known_groups = Vec::new();
    let mut groups_sorted = groups.iter().filter(|g| g.active == true && g.group_id != roamer.origin_group_id).cloned().collect::<Vec<Groups>>();
    groups_sorted.sort_by(|a, b| {
        let dist_a = (a.x as isize - roamer.roamer_x as isize).abs() + (a.y as isize - roamer.roamer_y as isize).abs();
        let dist_b = (b.x as isize - roamer.roamer_x as isize).abs() + (b.y as isize - roamer.roamer_y as isize).abs();
        dist_a.cmp(&dist_b)
    });
    for group in groups_sorted.iter().take(3) {
        known_groups.push(group.group_id);
    }
    roamer.known_groups = known_groups;
    // select a random group from the known_groups and write it into target_group
    //let target_group = known_groups.choose(rng);
    //roamer.target_group = Some(*target_group.unwrap());
}

fn get_3_groups_in_range(roamer: &mut RoamingIndividual, groups: &Vec<Groups>)  {
    let mut target_groups = Vec::new();
    //subset the grid to a 75 cell radius around the roamer
    let mut grid_subset = Vec::new();
    for i in 0..75 {
        for j in 0..75 {
            if let Some(x) = roamer.roamer_x.checked_add(i) {
                if let Some(y) = roamer.roamer_y.checked_add(j) {
                    grid_subset.push((x, y));
                }
            }
            if let Some(x) = roamer.roamer_x.checked_sub(i) {
                if let Some(y) = roamer.roamer_y.checked_sub(j) {
                    grid_subset.push((x, y));
                }
            }
            if let Some(x) = roamer.roamer_x.checked_add(i) {
                if let Some(y) = roamer.roamer_y.checked_sub(j) {
                    grid_subset.push((x, y));
                }
            }
            if let Some(x) = roamer.roamer_x.checked_sub(i) {
                if let Some(y) = roamer.roamer_y.checked_add(j) {
                    grid_subset.push((x, y));
                }
            }
        }
    }
    //check if any group is in the subset
    for group in groups.iter() {
        if grid_subset.iter().any(|(x, y)| group.x == *x && group.y == *y) {
            target_groups.push(group.group_id);
        }
    }
    // secet up to 3 target groups
    let mut target_groups_final = Vec::new();
    for group in target_groups.iter().take(3) {
        target_groups_final.push(*group);
    }
    
    roamer.known_groups = target_groups_final;

    if roamer.known_groups.is_empty() {
        //get all group ids
       let all_groups = get_all_group_ids(groups);
       // exclude parental group
       let known_groups = all_groups.iter()
           .filter(|&&g| g != roamer.origin_group_id)
           .cloned()
           .collect::<Vec<usize>>();
        roamer.known_groups = known_groups;
       }
       
 
}


fn select_target_group(roamer: &mut RoamingIndividual, rng: &mut impl Rng) -> Option<usize> {
    
    
    // Select a random group from the known groups
    let target_group = roamer.known_groups.choose(rng);
    if target_group.is_none() {
        return Some(0);
    }
    //log::info!("Roamer {:?} selected target group: {:?}", roamer.roamer_id, target_group.unwrap());
    roamer.target_group = Some(*target_group.unwrap());
    roamer.target_group

}

fn evaluate_and_set_target_cell(roamer: &mut RoamingIndividual, groups: &Vec<Groups>) {
    if let Some(target_group_id) = roamer.target_group {
        let target_group = groups.iter().find(|g| g.group_id == target_group_id).unwrap();
        let tx = target_group.x;
        let ty = target_group.y;
        let tcell = (tx, ty);
        roamer.target_cell = Some(tcell);
    }
}

fn roaming_check(roamer: &mut RoamingIndividual, groups: &Vec<Groups>, rng: &mut impl Rng) {
            if roamer.stay_time <= 0 && roamer.initial_dispersal == false {
            // check if all groups in known_groups exist and remove the ones from known groups that do not exist any more
            reevaluate_known_groups(roamer, groups);
                
            //log::info!("Roamer {:?} has reached target group {:?} and its stay time is up", roamer.roamer_id, roamer.target_group_id);
            let old_target_group = roamer.target_group.unwrap();
            let mut ptc = 0;
            while roamer.target_group.unwrap() == old_target_group && ptc < 10 {
                select_target_group(roamer, rng);
                roamer.target_group_id = roamer.target_group;
                ptc += 1;
                if ptc == 9 {
                     //set_list_of_target_groups(roamer, groups);
                     get_3_groups_in_range(roamer, groups);
                    select_target_group(roamer, rng);
                    roamer.target_group_id = roamer.target_group;
                }
                
            }
          
            evaluate_and_set_target_cell(roamer, groups);
            roamer.stay_time = 5 + rand::thread_rng().gen_range(0..10);
            roamer.reached_target = false;
            roamer.staying_with_target_group = false;
        }
}

fn reevaluate_known_groups(roamer: &mut RoamingIndividual, groups: &Vec<Groups>) {
    let mut known_groups = Vec::new();
    for group_id in roamer.known_groups.iter() {
        if groups.iter().any(|g| g.group_id == *group_id) {
            known_groups.push(*group_id);
        }
    }
    roamer.known_groups = known_groups;

    if roamer.known_groups.is_empty() {
        //get all group ids
       let all_groups = get_all_group_ids(groups);
       // exclude parental group
       let known_groups = all_groups.iter()
           .filter(|&&g| g != roamer.origin_group_id)
           .cloned()
           .collect::<Vec<usize>>();
        roamer.known_groups = known_groups;
       }
}

fn stay_with_target_group(roamer: &mut RoamingIndividual) {
    if roamer.stay_time > 0 {
    roamer.stay_time -= 1;
    } else {
    roamer.stay_time = 0;   
    }
}

fn move_roamer_with_target_group(roamer: &mut RoamingIndividual, grid: &mut Vec<Vec<Cell>>, i_layer: &mut InteractionLayer, time: usize, rng: &mut impl Rng, high_seats: &mut Vec<HighSeat>, hunting_statistics: &mut HuntingStatistics) -> bool {
    //let target_group: &Groups = groups.iter().find(|g| g.group_id == roamer.target_group.unwrap()).unwrap();
    //let tx = target_group.x;
    //let ty = target_group.y;
    //let tcell = (tx, ty);
    //roamer.target_cell = Some(tcell);

    if roamer.daily_distance < DEFAULT_DAILY_MOVEMENT_DISTANCE {
        roamer.daily_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE;
    }


    while roamer.daily_distance > 0 && roamer.staying_with_target_group == true && roamer.reached_target == false{
        //log::info!("Roamer {:?} is moving with group: {:?}", roamer.roamer_id, true);
        let move_towards_target = rand::thread_rng().gen_bool(0.25);
        let mut to_be_removed = false;    
        if move_towards_target {
            move_towards_target_cell_roamer(roamer, grid, i_layer);
          //  record_movement_in_interaction_layer_for_roamers(i_layer, roamer.roamer_x, roamer.roamer_y, time, roamer.origin_group_id,  "roamer", roamer.roamer_id);

          if hunting_check(grid, high_seats, rng, roamer.roamer_x, roamer.roamer_y) {
           
            hunting_statistics.add_hunted_individual(roamer.roamer_x, roamer.roamer_y, roamer.sex.clone(), roamer.age, roamer.age_class, roamer.individual_id, Some(roamer.origin_group_id) , IndividualType::GroupMember, time);
    
                to_be_removed = true;
               }

          i_layer.add_entity_and_record_movement(
            roamer.origin_group_id,
            "roamer",
            time,
            0, // Assuming time_left is not used
            0, // Assuming duration is not used
            roamer.individual_id,
            1.0, // Assuming interaction_strength is default
            roamer.roamer_x as f64, // Convert coordinates to f64 if necessary
            roamer.roamer_y as f64,  // Convert coordinates to f64 if necessary
            roamer.roamer_is_infected(),
            roamer.infection_stage.clone(),
        );

        if to_be_removed {
            return true
         }

            if let Some((target_x, target_y)) = roamer.target_cell {
                if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                    // Roamer reached target
                    roamer.reached_target = true;
                    break;
                }
            }

           
             
        } else {
            move_randomly_roamer(roamer, grid);

            if hunting_check(grid, high_seats, rng, roamer.roamer_x, roamer.roamer_y) {
           
                hunting_statistics.add_hunted_individual(roamer.roamer_x, roamer.roamer_y, roamer.sex.clone(), roamer.age, roamer.age_class, roamer.individual_id, Some(roamer.origin_group_id) , IndividualType::GroupMember, time);
        
                    to_be_removed = true;
                    
        
            }
          //  record_movement_in_interaction_layer_for_roamers(i_layer, roamer.roamer_x, roamer.roamer_y, time, roamer.origin_group_id,  "roamer", roamer.roamer_id);
          i_layer.add_entity_and_record_movement(
            roamer.origin_group_id,
            "roamer",
            time,
            0, // Assuming time_left is not used
            0, // Assuming duration is not used
            roamer.individual_id,
            1.0, // Assuming interaction_strength is default
            roamer.roamer_x as f64, // Convert coordinates to f64 if necessary
            roamer.roamer_y as f64,  // Convert coordinates to f64 if necessary
            roamer.roamer_is_infected(),
            roamer.infection_stage.clone(),
        );

        if to_be_removed {
            return true
         }

            if let Some((target_x, target_y)) = roamer.target_cell {
                if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                    // Roamer reached target
                    roamer.reached_target = true;
                    break;
                }
            }
        }     
    }
    //log::info!("Roamer {:?} finished moving with group for today", roamer.roamer_id);
    stay_with_target_group(roamer);
    roamer.daily_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE;
    return false
  
}

pub fn execute_roaming(roamers: &mut Vec<RoamingIndividual>, groups: &Vec<Groups>, grid: &mut Vec<Vec<Cell>>, rng: &mut impl Rng, i_layer: &mut InteractionLayer, time: usize, hs_vec: &mut Vec<HighSeat>, hunting_statistics: &mut HuntingStatistics) {
    let mut vec_ids_hunted_roamers: Vec<u32> = Vec::new(); 
    for roamer in roamers.iter_mut().filter(|roamer| roamer.initial_dispersal == false) {
        roaming_check(roamer, groups, rng);
        let mut just_moved_with_group = false;
        
        if roamer.staying_with_target_group == true {
            if move_roamer_with_target_group(roamer, grid, i_layer, time, rng, hs_vec, hunting_statistics){
                //add this roamers id to a list to be removed later
                vec_ids_hunted_roamers.push(roamer.roamer_id as u32);
            }
            if roamer.stay_time <= 0 {
                roamer.staying_with_target_group = false;
                roamer.reached_target = false;
                just_moved_with_group = true;
                //log::info!("Roamer {:?} has finished staying with target group", roamer.roamer_id);
                
            }
        }
        
        if just_moved_with_group == false {
           if move_roamer(roamer, grid, i_layer, time, rng, hs_vec, hunting_statistics){
            //add this roamers id to a list to be removed later
            vec_ids_hunted_roamers.push(roamer.roamer_id as u32);

            }
           }
        }

        // remove hunted roamers
        // take all the roamer id from the vec and remove them from the roamers vector
        for id in vec_ids_hunted_roamers.iter() {
            let index = roamers.iter().position(|r| r.roamer_id == *id as usize).unwrap();
            roamers.remove(index);
        }
        
    }     

