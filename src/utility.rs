use crate::*;


//function check if a disperser group has no members

pub fn check_empty_disperser_group(dispersing_groups: &mut Vec<DispersingFemaleGroup>) {
    
    for dg in dispersing_groups.iter() {
        if dg.dispersing_individuals.len() == 0 {
            println!("The dispersing group {} is empty", dg.disp_grp_id);
        }
    }
   
}
