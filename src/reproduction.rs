

use crate::*;


pub fn reproduction(month: u32, groups: &mut Vec<Groups>, current_tick: usize, year_modifier: bool) {
    let reproduction_probability = match month {
        1 => 0.06,
        2 => 0.16,
        3 => 0.39,
        4 => 0.73,
        5 => 0.80,
        6 => 0.88,
        7 => 0.94,
        8 => 0.97,
        9 => 1.00,
        _ => 0.0,
    };

    let mut range_max = 5;
    let mut range_min = 1;

    if year_modifier {
        range_max = 10;
        range_min = 5;
    }

    for group in groups.iter_mut() {

        // Calculate the difference of current group size with the max group size

        let group_size = group.group_members.len() as i32;
        let max_group_size = group.max_size as i32;
        let group_size_difference: i32 = max_group_size - group_size;

        // increase the min and max range variables based on the group size difference, e.g. if the difference is 10% or less dont change the value but for every 10% difference increase the range by 1 and half that for the min range

        if group_size_difference > 0 {
            let range_increase = (group_size_difference as f64 / max_group_size as f64) * 10.0;
            range_max += range_increase as i32;
            range_min += (range_increase / 2.0) as i32;
        }


        // Collect indices of members to be reproduced
        let members_to_reproduce_indices: Vec<usize> = group
            .group_members
            .iter()
            .enumerate()
            .filter(|(_, mem)| mem.has_reproduced && mem.age_class == AgeClass::Adult && mem.sex == Sex::Female)
            .map(|(i, _)| i)
            .collect();

        // Update members that need reproduction
        for index in members_to_reproduce_indices {
            let member = &mut group.group_members[index];
            if member.time_of_reproduction + (28 * 12) == current_tick {
                member.time_of_reproduction = 0;
                member.has_reproduced = false;
            }
        }

        // Collect indices of eligible members for reproduction
        let eligible_members_indices: Vec<usize> = group
            .group_members
            .iter()
            .enumerate()
            .filter(|(_, mem)| {
                mem.sex == Sex::Female
                    && !mem.has_reproduced
                    && mem.age_class == AgeClass::Adult
                    && rand::thread_rng().gen_bool(reproduction_probability)
            })
            .map(|(i, _)| i)
            .collect();

        // Add new members
        for index in eligible_members_indices {
            let num_new_members = rand::thread_rng().gen_range(range_min..range_max);

            for _ in 0..num_new_members {
                let new_sex = if rand::thread_rng().gen_bool(0.5) {
                    Sex::Female
                } else {
                    Sex::Male
                };

                let new_member = GroupMember {
                    individual_id: generate_individual_id(),
                    age: 0,
                    age_class: AgeClass::Piglet,
                    sex: new_sex,
                    health_status: HealthStatus::Susceptible,
                    time_of_birth: current_tick,
                    has_reproduced: false,
                    time_of_reproduction: 0,
                    origin_group_id: group.group_id,
                    has_dispersed: false,
                    current_group_id: group.group_id,
                };

                group.group_members.push(new_member);
            }

            // Mark the original members as having reproduced and record time
            group.group_members[index].has_reproduced = true;
            group.group_members[index].time_of_reproduction = current_tick;
        }
    }
}