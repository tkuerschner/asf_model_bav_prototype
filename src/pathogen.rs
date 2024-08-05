
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
            if member.health_status == HealthStatus::Infected && member.infection_stage == InfectionStage::Symptomatic {
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

            let infection_density_factor = infected_indices.len() as f64 / group.group_members.len() as f64;

        // Iterate over infected members by their indices
        for &infected_index in infected_indices.iter() {
            // Check if member is infected
            if group.group_members[infected_index].health_status == HealthStatus::Infected && group.group_members[infected_index].infection_stage == InfectionStage::Symptomatic {
                // Iterate through all group members (to potentially infect)
                for member in group.group_members.iter_mut() {
                    // Check if member is susceptible and not already infected
                    if member.health_status == HealthStatus::Susceptible {
                        // Check if transmission is successful
                        if rng.gen_bool(BETA_W * infection_density_factor) {
                            member.health_status = HealthStatus::Infected;
                            member.time_of_infection = Some(model.global_variables.current_time);
                        }
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
                        if let Some(time_of_infection) = member.time_of_infection {
                            let time_since_infection = current_time - time_of_infection;
                            if time_since_infection >= 7 {
                                if rng_clone.gen_bool(p_symptomatic) {
                                    member.infection_stage = InfectionStage::Symptomatic;
                                } else {
                                    member.infection_stage = InfectionStage::Recovered;
                                }
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

