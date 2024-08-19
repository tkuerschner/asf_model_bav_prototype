use crate::*;


pub fn combined_mortality(surv_prob: &SurvivalProbability, model: &mut Model) {
    let mut dead_group_members = Vec::new();

    for group in model.groups.iter_mut() {
        let current_group_size = group.group_members.len();
        let max_group_size = group.max_size;
        let diff = current_group_size as i32 - max_group_size as i32;
        let mut surv_prob_adult = surv_prob.adult;
        let mut surv_prob_piglet = surv_prob.piglet;
        let mut overcap_check = false;

        if diff > 0 {
            let decrease = diff as f64 * 0.001;
            surv_prob_adult -= surv_prob_adult * decrease;
            surv_prob_piglet -= surv_prob_piglet * decrease;
            overcap_check = true;
        }

        group.group_members.retain(|member| {
            let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let rounded_number = (random_number * 1e4).round() / 1e4;

            let survives = if member.age_class != AgeClass::Piglet {
                rounded_number < surv_prob_adult
            } else {
                rounded_number < surv_prob_piglet
            };

            if survives {
                true
            } else {
                if overcap_check {
                    model.global_variables.overcapacity_mortality += 1;
                } else {
                    model.global_variables.random_mortality += 1;
                }
                dead_group_members.push(member.clone()); // Clone the member
                false
            }
        });
    }

    // Create carcasses for all dead group members outside the loop
    for member in dead_group_members {
        create_carcass(&member, model);
    }
}


pub fn disperser_mortality(
    surv_prob: &SurvivalProbability,
    model: &mut Model
) {
    let mut dead_dispersing_individuals = Vec::new();

    for d_group in model.dispersers.iter_mut() {
        let retained_d_group_members: Vec<DispersingIndividual> = d_group.dispersing_individuals
            .drain(..)
            .filter(|member| {
                let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0);
                let rounded_number = (random_number * 1e4).round() / 1e4;

                let survives = if member.age_class != AgeClass::Piglet {
                    rounded_number < surv_prob.adult
                } else {
                    rounded_number < surv_prob.piglet
                };

                if survives {
                    true
                } else {
                    model.global_variables.random_mortality += 1;
                    dead_dispersing_individuals.push(member.clone()); // Clone the member
                    false
                }
            })
            .collect();

        d_group.dispersing_individuals.extend_from_slice(&retained_d_group_members);
    }

    // Create carcasses for all dead dispersing individuals outside the loop
    for member in dead_dispersing_individuals {
        create_carcass(&member, model);
    }
}



pub fn roamer_mortality(surv_prob: &SurvivalProbability, model: &mut Model) {
    let mut dead_roamers = Vec::new();

    model.roamers.retain(|member| {
        let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0);
        let rounded_number = (random_number * 1e4).round() / 1e4;

        let survives = if member.age_class != AgeClass::Piglet {
            rounded_number < surv_prob.adult
        } else {
            rounded_number < surv_prob.piglet
        };

        if survives {
            true
        } else {
            model.global_variables.random_mortality += 1;
            dead_roamers.push(member.clone());
            false
        }
    });

    for member in dead_roamers {
        create_carcass(&member, model);
    }
}

pub fn execute_mortality(model: &mut Model, surv_prob: &SurvivalProbability){


    combined_mortality(surv_prob, model);
    piglet_specific_mortality(&mut model.groups);
    disperser_mortality(surv_prob, model);
    roamer_mortality(surv_prob, model);
}

pub fn piglet_specific_mortality(groups: &mut Vec<Groups> ) {

for  group in groups {

    // if there is no adult left in the group, teh piglets will die
    if group.group_members.iter().all(|member| member.age_class != AgeClass::Adult) {
        group.group_members.retain(|member| member.age_class != AgeClass::Piglet);
    }
}



}

pub fn roamer_density_dependent_removal(model: &mut Model) {
    
    let n_adult_female = model.groups.iter().map(|group| group.group_members.iter().filter(|member| member.age_class == AgeClass::Adult && member.sex == Sex::Female).count()).sum::<usize>();

    let n_roaming_adults = model.roamers.iter().filter(|roamer| roamer.age_class == AgeClass::Adult).count();
    // as long as there are more roamers then adult females, remove random roamer

    if n_roaming_adults > n_adult_female {
        let n_to_remove = n_roaming_adults - n_adult_female;
        log::info!("Removing {} adult male roamers due to density dependent mortality since there are {} adult females", n_to_remove, n_adult_female);
        for _ in 0..n_to_remove {
            let index = rand::thread_rng().gen_range(0..model.roamers.len());
            model.roamers.remove(index);
        }

    }

 
}

//pub fn roamer_mortality(surv_prob: &SurvivalProbability, model: &mut Model){
// 
//    let retained_roamers: Vec<RoamingIndividual> = model.roamers
//    .drain(..)
//    .filter(|member| {
//        let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0);
//        let rounded_number = (random_number * 1e4).round() / 1e4;
//
//        if member.age_class != AgeClass::Piglet {
//        if rounded_number < surv_prob.adult {
//            true
//        } else {
//            model.global_variables.random_mortality += 1;
//            create_carcass(*member, model); 
//            false
//        }
//        } else {
//        if rounded_number < surv_prob.piglet {
//            true
//        } else {
//            model.global_variables.random_mortality += 1;
//            create_carcass(*member, model); 
//            false
//        }
//        }
//    })
//    .collect();
//    model.roamers.extend_from_slice(&retained_roamers);
//}


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

//pub fn disperser_mortality(surv_prob: &SurvivalProbability, dispersing_groups: &mut Vec<DispersingFemaleGroup>, //random_mortality: &mut u32){
//    for d_group in dispersing_groups.iter_mut() {
//        let retained_d_group_members: Vec<DispersingIndividual> = d_group.dispersing_individuals
//        .drain(..)
//        .filter(|member| {
//            let random_number: f64 = rand::thread_rng().gen_range(0.0..1.0);
//            let rounded_number = (random_number * 1e4).round() / 1e4;
//
//            if member.age_class != AgeClass::Piglet {
//                if rounded_number < surv_prob.adult {
//                    true
//                } else {
//                    *random_mortality += 1;
//                    false
//                }
//            } else {
//                if rounded_number < surv_prob.piglet {
//                    true
//                } else {
//                    *random_mortality += 1;
//                    false
//                }
//            }
//        })
//        .collect();
//    d_group.dispersing_individuals.extend_from_slice(&retained_d_group_members);
//    }
//}
