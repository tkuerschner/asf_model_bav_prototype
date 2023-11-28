use crate::Individual;
use crate::MAX_AGE;

pub fn ageing(individuals: &mut Vec<Individual>, age_mortality: &mut u32) {
    for individual in individuals.iter_mut() {
        individual.age += 1;
    }

    // Filter out individuals whose age exceeds the maximum age
    let retained_individuals: Vec<Individual> = individuals
        .drain(..)
        .filter(|individual| {
            if individual.age > MAX_AGE {
                // Increment age_mortality counter when an individual is removed
                *age_mortality += 1;
                false
            } else {
                true
            }
        })
        .collect();

    // Clear the original vector and insert retained individuals
    individuals.clear();
    individuals.extend_from_slice(&retained_individuals);
}