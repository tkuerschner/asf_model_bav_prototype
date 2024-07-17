use crate::*;

//pub fn within_group_pathogen_infection(model: &mut Model, rng: &mut impl Rng){
//
//
//}


pub fn infected_in_group(model: &mut Model, _rng: &mut impl Rng, group_nr: usize) -> bool {
    let mut infected = false;
    // look into group with group_nr == group_id
    let group = model.groups.iter_mut().find(|group| group.group_id == group_nr).unwrap();
        //iterate though group members
        for member in group.group_members.iter_mut() {
            if member.health_status == HealthStatus::Infected {
                infected = true;
                break;
            }
        }
    infected
}


//pub fn pathogen_transmission(model: &mut Model, rng: &mut impl Rng){
//    // iterate through all groups
//    for group in model.groups.iter_mut() {
//        // iterate through all group members
//        for member in group.group_members.iter_mut() {
//            // check if member is infected
//            if member.health_status == HealthStatus::Infected {
//                // iterate through all group members
//                for other_member in group.group_members.iter_mut() {
//                    // check if other member is not infected
//                    if other_member.health_status == HealthStatus::Susceptible {
//                        // check if transmission is successful
//                        if rng.gen_bool(model.transmission_rate) {
//                            other_member.health_status = HealthStatus::Infected;
//                        }
//                    }
//                }
//            }
//        }
//    }
//}
