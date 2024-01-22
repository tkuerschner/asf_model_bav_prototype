

use crate::*;


// FIX ME: complete rewrite for groups


//pub fn reproduction(month: u32, individuals: &mut Vec<Groups>, current_tick: usize) {
//    // Define the probability of reproduction based on the month
//    let reproduction_probability = match month {
//        1 => 0.06,  // From SwifCoIbm 
//        2 => 0.16,  // From SwifCoIbm 
//        3 => 0.39,  // From SwifCoIbm 
//        4 => 0.73,  // From SwifCoIbm 
//        5 => 0.80,  // From SwifCoIbm 
//        6 => 0.88,  // From SwifCoIbm 
//        7 => 0.94,  // From SwifCoIbm 
//        8 => 0.97,  // From SwifCoIbm 
//        9 => 1.00,  // From SwifCoIbm 
//       
//        _ => 0.0,  // Default to 0% probability
//    };
//
//// Create a separate vector for new individuals
//let mut new_individuals: Vec<Groups> = Vec::new();
//
//// Calculate the new ID outside the loop
//let new_id = individuals.len();
//
//// Check if individual can reproduce again 1 year after previous reproduction
//
//for individual in individuals.iter_mut().filter(|ind| {ind.has_reproduced}){
//
//    if individual.time_of_reproduction + (28 * 12) == current_tick {
//        individual.time_of_reproduction = 0;
//        individual.has_reproduced = false;
//    }
//
//}
//
//// Iterate over eligible individuals for reproduction
//for individual in individuals.iter_mut().filter(|ind| {
//    ind.sex == Sex::Female && !ind.has_reproduced && ind.age_class == AgeClass::Adult && rand::thread_rng().gen_bool(reproduction_probability)
//}) {
//    // Mark the individual as having reproduced and record time
//    individual.has_reproduced = true;
//    individual.time_of_reproduction = current_tick;
//
//    //debug print REMOVE ME
//    //println!("created {} new individuals", num_new_individuals);
//
//    // Generate new individuals
//    let num_new_individuals = rand::thread_rng().gen_range(1..5);
//
//    for _ in 0..num_new_individuals {
//
//        let new_sex;
//        if rand::thread_rng().gen_bool(0.5) == true {
//             new_sex = Sex::Female;
//        }else{
//             new_sex = Sex::Male;
//        }
//        
//        // = Sex {
//        //    male: rand::thread_rng().gen_bool(0.5),
//        //    female: rand::thread_rng().gen_bool(0.5),
//        //};
//
//        let new_individual = Groups {
//            id: new_id + new_individuals.len(),  // Use the pre-calculated new ID
//            group_id: individual.group_id,  // Inherit group ID
//            x: individual.x,  // Inherit current location
//            y: individual.y,
//            age: 0,  // New individual starts at age 0
//            sex: new_sex,
//            has_reproduced: false,  // New individual hasn't reproduced yet
//            time_of_reproduction: 0,
//            age_class: AgeClass::Piglet,
//            memory: GroupMemory {
//                known_cells: HashSet::new(),
//                known_cells_order: Vec::new(),
//                //last_visited_cells: HashSet::new(),
//                //last_visited_cells_order: Vec::new(),
//                group_member_ids: Vec::new(),
//                presence_timer: 0,
//            },
//            core_cell: None,
//            target_cell: None,
//            remaining_stay_time: 0,
//        };
//
//        // Add the new individual to the separate vector
//        new_individuals.push(new_individual);
//    }
//}
//
//// Append the new individuals to the original vector
//individuals.extend(new_individuals);
//
//}
//

