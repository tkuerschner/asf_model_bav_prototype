use crate::AgeClass;
use crate::Groups;
use crate::MAX_AGE;

//FIX ME to change the age of the group members


//pub fn ageing(individuals: &mut Vec<Groups>, age_mortality: &mut u32) {
//    for individual in individuals.iter_mut() {
//        individual.age += 1;
//
//        if individual.age < 12 * 28 { // FIX ME
//            individual.age_class = AgeClass::Piglet
//        }else if  individual.age < (12 * 28 * 2) { // FIX ME
//            individual.age_class = AgeClass::Yearling
//        }else {
//            individual.age_class = AgeClass::Adult
//        }
//    }
//
//    // Filter out individuals whose age exceeds the maximum age
//    let retained_individuals: Vec<Groups> = individuals
//        .drain(..)
//        .filter(|individual| {
//            if individual.age > MAX_AGE {
//                // Increment age_mortality counter when an individual is removed
//                *age_mortality += 1;
//                false
//            } else {
//                true
//            }
//        })
//        .collect();
//
//    // Clear the original vector and insert retained individuals
//    individuals.clear();
//    individuals.extend_from_slice(&retained_individuals);
//}