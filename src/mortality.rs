use crate::*;



//pub fn mortality(surv_prob: &SurvivalProbability, groups: &mut Vec<Groups>, random_mortality: &mut u32){
//
//    //mortality function that checks each groups group_members age their age class survival probability and removes them from the group if they die
//    for group in groups.iter_mut() {
//        let mut retained_group_members: Vec<GroupMember> = group.group_members
//            .drain(..)// remove all elements
//            .filter(|member| { // and add back the ones that survive
//                let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0); // random floating point number
//                let rounded_number = (random_number * 1e4).round() / 1e4; // rounded to 4 digits
//
//                if member.age_class != AgeClass::Piglet {   // if the age class is not piglet i.e. adult or yearling
//                    if rounded_number < surv_prob.adult {
//                        true
//                    } else {
//                        *random_mortality += 1; // increase the random mortality counter
//                        false // remove the individual
//                    }
//                } else {
//                    if rounded_number < surv_prob.piglet { // if the age class is piglet
//                        true
//                    } else {
//                        *random_mortality += 1; // increase the random mortality counter
//                        false // remove the individual
//                    }
//                }
//            })
//            .collect(); // collect the retained group members
//        group.group_members.extend_from_slice(&retained_group_members); // add the retained group members back to the group
//    }
//
//}

// increase mortality if number of group members exceed max group size
//pub fn max_group_size_mortality(surv_prob: &SurvivalProbability,groups: &mut Vec<Groups>, overcap_mortality: &mut u32) {
//    for group in groups.iter_mut() {
//        let current_group_size = group.group_members.len();
//        let max_group_size = group.max_size;
//        let diff = current_group_size as i32 - max_group_size as i32;
//        let mut surv_prob_adult = 0.0;
//        let mut surv_prob_piglet = 0.0;
//
//            if diff > 0 {
//             if diff > 10 && diff <= 25{
//
//                surv_prob_adult =  surv_prob.adult + (surv_prob.adult * 0.1);
//                surv_prob_piglet =  surv_prob.piglet + (surv_prob.piglet * 0.2);
//
//             }else if diff > 25{
//
//                surv_prob_adult =  surv_prob.adult + (surv_prob.adult * 0.2);
//                surv_prob_piglet =  surv_prob.piglet + (surv_prob.piglet * 0.4);
//
//            }
//        }
//
//        let mut retained_group_members: Vec<GroupMember> = group.group_members
//            .drain(..)
//            .filter(|member| {
//                let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0);
//                let rounded_number = (random_number * 1e4).round() / 1e4;
//
//                if member.age_class != AgeClass::Piglet {
//                    if rounded_number < surv_prob_adult {
//                        true
//                    } else {
//                        *overcap_mortality += 1;
//                        false
//                    }
//                } else {
//                    if rounded_number < surv_prob_piglet {
//                        true
//                    } else {
//                        *overcap_mortality += 1;
//                        false
//                    }
//                }
//            })
//            .collect();
//        group.group_members.extend_from_slice(&retained_group_members);
//    }
//}


pub fn combined_mortality(surv_prob: &SurvivalProbability,groups: &mut Vec<Groups>, overcap_mortality: &mut u32,  random_mortality: &mut u32){
    for group in groups.iter_mut() {
        let current_group_size = group.group_members.len();
        let max_group_size = group.max_size;
        let diff = current_group_size as i32 - max_group_size as i32;
        let surv_prob_adult;
        let surv_prob_piglet;
        let mut overcap_check = false;
        

        if diff > 0 { // if the group size is greater than the max group size
            let decrease = diff as f64 * 0.001; // decrease the survival probability by 0.1% for each individual over the max group size
            surv_prob_adult =   surv_prob.adult - (surv_prob.adult * decrease); 
            surv_prob_piglet =  surv_prob.piglet - (surv_prob.piglet * decrease);
            overcap_check = true;
            
        } else { // if the group size is less than the max group size
            surv_prob_adult = surv_prob.adult;
            surv_prob_piglet = surv_prob.piglet;
        }

        let retained_group_members: Vec<GroupMember> = group.group_members
        .drain(..)
        .filter(|member| {
            let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let rounded_number = (random_number * 1e4).round() / 1e4;

            if member.age_class != AgeClass::Piglet {
                if rounded_number < surv_prob_adult {
                    true
                } else {
                    if overcap_check {
                        *overcap_mortality += 1;
                    } else {
                        *random_mortality += 1;
                    }   
                    false
                }
            } else {
                if rounded_number < surv_prob_piglet {
                    true
                } else {
                    if overcap_check {
                        *overcap_mortality += 1;
                    } else {
                        *random_mortality += 1;
                    }
                    false
                }
            }
        })
        .collect();
    group.group_members.extend_from_slice(&retained_group_members);
    }
}

pub fn disperser_mortality(surv_prob: &SurvivalProbability, dispersing_groups: &mut Vec<DispersingFemaleGroup>, random_mortality: &mut u32){
    for d_group in dispersing_groups.iter_mut() {
        let retained_d_group_members: Vec<DispersingIndividual> = d_group.dispersing_individuals
        .drain(..)
        .filter(|member| {
            let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let rounded_number = (random_number * 1e4).round() / 1e4;

            if member.age_class != AgeClass::Piglet {
                if rounded_number < surv_prob.adult {
                    true
                } else {
                    *random_mortality += 1;
                    false
                }
            } else {
                if rounded_number < surv_prob.piglet {
                    true
                } else {
                    *random_mortality += 1;
                    false
                }
            }
        })
        .collect();
    d_group.dispersing_individuals.extend_from_slice(&retained_d_group_members);
    }
}

pub fn roamer_mortality(surv_prob: &SurvivalProbability, roaming_individuals: &mut Vec<RoamingIndividual>, random_mortality: &mut u32){
 
        let retained_r_group_members: Vec<RoamingIndividual> = roaming_individuals
        .drain(..)
        .filter(|member| {
            let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let rounded_number = (random_number * 1e4).round() / 1e4;

            if member.age_class != AgeClass::Piglet {
                if rounded_number < surv_prob.adult {
                    true
                } else {
                    *random_mortality += 1;
                    false
                }
            } else {
                if rounded_number < surv_prob.piglet {
                    true
                } else {
                    *random_mortality += 1;
                    false
                }
            }
        })
        .collect();
    roaming_individuals.extend_from_slice(&retained_r_group_members);
    
    
}

pub fn execute_mortality(surv_prob: &SurvivalProbability, groups: &mut Vec<Groups>, dispersing_groups: &mut Vec<DispersingFemaleGroup>, roaming_individuals: &mut Vec<RoamingIndividual>, random_mortality: &mut u32, overcap_mortality: &mut u32){
    combined_mortality(surv_prob, groups, overcap_mortality, random_mortality);
    disperser_mortality(surv_prob, dispersing_groups, random_mortality);
    roamer_mortality(surv_prob, roaming_individuals, random_mortality);
}