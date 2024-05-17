use crate::AgeClass;
use crate::Groups;
use crate::MAX_AGE;


pub fn ageing(groups: &mut Vec<Groups>, age_mortality: &mut u32) {
    for group in groups.iter_mut() {
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
                *age_mortality += 1;
                continue;
            }
        }
        // Remove members whose age exceeds the maximum age
        group.group_members.retain(|member| member.age < MAX_AGE);
    }
}