//pub fn reproduction(month: u32, groups: &mut Vec<Groups>, current_tick: usize) {
//    // Define the probability of reproduction based on the month
//    let reproduction_probability = match month {
//        1 => 0.06,
//        2 => 0.16,
//        3 => 0.39,
//        4 => 0.73,
//        5 => 0.80,
//        6 => 0.88,
//        7 => 0.94,
//        8 => 0.97,
//        9 => 1.00,
//        _ => 0.0,
//    };
//
//    // Iterate over groups
//    for group in groups.iter_mut() {
//         // Clone the group members to avoid multiple mutable borrows
//         let members_to_reproduce: Vec<GroupMember> = group.group_members.clone();
//
//         // Check if group members can reproduce again 1 year after previous reproduction
//         for member in members_to_reproduce.iter().filter(|mem| mem.has_reproduced) {
//             if member.time_of_reproduction + (28 * 12) == current_tick {
//                 // Find the corresponding mutable reference and update the original group
//                 if let Some(original_member) = group.group_members.iter_mut().find(|original| original.individual_id == member.individual_id) {
//                     original_member.time_of_reproduction = 0;
//                     original_member.has_reproduced = false;
//                 }
//             }
//         }
//        // Iterate over eligible group members for reproduction
//        for member in group.group_members.iter_mut().filter(|mem| {
//            mem.sex == Sex::Female
//                && !mem.has_reproduced
//                && mem.age_class == AgeClass::Adult
//                && rand::thread_rng().gen_bool(reproduction_probability)
//        }) {
//            // Mark the group member as having reproduced and record time
//            member.has_reproduced = true;
//            member.time_of_reproduction = current_tick;
//
//            // Generate new group members
//            let num_new_members = rand::thread_rng().gen_range(1..5);
//
//            for _ in 0..num_new_members {
//                let new_sex = if rand::thread_rng().gen_bool(0.5) {
//                    Sex::Female
//                } else {
//                    Sex::Male
//                };
//
//                let new_member = GroupMember {
//                    individual_id: generate_individual_id(),
//                    age: 0,  // Set the initial age for new members
//                    age_class: AgeClass::Piglet,  // Set the initial age class for new members
//                    sex: new_sex,
//                    health_status: HealthStatus::Susceptible,
//                    time_of_birth: current_tick,  // Record the birth time for new members
//                    has_reproduced: false,
//                    time_of_reproduction: 0,
//                };
//
//                // Add the new group member to the group
//                group.group_members.push(new_member);
//            }
//        }
//    }
//}

pub fn reproduction(month: u32, groups: &mut Vec<Groups>, current_tick: usize) {
    let reproduction_probability = match month {
        1 => 0.06,
        2 => 0.16,
        3 => 0.39,
        4 => 0.73,
        5 => 0.80,
        6 => 0.88,
        7 => 0.94,
        8 => 0.97,
        9 => 1.00,
        _ => 0.0,
    };

    for group in groups.iter_mut() {
        // Collect indices of members to be reproduced
        let members_to_reproduce_indices: Vec<usize> = group
            .group_members
            .iter()
            .enumerate()
            .filter(|(_, mem)| mem.has_reproduced)
            .map(|(i, _)| i)
            .collect();

        // Update members that need reproduction
        for index in members_to_reproduce_indices {
            let member = &mut group.group_members[index];
            if member.time_of_reproduction + (28 * 12) == current_tick {
                member.time_of_reproduction = 0;
                member.has_reproduced = false;
            }
        }

        // Collect indices of eligible members for reproduction
        let eligible_members_indices: Vec<usize> = group
            .group_members
            .iter()
            .enumerate()
            .filter(|(_, mem)| {
                mem.sex == Sex::Female
                    && !mem.has_reproduced
                    && mem.age_class == AgeClass::Adult
                    && rand::thread_rng().gen_bool(reproduction_probability)
            })
            .map(|(i, _)| i)
            .collect();

        // Add new members
        for index in eligible_members_indices {
            let num_new_members = rand::thread_rng().gen_range(1..5);

            for _ in 0..num_new_members {
                let new_sex = if rand::thread_rng().gen_bool(0.5) {
                    Sex::Female
                } else {
                    Sex::Male
                };

                let new_member = GroupMember {
                    individual_id: generate_individual_id(),
                    age: 0,
                    age_class: AgeClass::Piglet,
                    sex: new_sex,
                    health_status: HealthStatus::Susceptible,
                    time_of_birth: current_tick,
                    has_reproduced: false,
                    time_of_reproduction: 0,
                };

                group.group_members.push(new_member);
            }

            // Mark the original members as having reproduced and record time
            group.group_members[index].has_reproduced = true;
            group.group_members[index].time_of_reproduction = current_tick;
        }
    }
}