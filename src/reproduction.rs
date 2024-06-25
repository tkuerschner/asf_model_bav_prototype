

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

    if year_modifier {
        range_max = 10;
    }

    for group in groups.iter_mut() {
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
            let num_new_members = rand::thread_rng().gen_range(1..range_max);

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