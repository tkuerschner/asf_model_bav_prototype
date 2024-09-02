
use crossbeam::thread::scope;
use crate::*;

//pub fn within_group_pathogen_infection(model: &mut Model, rng: &mut impl Rng){
//
//
//}
#[derive(Debug, Clone, PartialEq)]
pub enum InfectionStage {
    Incubation,
    Symptomatic,
    HighlyInfectious,
    Recovered,
    Vaccinated,
    Dead,
    NotInfected,
}

impl fmt::Display for InfectionStage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfectionStage::Incubation => write!(f, "Incubation"),
            InfectionStage::Symptomatic => write!(f, "Symptomatic"),
            InfectionStage::HighlyInfectious => write!(f, "HighlyInfectious"),
            InfectionStage::Recovered => write!(f, "Recovered"),
            InfectionStage::Vaccinated => write!(f, "Vaccinated"),
            InfectionStage::Dead => write!(f, "Dead"),
            InfectionStage::NotInfected => write!(f, "NotInfected"),
        }
    }
}

pub fn infected_in_group(model: &mut Model, _rng: &mut impl Rng, group_nr: usize) -> bool {
    let mut infected = false;
    // look into group with group_nr == group_id
    let group = model.groups.iter_mut().find(|group| group.group_id == group_nr).unwrap();
        //iterate though group members
        for member in group.group_members.iter_mut() {
            if member.health_status == HealthStatus::Infected && (member.infection_stage == InfectionStage::Symptomatic || member.infection_stage == InfectionStage::HighlyInfectious) {
                infected = true;
                break;
            }
        }
    infected
}

