use crate::*;
use uuid::Uuid;


pub fn progress_time(global_variables: &mut GlobalVariables) {
    // Increment the day
    global_variables.day += 1;

    // Check if a month has passed (28 days in a month)
    if global_variables.day > 28 {
        global_variables.day = 1;
        global_variables.month += 1;

        // Check if a year has passed (12 months in a year)
        if global_variables.month > 12 {
            global_variables.month = 1;
            global_variables.year += 1;
        }
    }
}


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
        if random_number <= CONFIG.good_year_chance {
            model.global_variables.good_year = true;
        }
    } else {
        model.global_variables.good_year = false;
    }

}

// function to remove half of all group member and half of all roamers

//pub fn remove_half_of_all_groups(model: &mut Model) {
//  
//    //set the age of half of all group members and roamers to MAX_AGE + 5
//    for group in model.groups.iter_mut() {
//        let half_group_size = group.group_members.len() / 2;
//        for i in 0..half_group_size {
//            group.group_members[i].age = MAX_AGE + 5;
//        }
//    }
//
//    let half_roamers_size = model.roamers.len() / 2;
//    for i in 0..half_roamers_size {
//        model.roamers[i].age = MAX_AGE + 5;
//    }
//}

pub fn reset_group_coordinates_to_core_cell(group: &mut Groups) -> (usize, usize) {
    group.x = group.core_cell.unwrap().0;
    group.y = group.core_cell.unwrap().1;
    (group.x, group.y)
}

 // This function should return global variable current_time
//pub fn current_time(model: &mut Model) -> usize {
//
//   model.global_variables.current_time
//
//
 
pub fn generate_unique_simulation_id() -> String {
    // Generate a UUID
    let uuid = Uuid::new_v4();
    // Convert UUID to string and remove hyphens
    uuid.hyphenated().to_string().replace("-", "")
}

pub fn copy_last_sim_to_active(folder_path: String) {
    let output_folder = Path::new("output");
    let output_files = fs::read_dir(folder_path).unwrap();
    for file in output_files {
        let file = file.unwrap();
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let output_file = output_folder.join(file_name);
        fs::copy(path, output_file).expect("Failed to copy file to output folder");
    }
    println!("Last simulation copied to active folder");
    }

    pub fn generate_iteration_sim_output_row(model: &mut Model){
        let stage_count = count_infection_stage(model);
        let row = vec![
            model.global_variables.current_time.to_string(),
            model.global_variables.day.to_string(),
            model.global_variables.month.to_string(),
            model.global_variables.year.to_string(),
            model.global_variables.good_year.to_string(),
            count_all_group_members(model).to_string(),
            count_all_roamers(model).to_string(),
            count_all_dispersing_individuals(model).to_string(),
            count_infected_individuals(model).to_string(),
            stage_count.0.to_string(),
            stage_count.1.to_string(),
            stage_count.2.to_string(),
            stage_count.3.to_string(),
            stage_count.4.to_string(),
            stage_count.5.to_string(),
        ];
        model.metadata.iteration_output.push(row);

    }

    pub fn count_all_group_members(model: &mut Model) -> usize {
        let mut member_count = 0;
        //let mut female_member_count = 0;
        //let mut male_member_count = 0;
        //let mut female_adult_count: usize = 0;
        //let mut male_adult_count: usize = 0;
        //let mut female_adult_count: usize = 0;
        //let mut male_piglet_count: usize = 0;

        for group in model.groups.iter() {
            member_count += group.group_members.len();
        }
        member_count
    }

    pub fn count_all_roamers(model: &mut Model) -> usize {
        model.roamers.len()
    }

    pub fn count_all_dispersing_individuals(model: &mut Model) -> usize {
        let mut count = 0;
        for disperser in model.dispersers.iter() {
            count += disperser.dispersing_individuals.len();
        }
        count
    }

    pub fn count_infected_individuals(model: &mut Model) -> usize {
        let mut count = 0;
        for group in model.groups.iter() {
            for member in group.group_members.iter() {
                if member.health_status == HealthStatus::Infected {
                    count += 1;
                }
            }
        }
        for roamer in model.roamers.iter() {
            if roamer.health_status == HealthStatus::Infected {
                count += 1;
            }
        }
        for disperser in model.dispersers.iter() {
            for member in disperser.dispersing_individuals.iter() {
                if member.health_status == HealthStatus::Infected {
                    count += 1;
                }
            }
        }
        count
    }

    // count the number of individuals per InfectionStage
    pub fn count_infection_stage(model: &mut Model) -> (usize, usize, usize, usize, usize, usize) {
        let mut incubation = 0;
        let mut symptomatic = 0;
        let mut infected = 0;
        let mut recovered = 0;
        let mut dead = 0;
        let mut susceptible = 0;
        for group in model.groups.iter() {
            for member in group.group_members.iter() {
                match member.infection_stage {
                    InfectionStage::Incubation => incubation += 1,
                    InfectionStage::Symptomatic => symptomatic += 1,
                    InfectionStage::HighlyInfectious => infected += 1,
                    InfectionStage::Recovered => recovered += 1,
                    InfectionStage::Dead => dead += 1,
                    InfectionStage::NotInfected => susceptible += 1,
                    _ => (),
                }
            }
        }
        for roamer in model.roamers.iter() {
            match roamer.infection_stage {
                InfectionStage::Incubation => incubation += 1,
                InfectionStage::Symptomatic => symptomatic += 1,
                InfectionStage::HighlyInfectious => infected += 1,
                InfectionStage::Recovered => recovered += 1,
                InfectionStage::Dead => dead += 1,
                InfectionStage::NotInfected => susceptible += 1,
                _ => (),
            }
        }
        for disperser in model.dispersers.iter() {
            for member in disperser.dispersing_individuals.iter() {
                match member.infection_stage {
                    InfectionStage::Incubation => incubation += 1,
                    InfectionStage::Symptomatic => symptomatic += 1,
                    InfectionStage::HighlyInfectious => infected += 1,
                    InfectionStage::Recovered => recovered += 1,
                    InfectionStage::Dead => dead += 1,
                    InfectionStage::NotInfected => susceptible += 1,
                    _ => (),
                }
            }
        }
        (incubation, symptomatic, infected, recovered, dead, susceptible)
    }

    pub fn read_config(filename: &str) -> Config {
        let data = fs::read_to_string(filename).expect("Unable to read file");
        serde_json::from_str(&data).expect("Unable to parse JSON")
    }
    