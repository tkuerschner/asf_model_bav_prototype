use crate::*;
use std::collections::HashMap;


use crate::grid_functions::*;


// Static counter for disperser_id
static mut ROAMER_COUNTER: usize = 0;

// Function to generate a unique individual_id
pub fn generate_roamer_id() -> usize {
    unsafe {
        ROAMER_COUNTER += 1;
        ROAMER_COUNTER
    }
}



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
    pub disperser_id: usize,
    pub target_cell:Option<(usize,usize)>,
    pub daily_distance: usize,
    pub target_group: Option<usize>,
    pub known_groups: Vec<usize>,
    pub initial_dispersal: bool,
    pub target_group_id: Option<usize>,
    pub reached_target: bool,
    pub stay_time: usize,
    pub staying_with_target_group: bool,
}

pub fn roamer_assignemnt(roamers: &mut Vec<RoamingIndividual>, groups: &mut Vec<Groups>) {
    
    // Create a map to store roaming individuals by their origin group ID
    //let mut roaming_by_group: HashMap<u64, Vec<RoamingIndividual>> = HashMap::new();

    for group in groups.iter_mut() {

        let members_to_roam_indices: Vec<usize> = group
            .group_members
            .iter()
            .enumerate()
            .filter(|(_, mem)| mem.age_class == AgeClass::Yearling && mem.sex == Sex::Male && !mem.has_dispersed)
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
                disperser_id: generate_disperser_id(),
                target_cell: None,
                daily_distance: DEFAULT_DAILY_MOVEMENT_DISTANCE,
                target_group: None,
                known_groups: Vec::new(),
                initial_dispersal: true,
                target_group_id: None,
                reached_target: false,
                stay_time: 5 + rand::thread_rng().gen_range(0..10), // Random stay time between 5 and 15 days
                staying_with_target_group: false,
            };
            // Add the dispersing individual to the dispersing_by_group map
           //let roamer_group = roaming_by_group.entry(group.group_id as u64).or_insert_with(Vec::new);
           //roamer_group.push(roamer.clone());
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

pub fn initial_roamer_dispersal_target(roamers: &mut Vec<RoamingIndividual>, groups: &Vec<Groups>, grid: &Vec<Vec<Cell>>, rng: &mut impl Rng) {
    for roamer in roamers.iter_mut() {
        if roamer.initial_dispersal == true {
       //select a target cell that is at least 100 cells away from any cell belonging to this roamers origin group and is_valid_cell using the grid_functions.rs
        let distance = 0;
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
           
        }
    }
 }
//}


pub fn initial_roamer_dispersal_movement(roamers: &mut Vec<RoamingIndividual>, grid: &Vec<Vec<Cell>>, groups: &Vec<Groups>) {
    
    for roamer in roamers.iter_mut() {
        while roamer.daily_distance > 0 && roamer.initial_dispersal == true{
            let move_towards_target = rand::thread_rng().gen_bool(0.25);

            //log::info!("Dispersing roamer {:?} is moving towards target: {:?}", roamer.roamer_id, move_towards_target);

            if move_towards_target {
                move_towards_target_cell_roamer(roamer, grid);
                if let Some((target_x, target_y)) = roamer.target_cell {
                    if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                        // Roamer reached target
                       // log::info!("Roamer {:?} reached target", roamer.roamer_id);                        
                        roamer.initial_dispersal = false;
                        set_list_of_target_group(roamer, groups);
                        break;
                    }
                }
            } else {
                move_randomly_roamer(roamer, grid);
                if let Some((target_x, target_y)) = roamer.target_cell {
                    if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                        // Roamer reached target
                       // log::info!("Roamer {:?} reached target", roamer.roamer_id);
                        roamer.initial_dispersal = false;
                        set_list_of_target_group(roamer, groups);
                        break;
                    }
                }
            }
        }
        //log::info!("Dispersing roamer {:?} finished moving towards target for today", roamer.roamer_id);
        roamer.daily_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE;
    }
}


