use crate::*;
use std::collections::HashMap;
//type InteractionLayer = HashMap<(usize, usize, usize), InteractionCell>;

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
    pub disp_indiv_x: usize,
    pub disp_indiv_y: usize,
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
    pub target_cell: Option<(usize,usize)>,
    pub daily_distance: usize,
    pub infection_stage: InfectionStage,
    pub time_of_infection: Option<usize>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DispersingFemaleGroup {
    pub dispersing_individuals: Vec<DispersingIndividual>,
    pub disp_grp_id: usize,
    pub target_cell: Option<(usize, usize)>,
    pub daily_distance: usize,
    pub disp_grp_x: usize,
    pub disp_grp_y: usize,
    pub marked_for_removal: bool,
}

impl DispersingFemaleGroup {
 
    pub fn infected_member_present(&self)->bool{
        let mut infected = false;
        for member in &self.dispersing_individuals {
            if member.health_status == HealthStatus::Infected && member.infection_stage == InfectionStage::Symptomatic {
                infected = true;
                break;
            }
        }
        infected
    }

pub fn get_id_random_group_member(&self) -> usize {
    if let Some(index) = self.dispersing_individuals.choose(&mut rand::thread_rng()) {
        index.individual_id
    } else {
        0
    }
}

pub fn remove_member_by_id(&mut self, id: usize) {
    if let Some(index) = self.dispersing_individuals.iter().position(|x| x.individual_id == id) {
        self.dispersing_individuals.remove(index);
    }
}

}

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
        if group.group_members.len() > group.max_size {
        // Iterate over indices in reverse order to remove elements safely
        for &index in members_to_disperse_indices.iter().rev() {
            //check against group size
            if group.group_members.len() > group.max_size {
            // Remove the member from the group and collect it as a dispersing individual
            let member = group.group_members.remove(index);
            let dispersing_individual = DispersingIndividual {
                disp_indiv_x: group.x,
                disp_indiv_y: group.y,
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
                infection_stage: InfectionStage::NotInfected,
                time_of_infection: None,
            };
            // Add the dispersing individual to the dispersing_by_group map
            let dispersing_group = dispersing_by_group.entry(group.group_id as u64).or_insert_with(Vec::new);
            dispersing_group.push(dispersing_individual.clone());
            }
            //log::info!("Number of dispersers: {}", members_to_disperse_indices.len());
        }
     }
    }
    log::info!("Number of dispersers: {}", dispersing_individuals.len());
    // Iterate over dispersing_by_group to create DispersingFemaleGroup instances as needed
    for (group_id, dispersing_individuals) in dispersing_by_group {
        // If there are at least two individuals, create a dispersing group
        if dispersing_individuals.len() >= 2 {
            let dispersing_group = DispersingFemaleGroup {
                dispersing_individuals: dispersing_individuals.clone(),
                disp_grp_id: generate_disperser_id(),
                target_cell: None,
                daily_distance: DEFAULT_DAILY_MOVEMENT_DISTANCE,
                disp_grp_x: dispersing_individuals[0].disp_indiv_x, // Use x of the first individual
                disp_grp_y: dispersing_individuals[0].disp_indiv_y, // Use y of the first individual
                marked_for_removal: false,
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
                time_of_infection: None,
                infection_stage: InfectionStage::NotInfected,
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

pub fn assign_dispersal_targets_groups(dispersing_groups: &mut Vec<DispersingFemaleGroup>, groups: &mut Vec<Groups>, grid: &Vec<Vec<Cell>>, rng: &mut impl Rng) {
   // let mut groups_to_remove = Vec::new(); // Vector to store indices of groups to remove
    let mut indices_to_remove = Vec::new();
    for (index, dispersing_group) in dispersing_groups.iter_mut().enumerate() {

        if dispersing_group.target_cell.is_none() {
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
               // println!("Unable to find a suitable target cell, merging dispersing group back");
                // function that tries to merge a dispersing group back into its oringal group if the original groups groups size is below the maximum group size, otherwise the individuals in the dispersing group die
                merge_dispersing_group_back_to_origin(dispersing_group, groups);
                indices_to_remove.push(index);
   
           }else{
           
            dispersing_group.target_cell = Some(target_cell);
           }
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
                        time_of_infection: None,
                        infection_stage: InfectionStage::NotInfected,
                };
                origin_group.group_members.push(group_member);

            }
    } else {
        // Handle the case where the origin group cannot be found
        println!("Error: Origin group not found!");
    }

}

pub fn move_female_disperser_group(model: &mut Model, rng: &mut impl Rng, time: usize) {
        let mut groups_to_remove = Vec::new(); // Vector to store indices of groups to remove
       // println!("Number of dispersing groups start: {}", dispersing_groups.len());
        for (index, disperser_group) in model.dispersers.iter_mut().enumerate() {
            let mut reached_target = false;

            if !disperser_group.dispersing_individuals.is_empty() {

            while disperser_group.daily_distance > 0 && !reached_target {
                // Randomly decide whether to move towards the target or move randomly
                let move_towards_target = rand::thread_rng().gen_bool(0.25);

                if move_towards_target {
                    move_towards_target_cell_group(disperser_group, &model.grid);

                    if hunting_check(&mut model.grid, &mut model.high_seats, rng, disperser_group.disp_grp_x, disperser_group.disp_grp_y) {
                     

                        let hunted_id = disperser_group.get_id_random_group_member();
                        let hi = disperser_group.dispersing_individuals.iter_mut().find(|individual| individual.individual_id == hunted_id).unwrap();

                        model.hunting_statistics.add_hunted_individual(disperser_group.disp_grp_x, disperser_group.disp_grp_y, hi.sex.clone(), hi.age, hi.age_class, hi.individual_id, Some(hi.origin_group_id), IndividualType::Disperser, model.global_variables.current_time);

                        disperser_group.remove_member_by_id(hunted_id);

                        if disperser_group.dispersing_individuals.is_empty() {
                            break;
                        }
                    }

                    let inf_here = disperser_group.infected_member_present();
                    let mut stage = InfectionStage::NotInfected;
                    if inf_here {
                       stage = InfectionStage::Symptomatic;
                    }

                    //record_movement_in_interaction_layer(i_layer, disperser_group.disp_grp_x, disperser_group.disp_grp_y, time, disperser_group.dispersing_individuals.last().unwrap().origin_group_id,  "disperser", disperser_group.dispersing_individuals.last().unwrap().disperser_id);
                    model.interaction_layer.add_entity_and_record_movement(
                        disperser_group.dispersing_individuals.last().unwrap().origin_group_id,
                        "disperser",
                        time,
                        0, // Assuming time_left is not used
                        0, // Assuming duration is not used
                        disperser_group.dispersing_individuals.last().unwrap().disperser_id,
                        1.0, // Assuming interaction_strength is default
                        disperser_group.disp_grp_x as f64, // Convert coordinates to f64 if necessary
                        disperser_group.disp_grp_y as f64,  // Convert coordinates to f64 if necessary
                        inf_here,
                        stage,

                    );

                    if disperser_group.disp_grp_x == disperser_group.target_cell.unwrap().0 && disperser_group.disp_grp_y == disperser_group.target_cell.unwrap().1 {
                        reached_target = true;
                       // println!("disperser reached target tw");
                        break;
                    }
                } else {
                    move_randomly_group(disperser_group, &model.grid);

                    if hunting_check(&mut model.grid, &mut model.high_seats, rng, disperser_group.disp_grp_x, disperser_group.disp_grp_y) {
                     

                        let hunted_id = disperser_group.get_id_random_group_member();
                        let hi = disperser_group.dispersing_individuals.iter_mut().find(|individual| individual.individual_id == hunted_id).unwrap();

                        model.hunting_statistics.add_hunted_individual(disperser_group.disp_grp_x, disperser_group.disp_grp_y, hi.sex.clone(), hi.age, hi.age_class, hi.individual_id, Some(hi.origin_group_id), IndividualType::Disperser, model.global_variables.current_time);

                        disperser_group.remove_member_by_id(hunted_id);

                        if disperser_group.dispersing_individuals.is_empty() {
                            break;
                        }
                    }

                    let inf_here = disperser_group.infected_member_present();
                    let mut stage = InfectionStage::NotInfected;
                    if inf_here {
                       stage = InfectionStage::Symptomatic;
                    }

                    //record_movement_in_interaction_layer(i_layer, disperser_group.disp_grp_x, disperser_group.disp_grp_y, time, disperser_group.dispersing_individuals.last().unwrap().origin_group_id,  "disperser", disperser_group.dispersing_individuals.last().unwrap().disperser_id);
                    model.interaction_layer.add_entity_and_record_movement(
                        disperser_group.dispersing_individuals.last().unwrap().origin_group_id,
                        "disperser",
                        time,
                        0, // Assuming time_left is not used
                        0, // Assuming duration is not used
                        disperser_group.dispersing_individuals.last().unwrap().disperser_id,
                        1.0, // Assuming interaction_strength is default
                        disperser_group.disp_grp_x as f64, // Convert coordinates to f64 if necessary
                        disperser_group.disp_grp_y as f64,  // Convert coordinates to f64 if necessary
                        inf_here,
                        stage,
                    );
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
                //println!("disperser reached target");
                handle_reached_target(disperser_group, &mut model.grid, &mut model.groups, rng, &mut groups_to_remove, index, model.global_variables.month );
            }

            }
        }
      //println!("Groups to remove: {:?}", groups_to_remove);
     // groups_to_remove.sort_unstable_by(|a, b| b.cmp(a));
     // for &index in groups_to_remove.iter().rev() {
     //   //println!("Removing group at index {}", index);
     //   if index < dispersing_groups.len() {
     //       dispersing_groups.remove(index);
     //   } else {
     //       println!("Index {} is out of bounds (length: {})", index , dispersing_groups.len());
     //   }
     //}
     
    // only keep the groups not marked for removal
    model.dispersers.retain(|dispersing_group| !dispersing_group.marked_for_removal);
    
    
}

fn handle_reached_target(
    disperser_group: &mut DispersingFemaleGroup,
    grid: &mut Vec<Vec<Cell>>,
    groups: &mut Vec<Groups>,
    rng: &mut impl Rng,
    _groups_to_remove: &mut Vec<usize>,
    _index: usize,
    month: u32,
) {
    if !is_valid_territory(grid, disperser_group.target_cell.unwrap().0, disperser_group.target_cell.unwrap().1) {
        // If the target is not a valid territory, redraw the dispersal target
        redraw_dispersal_target(disperser_group, grid, rng, groups);
        return;
    } else {
        let (target_x, target_y) = disperser_group.target_cell.unwrap();
        add_new_group_at_location(groups, grid, target_x, target_y);
        let new_group_id = groups.last().unwrap().group_id;
        //log::info!("New group will be added, new group id: {}", new_group_id);
        make_core_cell_an_ap(grid, groups.last().unwrap().core_cell.unwrap().0, groups.last().unwrap().core_cell.unwrap().1);
        
        if month > 6 && month < 10 {
            place_attraction_points_in_territory(grid, new_group_id, 6, rng);
        } else {
            place_attraction_points_in_territory(grid, new_group_id, 3, rng);
        }
        
        remove_ap_on_cells_with_quality_0(grid);
        
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
                time_of_infection: None,
                infection_stage: InfectionStage::NotInfected,
            };
            //filter for group with group_id == new_group_id and push new_group_member to group_members

           // groups[new_group_id].group_members.push(new_group_member);

           for group in groups.iter_mut() {
            if group.group_id == new_group_id {
                group.group_members.push(new_group_member);
                break;
                }
            }
           // log::info!("Individual {} moved to group {}", disperser.individual_id, new_group_id);
        }
        
        //groups_to_remove.push(index);
        disperser_group.marked_for_removal = true;
    }
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

pub fn redraw_dispersal_target(dispersing_group: &mut DispersingFemaleGroup, grid: &mut Vec<Vec<Cell>>, rng: &mut impl Rng, groups: &mut Vec<Groups>) {

    let mut target_cell = select_random_free_cell_in_range(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, rng, groups);
    let mut ptc = 0;
    //while !check_surrounding(grid, target_cell.0, target_cell.1, 100) && ptc < 10{ // check 100 cells around the target cell if they are taken
        while !is_valid_territory(grid, target_cell.0, target_cell.1) && ptc < 5{
        //println!("Target cell is isolated, looking for new target cell");
        //println!("Target cell: {:?}", target_cell);
        target_cell = select_random_free_cell_in_range(grid, dispersing_group.disp_grp_x, dispersing_group.disp_grp_y, rng, groups);
        ptc += 1;
    }
    if !is_valid_territory(grid, target_cell.0, target_cell.1)
    //if !check_surrounding(grid, target_cell.0, target_cell.1, 100)
    {
        //println!("Unable to find a suitable target cell, merging dispersing group");
        // function that tries to merge a dispersing group back into its oringal group if the original groups groups size is below the maximum group size, otherwise the individuals in the dispersing group die
        merge_dispersing_group_back_to_origin(dispersing_group, groups);
        
        dispersing_group.marked_for_removal = true;
        

        return;


    }else{
     dispersing_group.target_cell = Some(target_cell);
    }
}

//check for empty dispersal groups and remove them

pub fn check_and_remove_empty_dispersal_groups(dispersing_groups: &mut Vec<DispersingFemaleGroup>) {
    let mut indices_to_remove = Vec::new();
    for (index, dispersing_group) in dispersing_groups.iter().enumerate() {
        if dispersing_group.dispersing_individuals.is_empty() {
            indices_to_remove.push(index);
        }
    }

    for &index in indices_to_remove.iter().rev() {
        dispersing_groups.remove(index);
    }
}
