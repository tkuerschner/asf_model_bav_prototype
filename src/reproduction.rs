
use crate::*;

pub fn reproduction(month: u32, individuals: &mut Vec<Individual>, num_new_individuals: usize) {
    // Define the probability of reproduction based on the month
    let reproduction_probability = match month {
        1 => 1.00,  // Adjust 
        2 => 1.00,  // Adjust 
        // add from other model
        _ => 1.0,  // Default to 1 probability
    };

// Create a separate vector for new individuals
let mut new_individuals: Vec<Individual> = Vec::new();

// Calculate the new ID outside the loop
let new_id = individuals.len();

// Iterate over eligible individuals for reproduction
for individual in individuals.iter_mut().filter(|ind| {
    ind.sex == Sex::female && !ind.has_reproduced && ind.age_class == AgeClass::Adult && rand::thread_rng().gen_bool(reproduction_probability)
}) {
    // Mark the individual as having reproduced
    individual.has_reproduced = true;

    //debug print REMOVE ME
    //println!("created {} new individuals", num_new_individuals);

    // Generate new individuals
    for _ in 0..num_new_individuals {

        let new_sex;
        if rand::thread_rng().gen_bool(0.5) == true {
             new_sex = Sex::female;
        }else{
             new_sex = Sex::male;
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
            age_class: AgeClass::Piglet,
            memory: IndividualMemory {
                known_cells: HashSet::new(),
                known_cells_order: Vec::new(),
                last_visited_cells: HashSet::new(),
                last_visited_cells_order: Vec::new(),
                group_member_ids: Vec::new(),
                presence_timer: 0,
            },
        };

        // Add the new individual to the separate vector
        new_individuals.push(new_individual);
    }
}

// Append the new individuals to the original vector
individuals.extend(new_individuals);

}