fn move_towards_target_cell_roamer(roamer: &mut RoamingIndividual, grid: &Vec<Vec<Cell>>) {
    if let Some((target_x, target_y)) = roamer.target_cell {
        let dx = (target_x as isize - roamer.roamer_x as isize).signum();
        let dy = (target_y as isize - roamer.roamer_y as isize).signum();
        let new_x = (roamer.roamer_x as isize + dx) as usize;
        let new_y = (roamer.roamer_y as isize + dy) as usize;
        // Update roamer's position if within grid boundaries
        if new_x < grid.len() && new_y < grid[0].len() && is_valid_cell(grid, new_x, new_y) {
            roamer.roamer_x = new_x;
            roamer.roamer_y = new_y;
            roamer.daily_distance -= 1;
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

pub fn move_roamer(roamer: &mut RoamingIndividual, grid: &Vec<Vec<Cell>>) {
  
        while roamer.daily_distance > 0 && roamer.staying_with_target_group == false{
            let move_towards_target = rand::thread_rng().gen_bool(0.25);
            //log::info!("Roamer {:?} is moving towards target: {:?}", roamer.roamer_id, move_towards_target);
            if move_towards_target {
                move_towards_target_cell_roamer(roamer, grid);
                if let Some((target_x, target_y)) = roamer.target_cell {
                    if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                        // Roamer reached target
                        roamer.reached_target = true;
                        break;
                    }
                }
            } else {
                move_randomly_roamer(roamer, grid);
                if let Some((target_x, target_y)) = roamer.target_cell {
                    if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                        // Roamer reached target
                        roamer.reached_target = true;
                        break;
                    }
                }

            }
        }
        //log::info!("Roamer {:?} finished moving for today", roamer.roamer_id);
        roamer.daily_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE;

    
}
fn set_list_of_target_group(roamer: &mut RoamingIndividual, groups: &Vec<Groups>) {
 // take the 5 closet groups to the current x/y and write their group_id into known_groups
    let mut known_groups = Vec::new();
    let mut groups_sorted = groups.iter().filter(|g| g.active == true && g.group_id != roamer.origin_group_id).cloned().collect::<Vec<Groups>>();
    groups_sorted.sort_by(|a, b| {
        let dist_a = (a.x as isize - roamer.roamer_x as isize).abs() + (a.y as isize - roamer.roamer_y as isize).abs();
        let dist_b = (b.x as isize - roamer.roamer_x as isize).abs() + (b.y as isize - roamer.roamer_y as isize).abs();
        dist_a.cmp(&dist_b)
    });
    for group in groups_sorted.iter().take(5) {
        known_groups.push(group.group_id);
    }
    roamer.known_groups = known_groups;
    // select a random group from the known_groups and write it into target_group
    //let target_group = known_groups.choose(rng);
    //roamer.target_group = Some(*target_group.unwrap());
}


fn select_target_group(roamer: &mut RoamingIndividual, rng: &mut impl Rng) -> Option<usize> {
    // Select a random group from the known groups
    let target_group = roamer.known_groups.choose(rng);
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
            if roamer.reached_target == true && roamer.stay_time <= 0 {
            let old_target_group = roamer.target_group.unwrap();
            let mut ptc = 0;
            while roamer.target_group.unwrap() == old_target_group && ptc < 10 {
                select_target_group(roamer, rng);
                ptc += 1;
                if ptc == 9 {
                    set_list_of_target_group(roamer, groups);
                    select_target_group(roamer, rng);
                    
                }
                
            }
            evaluate_and_set_target_cell(roamer, groups);
            roamer.stay_time = 5 + rand::thread_rng().gen_range(0..10);
            roamer.reached_target = false;
            roamer.staying_with_target_group = false;
        }
}

fn stay_with_target_group(roamer: &mut RoamingIndividual) {
    roamer.stay_time -= 1;
}

fn move_roamer_with_target_group(roamer: &mut RoamingIndividual, groups: &Vec<Groups>, grid: &Vec<Vec<Cell>>){
    let target_group: &Groups = groups.iter().find(|g| g.group_id == roamer.target_group.unwrap()).unwrap();
    let tx = target_group.x;
    let ty = target_group.y;
    let tcell = (tx, ty);
    roamer.target_cell = Some(tcell);

    while roamer.daily_distance > 0 {
        //log::info!("Roamer {:?} is moving with group: {:?}", roamer.roamer_id, true);
        let move_towards_target = rand::thread_rng().gen_bool(0.25);

        if move_towards_target {
            move_towards_target_cell_roamer(roamer, grid);
            if let Some((target_x, target_y)) = roamer.target_cell {
                if roamer.roamer_x == target_x && roamer.roamer_y == target_y {
                    // Roamer reached target
                    roamer.reached_target = true;
                    break;
                }
            }
        } else {
            move_randomly_roamer(roamer, grid);
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
  
}

pub fn execute_roaming(roamers: &mut Vec<RoamingIndividual>, groups: &Vec<Groups>, grid: &Vec<Vec<Cell>>, rng: &mut impl Rng) {
    
    for roamer in roamers.iter_mut() {
        roaming_check(roamer, groups, rng);
        move_roamer(roamer, grid);
        
        if roamer.staying_with_target_group == true {
            move_roamer_with_target_group(roamer, groups, grid);
            if roamer.stay_time == 0 {
                roamer.staying_with_target_group = false;
            }
        }
    }     
}
