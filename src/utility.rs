use crate::*;


//function check if a disperser group has no members

pub fn check_empty_disperser_group(dispersing_groups: &mut Vec<DispersingFemaleGroup>) {
    
    for dg in dispersing_groups.iter() {
        if dg.dispersing_individuals.len() == 0 {
            println!("The dispersing group {} is empty", dg.disp_grp_id);
        }
    }
   
}


pub fn good_year_check(model: &mut Model, rng: &mut impl Rng ){

    if model.global_variables.good_year == false {
        let random_number = rng.gen_range(0..100);
        if random_number <= GODD_YEAR_CHANCE {
            model.global_variables.good_year = true;
        }
    } else {
        model.global_variables.good_year = false;
    }

}

// function to remove half of all group member and half of all roamers

pub fn remove_half_of_all_groups(model: &mut Model) {
  
    //set the age of half of all group members and roamers to MAX_AGE + 5
    for group in model.groups.iter_mut() {
        let half_group_size = group.group_members.len() / 2;
        for i in 0..half_group_size {
            group.group_members[i].age = MAX_AGE + 5;
        }
    }

    let half_roamers_size = model.roamers.len() / 2;
    for i in 0..half_roamers_size {
        model.roamers[i].age = MAX_AGE + 5;
    }
}

pub fn reset_group_coordinates_to_core_cell(group: &mut Groups) -> (usize, usize) {
    group.x = group.core_cell.unwrap().0;
    group.y = group.core_cell.unwrap().1;
    (group.x, group.y)
}