pub fn pathogen_transmission_within_groups(model: &mut Model, rng: &mut impl Rng) {
    // Iterate through all groups
    for group in model.groups.iter_mut() {
        // Collect indices of infected members
        let infected_indices: Vec<usize> = group.group_members
            .iter()
            .enumerate()
            .filter_map(|(index, member)| {
                if member.health_status == HealthStatus::Infected && member.infection_stage == InfectionStage::Symptomatic {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let highly_infectious_indices: Vec<usize> = group.group_members
            .iter()
            .enumerate()
            .filter_map(|(index, member)| {
                if member.health_status == HealthStatus::Infected && member.infection_stage == InfectionStage::HighlyInfectious {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

            // density factor for infected individuals
            let infection_density_factor = infected_indices.len()  as f64 / group.group_members.len() as f64;
            // density factor for highly infectious individuals
            let highly_infectious_density_factor = highly_infectious_indices.len() as f64 / (group.group_members.len() * 10) as f64; //FIX ME
            // if there are infected in the group
         
            if infected_indices.len() + highly_infectious_indices.len() > 0 {
             // Iterate through all group members (to potentially infect)
            for member in group.group_members.iter_mut() {
                // Check if member is susceptible and not already infected
                if member.health_status == HealthStatus::Susceptible {
                    // Calculate the transmission probability based on the number of symptomatic and highly infectious individuals
                    let transmission_probability = BETA_W * (infection_density_factor + highly_infectious_density_factor);

                    // Check if transmission is successful
                    if rng.gen_bool(transmission_probability) {
                        member.health_status = HealthStatus::Infected;
                        member.infection_stage = InfectionStage::Incubation;
                        member.time_of_infection = Some(model.global_variables.current_time);
                    }
                }
            }

        }
    }
}


pub fn pathogen_progression(model: &mut Model, rng: &mut StdRng) {
    let current_time = model.global_variables.current_time;
    let p_symptomatic = P_SYMPTOMATIC;

    // Create a thread-safe RNG by cloning the main RNG for each thread
    scope(|s| {
        // Process groups
        for group in &mut model.groups {
            let mut rng_clone = rng.clone();
            s.spawn(move |_| {
                for member in &mut group.group_members {
                    if member.health_status == HealthStatus::Infected {
                        //println!("{} ", member.infection_stage);
                        if let Some(time_of_infection) = member.time_of_infection {
                            let time_since_infection = current_time - time_of_infection;
                            //println!("{} ", time_since_infection);
                            if time_since_infection >= 7 {
                                if rng_clone.gen_bool(p_symptomatic) {
                                    member.infection_stage = InfectionStage::Symptomatic;
                                    //println!("{} ", member.infection_stage);
                                } else {
                                    member.infection_stage = InfectionStage::Recovered;
                                }
                            }
                            if time_since_infection >= 10 {
                                member.infection_stage = InfectionStage::HighlyInfectious;
                            }
                            if time_since_infection >= 14 {
                                member.infection_stage = InfectionStage::Dead;
                            }
                        }
                    }
                }
            });
        }

        // Process roamers
        for roamer in &mut model.roamers {
            let mut rng_clone = rng.clone();
            s.spawn(move |_| {
                if roamer.health_status == HealthStatus::Infected {
                    if let Some(time_of_infection) = roamer.time_of_infection {
                        let time_since_infection = current_time - time_of_infection;
                        if time_since_infection >= 7 {
                            if rng_clone.gen_bool(p_symptomatic) {
                                roamer.infection_stage = InfectionStage::Symptomatic;
                            } else {
                                roamer.infection_stage = InfectionStage::Recovered;
                            }
                            if time_since_infection >= 10 {
                                roamer.infection_stage = InfectionStage::HighlyInfectious;
                            }
                            if time_since_infection >= 14 {
                                roamer.infection_stage = InfectionStage::Dead;
                            }
                        }
                    }
                }
            });
        }

        // Process dispersers
        for disperser in &mut model.dispersers {
            let mut rng_clone = rng.clone();
            s.spawn(move |_| {
                for d_member in &mut disperser.dispersing_individuals {
                    if d_member.health_status == HealthStatus::Infected {
                        if let Some(time_of_infection) = d_member.time_of_infection {
                            let time_since_infection = current_time - time_of_infection;
                            if time_since_infection >= 7 {
                                if rng_clone.gen_bool(p_symptomatic) {
                                    d_member.infection_stage = InfectionStage::Symptomatic;
                                } else {
                                    d_member.infection_stage = InfectionStage::Recovered;
                                }
                            }
                            if time_since_infection >= 10 {
                                d_member.infection_stage = InfectionStage::HighlyInfectious;
                            }
                            if time_since_infection >= 14 {
                                d_member.infection_stage = InfectionStage::Dead;
                            }
                        }
                    }
                }
            });
        }
    }).unwrap();

}


pub fn remove_dead_individuals(model: &mut Model) {
    // Vectors to collect the data of dead individuals
    let mut dead_group_members: Vec<(usize, GroupMember)> = Vec::new();
    let mut dead_roamers: Vec<RoamingIndividual> = Vec::new();
    let mut dead_dispersers: Vec<(usize, DispersingIndividual)> = Vec::new();

    // Collect dead group members
    for (group_index, group) in model.groups.iter_mut().enumerate() {
        let members: Vec<_> = group.group_members.iter()
            .filter(|member| member.infection_stage == InfectionStage::Dead)
            .cloned()
            .collect();

        for member in members {
            dead_group_members.push((group_index, member));
        }
    }

    // Collect dead roamers
    let roamers: Vec<_> = model.roamers.iter()
        .filter(|roamer| roamer.infection_stage == InfectionStage::Dead)
        .cloned()
        .collect();

    dead_roamers.extend(roamers);

    // Collect dead dispersers
    for (disperser_index, disperser) in model.dispersers.iter_mut().enumerate() {
        let members: Vec<_> = disperser.dispersing_individuals.iter()
            .filter(|d_member| d_member.infection_stage == InfectionStage::Dead)
            .cloned()
            .collect();

        for d_member in members {
            dead_dispersers.push((disperser_index, d_member));
        }
    }

    // Create carcasses for dead group members
    for (group_index, member) in dead_group_members.iter() {
        create_carcass(member, model);
    }

    // Create carcasses for dead roamers
    for roamer in dead_roamers.iter() {
        create_carcass(roamer, model);
    }

    // Create carcasses for dead dispersers
    for (disperser_index, d_member) in dead_dispersers.iter() {
        create_carcass(d_member, model);
    }

    // Remove dead group members
    for (group_index, member) in dead_group_members.into_iter().rev() {
        let group = &mut model.groups[group_index];
        group.group_members.retain(|m| m.individual_id != member.individual_id);
    }

    // Remove dead roamers
    model.roamers.retain(|roamer| {
        !dead_roamers.iter().any(|r| r.individual_id == roamer.individual_id)
    });

    // Remove dead dispersers
    for (disperser_index, d_member) in dead_dispersers.into_iter().rev() {
        let disperser = &mut model.dispersers[disperser_index];
        disperser.dispersing_individuals.retain(|m| m.individual_id != d_member.individual_id);
    }
}




   // // create carcasses
   // for group in &mut model.groups {
   //     for member in &mut group.group_members {
   //         if member.infection_stage == InfectionStage::Dead {
   //             create_carcass(member, model)
   //         }
   //     }
   // }
//
//
   // 
   // // Remove dead individuals from groups
   // for group in &mut model.groups {
   //     group.group_members.retain(|member| member.infection_stage != InfectionStage::Dead);
   // }
//
   // // Remove dead individuals from roamers
   // model.roamers.retain(|roamer| roamer.infection_stage != InfectionStage::Dead);
//
   // // Remove dead individuals from dispersers
   // for disperser in &mut model.dispersers {
   //     disperser.dispersing_individuals.retain(|d_member| d_member.infection_stage != InfectionStage::Dead);
   // }



pub fn experimental_outbreak(model: &mut Model){

    for c in model.carcasses.iter_mut(){
        // 50 50 chance of carcass being infected
        let infected = rand::thread_rng().gen_bool(0.25);
        if infected {
            c.is_infected = true;
        }

        //c.is_infected = true;
    }

}

pub fn experimental_outbreak2(model: &mut Model){

    for c in model.carcasses.iter_mut(){
        // 50 50 chance of carcass being infected
        let infected = rand::thread_rng().gen_bool(0.01);
        if infected {
            c.is_infected = true;
        }

        //c.is_infected = true;
    }

}