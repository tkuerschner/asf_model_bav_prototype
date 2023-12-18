

use crate::*;

pub fn reproduction(month: u32, individuals: &mut Vec<Individual>, current_tick: usize) {
    // Define the probability of reproduction based on the month
    let reproduction_probability = match month {
        1 => 0.06,  // From SwifCoIbm 
        2 => 0.16,  // From SwifCoIbm 
        3 => 0.39,  // From SwifCoIbm 
        4 => 0.73,  // From SwifCoIbm 
        5 => 0.80,  // From SwifCoIbm 
        6 => 0.88,  // From SwifCoIbm 
        7 => 0.94,  // From SwifCoIbm 
        8 => 0.97,  // From SwifCoIbm 
        9 => 1.00,  // From SwifCoIbm 
       
        _ => 0.0,  // Default to 0% probability
    };

// Create a separate vector for new individuals
let mut new_individuals: Vec<Individual> = Vec::new();

// Calculate the new ID outside the loop
let new_id = individuals.len();

// Check if individual can reproduce again 1 year after previous reproduction

for individual in individuals.iter_mut().filter(|ind| {ind.has_reproduced}){

    if individual.time_of_reproduction + (28 * 12) == current_tick {
        individual.time_of_reproduction = 0;
        individual.has_reproduced = false;
    }

}

// Iterate over eligible individuals for reproduction
for individual in individuals.iter_mut().filter(|ind| {
    ind.sex == Sex::Female && !ind.has_reproduced && ind.age_class == AgeClass::Adult && rand::thread_rng().gen_bool(reproduction_probability)
}) {
    // Mark the individual as having reproduced and record time
    individual.has_reproduced = true;
    individual.time_of_reproduction = current_tick;

    //debug print REMOVE ME
    //println!("created {} new individuals", num_new_individuals);

    // Generate new individuals
    let num_new_individuals = rand::thread_rng().gen_range(1..5);

    for _ in 0..num_new_individuals {

        let new_sex;
        if rand::thread_rng().gen_bool(0.5) == true {
             new_sex = Sex::Female;
        }else{
             new_sex = Sex::Male;
        }
        
        // = Sex {
        //    male: rand::thread_rng().gen_bool(0.5),
        //    female: rand::thread_rng().gen_bool(0.5),
        //};

        let new_individual = Individual {
            id: new_id + new_individuals.len(),  // Use the pre-calculated new ID
            group_id: individual.group_id,  // Inherit group ID
            x: individual.x,  // Inherit current location
            y: individual.y,
            age: 0,  // New individual starts at age 0
            sex: new_sex,
            has_reproduced: false,  // New individual hasn't reproduced yet
            time_of_reproduction: 0,
            age_class: AgeClass::Piglet,
            memory: IndividualMemory {
                known_cells: HashSet::new(),
                known_cells_order: Vec::new(),
                //last_visited_cells: HashSet::new(),
                //last_visited_cells_order: Vec::new(),
                group_member_ids: Vec::new(),
                presence_timer: 0,
            },
            core_cell: None,
            target_cell: None,
            remaining_stay_time: 0,
        };

        // Add the new individual to the separate vector
        new_individuals.push(new_individual);
    }
}

// Append the new individuals to the original vector
individuals.extend(new_individuals);

}
