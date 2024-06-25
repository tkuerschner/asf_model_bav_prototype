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