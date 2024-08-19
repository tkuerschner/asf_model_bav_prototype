use crate::AgeClass;
//use crate::Groups;
use crate::Model;
use crate::MAX_AGE;


//pub fn ageing(groups: &mut Vec<Groups>, age_mortality: &mut u32) {
//    for group in groups.iter_mut() {
//        for member in &mut group.group_members {
//            member.age += 1;
//            if member.age < 12 * 28 {
//                member.age_class = AgeClass::Piglet;
//            } else if member.age < 12 * 28 * 2 {
//                member.age_class = AgeClass::Yearling;
//            } else {
//                member.age_class = AgeClass::Adult;
//            }
//
//            if member.age >= MAX_AGE {
//                *age_mortality += 1;
//                continue;
//            }
//        }
//        // Remove members whose age exceeds the maximum age
//        group.group_members.retain(|member| member.age < MAX_AGE);
//    }
//}



pub fn ageing_old(model: &mut Model) {
    
    for group in model.groups.iter_mut() {
        for member in &mut group.group_members {
            member.age += 1;
            if member.age < 12 * 28 {
                member.age_class = AgeClass::Piglet;
            } else if member.age < 12 * 28 * 2 {
                member.age_class = AgeClass::Yearling;
            } else {
                member.age_class = AgeClass::Adult;
            }

            if member.age >= MAX_AGE {
                model.global_variables.age_mortality += 1;
                continue;
            }
        }
        // Remove members whose age exceeds the maximum age
        group.group_members.retain(|member| member.age < MAX_AGE);
    }

    for roamer in &mut model.roamers.iter_mut() {
        roamer.age += 1;
        if roamer.age < 12 * 28 {
            roamer.age_class = AgeClass::Piglet;
        } else if roamer.age < 12 * 28 * 2 {
            roamer.age_class = AgeClass::Yearling;
        } else {
            roamer.age_class = AgeClass::Adult;
        }

        if roamer.age >= MAX_AGE {
            model.global_variables.age_mortality += 1;
            continue;
        }
        
    }
    // Remove roamers whose age exceeds the maximum age
    model.roamers.retain(|roamer| roamer.age < MAX_AGE);
   
    for disperser_group in model.dispersers.iter_mut() {
        for disperser in &mut disperser_group.dispersing_individuals {
            disperser.age += 1;
            if disperser.age < 12 * 28 {
                disperser.age_class = AgeClass::Piglet;
            } else if disperser.age < 12 * 28 * 2 {
                disperser.age_class = AgeClass::Yearling;
            } else {
                disperser.age_class = AgeClass::Adult;
            }

            if disperser.age >= MAX_AGE {
                model.global_variables.age_mortality += 1;
                continue;
            }
        }
    }

    // Remove dispersers whose age exceeds the maximum age
    model.dispersers.retain(|disperser_group| disperser_group.dispersing_individuals.iter().all(|disperser| disperser.age < MAX_AGE));
    
}

use crossbeam::scope;
use std::sync::{Arc, Mutex};

pub fn ageing(model: &mut Model) {
    const PIGLET_AGE: u32 = 12 * 28;
    const YEARLING_AGE: u32 = 12 * 28 * 2;

    let age_mortality = Arc::new(Mutex::new(&mut model.global_variables.age_mortality));

    let groups = &mut model.groups;
    let roamers = &mut model.roamers;
    let dispersers = &mut model.dispersers;

    scope(|s| {
        // Process groups in parallel
        let age_mortality_clone = Arc::clone(&age_mortality);
        s.spawn(move |_| {
            for group in groups {
                for member in &mut group.group_members {
                    member.age += 1;
                    member.age_class = if member.age < PIGLET_AGE {
                        AgeClass::Piglet
                    } else if member.age < YEARLING_AGE {
                        AgeClass::Yearling
                    } else {
                        AgeClass::Adult
                    };

                    if member.age >= MAX_AGE {
                        let mut age_mortality = age_mortality_clone.lock().unwrap();
                        **age_mortality += 1;
                    }
                }
                group.group_members.retain(|member| member.age < MAX_AGE);
            }
        });

        // Process roamers in parallel
        let age_mortality_clone = Arc::clone(&age_mortality);
        s.spawn(move |_| {
            for roamer in &mut * roamers {
                roamer.age += 1;
                if roamer.age_class != AgeClass::Adult {
                 roamer.age_class = if roamer.age < PIGLET_AGE {
                     AgeClass::Piglet
                 } else if roamer.age < YEARLING_AGE {
                     AgeClass::Yearling
                 } else {
                    // println!("Roamer {} grew up: {}", roamer.individual_id ,roamer.age);
                     AgeClass::Adult
                 };
                }

                if roamer.age >= MAX_AGE {
                    let mut age_mortality = age_mortality_clone.lock().unwrap();
                    **age_mortality += 1;
                }
            }
            roamers.retain(|roamer| roamer.age < MAX_AGE);
        });

        // Process dispersers in parallel
        let age_mortality_clone = Arc::clone(&age_mortality);
        s.spawn(move |_| {
            for disperser_group in dispersers {
                for disperser in &mut disperser_group.dispersing_individuals {
                    disperser.age += 1;
                    disperser.age_class = if disperser.age < PIGLET_AGE {
                        AgeClass::Piglet
                    } else if disperser.age < YEARLING_AGE {
                        AgeClass::Yearling
                    } else {
                        AgeClass::Adult
                    };

                    if disperser.age >= MAX_AGE {
                        let mut age_mortality = age_mortality_clone.lock().unwrap();
                        **age_mortality += 1;
                    }
                }
                disperser_group.dispersing_individuals.retain(|disperser| disperser.age < MAX_AGE);
            }
        });
    }).unwrap();
}
