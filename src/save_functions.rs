// Saving Output functions

use crate::*;
//use bson::{to_bson, Bson, doc,Document};
use std::fs::File;
use std::io::{self, Write, BufWriter};
use rayon::prelude::*;
//use std::string;

pub fn save_outputs(
    folder_name: &String,
    all_grid_states: Vec<(usize, Vec<Vec<Cell>>)>,
    all_group_states: Vec<(usize, Vec<Groups>)>,
    all_global_variables: Vec<GlobalVariables>,
    all_disperser_states: Vec<(usize, Vec<DispersingFemaleGroup>)>,
    all_roamer_states: Vec<(usize, Vec<RoamingIndividual>)>,
    all_carcass_states: Vec<(usize, Vec<Carcass>)>,
    all_high_seat_states: Vec<(usize, Vec<HighSeat>)>,
    all_hunting_statistics: Vec<(usize, HuntingStatistics)>,
    all_interaction_layers: Vec<(usize, InteractionLayer)>,
    folder_path: String,
    meta_data: Vec<(usize, SimMetaData)>,
) {
    
    let now = chrono::Utc::now();
    // Save all grid states to a single CSV file
    save_grid_as_csv(format!("output/{}/all_grid_states.csv", folder_name).as_str(), &all_grid_states).expect("Failed to save grid states as CSV");

    // Save all individual states to a single CSV file
    //save_groups_as_csv(format!("output/{}/all_groups.csv", folder_name).as_str(), &all_group_states).expect("Failed to save groups as CSV");

    // Save all global variables to a single CSV file
    save_global_variables_as_csv(format!("output/{}/all_global_variables.csv", folder_name).as_str(), &all_global_variables).expect("Failed to save global variables as CSV");

    // Save all disperser states to a single CSV file
    save_disperser_group_as_csv(format!("output/{}/all_dispersers.csv", folder_name).as_str(), &all_disperser_states).expect("Failed to save disperser as CSV");

    // Save all roamer states to a single CSV file
    save_roamers_as_csv(format!("output/{}/all_roamers.csv", folder_name).as_str(), &all_roamer_states).expect("Failed to save roamer as CSV");

    // Save all interaction layer to a single CSV file
    save_interaction_layer_as_csv(format!("output/{}/all_interaction_layer.csv", folder_name).as_str(), &all_interaction_layers).expect("Failed to save interaction layer as CSV");

    // Save all carcass states to a
    save_carcasses_as_csv(format!("output/{}/all_carcasses.csv", folder_name).as_str(), &all_carcass_states).expect("Failed to save carcasses as CSV");

    // Save all high seat states to a
    save_high_seats_as_csv(format!("output/{}/all_high_seats.csv", folder_name).as_str(), &all_high_seat_states).expect("Failed to save high seats as CSV");

    // Save all hunting statistics to a
    save_hunting_statistics_as_csv(format!("output/{}/all_hunting_statistics.csv", folder_name).as_str(), &all_hunting_statistics, &all_global_variables).expect("Failed to save hunting statistics as CSV");

    save_sim_meta_data_row_output_as_csv(format!("output/{}/all_sim_output_data.csv", folder_name).as_str(), &meta_data).expect("Failed to save sim meta data as CSV");


    save_groups_as_csv_parallel(format!("output/{}/all_groups.csv", folder_name).as_str(), &all_group_states).expect("Failed to save groups as CSV");

    //calculate the time taken to save the outputs
    let end = chrono::Utc::now();
    let duration = end.signed_duration_since(now);
    println!("Time taken to save outputs: {:?}", duration);

    //copy all the files from the specific output folder to the output folder
    copy_last_sim_to_active(folder_path);
}



//pub fn save_groups_as_csv(filename: &str, group_states: &[(usize, Vec<Groups>)]) -> io::Result<()> {
//    // Create or open the CSV file
//    let mut file = File::create(filename)?;
//
//    // Write the header line
//    writeln!(file, "iteration,individual_id,group_id,x,y,sex,age,age_class,known_cells,target_cell,core_cell,movement_type,remaining_stay_time,origin_group,ap_list,infection_stage,health_status")?;//,group_member_ids")?;
//
//    // Write each individual's data for each iteration
//    for (iteration, groups) in group_states {
//        for group in groups {
//            for group_members in &group.group_members {
//                // Convert variables to strings for CSV output
//                let known_cells_str: String = group
//                    .memory
//                    .known_cells_order
//                    .iter()
//                    .map(|&(x, y)| format!("[{}_{}]", x, y))
//                    .collect::<Vec<String>>()
//                    .join(";");
//
//                let ap_list_str: String = group
//                    .current_ap
//                    .iter()
//                    .map(|&(x, y)| format!("[{}_{}]", x, y))
//                    .collect::<Vec<String>>()
//                    .join(";");
//
//                //let group_member_ids_str: String = format!(
//                //    "[{}]",
//                //    group_members
//                //        .memory
//                //        .group_member_ids
//                //        .iter()
//                //        .map(|&id| id.to_string())
//                //        .collect::<Vec<String>>()
//                //        .join(";")
//                //);
//
//                //let last_three_cells_str: String = individual
//                //    .memory
//                //    .last_visited_cells_order
//                //    .iter()
//                //    .map(|&(x, y)| format!("[{}_{}]", x, y))
//                //    .collect::<Vec<String>>()
//                //    .join(";");
//
//                let target_cell_str: String = group
//                    .target_cell
//                    .iter()
//                    .map(|&(x, y)| format!("[{}_{}]", x, y))
//                    .collect::<Vec<String>>()
//                    .join(";");
//
//                let core_cell_str: String = group
//                    .core_cell
//                    .iter()
//                    .map(|&(x, y)| format!("[{}_{}]", x, y))
//                    .collect::<Vec<String>>()
//                    .join(";");
//
//                let age_class_str: String = format!("{}", group_members.age_class);
//                let sex_str: String = format!("{}", group_members.sex);
//                let stage_string: String = format!("{}", group_members.infection_stage);
//                let infected_string: String = format!("{}", group_members.health_status);
//                //let target_cell_str: String = format!("[{:?}]", group.target_cell);
//                //let core_cell_str: String = format!("[{:?}]", group.core_cell);
//                //let remaining_stay_stime_str: String = format!("{}", group.remaining_stay_time);
//                
//                writeln!(
//                    file,
//                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
//                    iteration,
//                    group_members.individual_id,
//                    group.group_id,
//                    group.x,
//                    group.y,
//                    sex_str,
//                    group_members.age,
//                    age_class_str,
//                    known_cells_str,
//                    target_cell_str,
//                    core_cell_str,
//                    group.movement,
//                    group.remaining_stay_time,
//                    group_members.origin_group_id,
//                    ap_list_str,
//                    stage_string,
//                    infected_string,
//                    //remaining_stay_stime_str,
//                    //group_member_ids_str,
//                    //last_three_cells_str
//                )?;
//            }
//        }
//    }
//
//    println!("Groups saved to: {}", filename);
//    Ok(())
//}


/* 
pub fn save_grid_as_csv(filename: &str, grid_states: &[(usize, Vec<Vec<Cell>>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,x,y,quality,counter,x_grid_corrected,y_grid_corrected,is_ap,is_territory,territory_of_group,hunting_zone")?;

    // Write each cell's data for each iteration
    for (iteration, grid) in grid_states {
        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                writeln!(file, "{},{},{},{},{},{},{},{},{},{},{}", iteration, x, y, cell.quality, cell.counter, cell.x_grid, cell.y_grid, cell.territory.is_ap, cell.territory.is_taken, cell.territory.taken_by_group,cell.hunting_zone)?;
            }
        }
    }

    println!("Grid states saved to: {}", filename);
    Ok(())
}
*/

pub fn save_grid_as_csv(filename: &str, grid_states: &[(usize, Vec<Vec<Cell>>)]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "iteration,x,y,quality,counter,x_grid_corrected,y_grid_corrected,is_ap,is_territory,territory_of_group,hunting_zone")?;

    // Collect all the lines in parallel
    let csv_lines: Vec<String> = grid_states
        .par_iter()
        .flat_map(|(iteration, grid)| {
            grid.par_iter().enumerate().flat_map(move |(x, row)| {
                row.par_iter().enumerate().map(move |(y, cell)| {
                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{}",
                        iteration,
                        x,
                        y,
                        cell.quality,
                        cell.counter,
                        cell.x_grid,
                        cell.y_grid,
                        cell.territory.is_ap,
                        cell.territory.is_taken,
                        cell.territory.taken_by_group,
                        cell.hunting_zone
                    )
                })
            })
        })
        .collect();

    // Write all the collected lines sequentially
    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("Grid states saved to: {}", filename);
    Ok(())
}

pub fn save_global_variables_as_csv(filename: &str, global_variables: &[GlobalVariables]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,day,month,year,n_individuals,age_mortality,random_mortality,overcap_mortality,n_groups,n_roamers,n_dispersers,good_year")?;

    // Write each iteration's global variables
    for (iteration, globals) in global_variables.iter().enumerate() {
        writeln!(file, "{},{},{},{},{},{},{},{},{},{},{},{}", 
        iteration, 
        globals.day, 
        globals.month, 
        globals.year, 
        globals.n_individuals, 
        globals.age_mortality, 
        globals.random_mortality, 
        globals.overcapacity_mortality,
        globals.n_groups,
        globals.n_roamers,
        globals.n_dispersers,
        globals.good_year,
    
    )?;
    }

    println!("Global variables saved to: {}", filename);
    Ok(())
}


/* 
pub fn save_disperser_group_as_csv(filename: &str, disperser_states: &[(usize, Vec<DispersingFemaleGroup>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,individual_id,disperser_id,age,age_class,sex,health_status,origin_group_id,disperser_group_x,disperser_group_y,disperser_group_id,infection_stage")?;

    // Write each disperser's data for each iteration
    for (iteration, disperser_groups) in disperser_states {
        for disperser_group in disperser_groups {
            for disperser_group_member in disperser_group.dispersing_individuals.iter() {

                let string_infection_stage: String = format!("{}", disperser_group_member.infection_stage);

                writeln!(
                    file,
                    "{},{},{},{},{},{},{},{},{},{},{},{}",
                    iteration,
                    disperser_group_member.individual_id,
                    disperser_group_member.disperser_id,
                    disperser_group_member.age,
                    disperser_group_member.age_class,
                    disperser_group_member.sex,
                    disperser_group_member.health_status,
                    disperser_group_member.origin_group_id,
                    disperser_group.disp_grp_x,
                    disperser_group.disp_grp_y,
                    disperser_group.disp_grp_id,
                    string_infection_stage,                    
          
            )?;
        }
    }
}

    println!("Disperser states saved to: {}", filename);
    Ok(())
}
*/

pub fn save_disperser_group_as_csv(filename: &str, disperser_states: &[(usize, Vec<DispersingFemaleGroup>)]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "iteration,individual_id,disperser_id,age,age_class,sex,health_status,origin_group_id,disperser_group_x,disperser_group_y,disperser_group_id,infection_stage")?;

    // Collect all the lines in parallel
    let csv_lines: Vec<String> = disperser_states
        .par_iter()
        .flat_map(|(iteration, disperser_groups)| {
            disperser_groups.par_iter().flat_map(move |disperser_group| {
                disperser_group.dispersing_individuals.par_iter().map(move |disperser_group_member| {
                    let string_infection_stage = format!("{}", disperser_group_member.infection_stage);

                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{},{}",
                        iteration,
                        disperser_group_member.individual_id,
                        disperser_group_member.disperser_id,
                        disperser_group_member.age,
                        disperser_group_member.age_class,
                        disperser_group_member.sex,
                        disperser_group_member.health_status,
                        disperser_group_member.origin_group_id,
                        disperser_group.disp_grp_x,
                        disperser_group.disp_grp_y,
                        disperser_group.disp_grp_id,
                        string_infection_stage,
                    )
                })
            })
        })
        .collect();

    // Write all the collected lines sequentially
    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("Disperser states saved to: {}", filename);
    Ok(())
}

/*
pub fn save_roamers_as_csv(filename: &str, roamer_states: &[(usize, Vec<RoamingIndividual>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    writeln!(file, "iteration,roamer_id,individual_id,age,age_class,sex,health_status,origin_group_id,roamer_x,roamer_y,known_groups,target_group,daily_distance,target_group_id,reached_target,stay_time,staying_with_target_group,target_cell,initial_dispersal,infection_stage")?;

    // Write each roamer's data for each iteration
    for (iteration, roamers) in roamer_states {
        for roamer in roamers {
            let known_groups_str: String = roamer
                .known_groups
                .iter()
                .map(|&id| id.to_string())
                .collect::<Vec<String>>()
                .join(";");

            let target_cell_string: String = roamer
                .target_cell
                .iter()
                .map(|&(x, y)| format!("[{}_{}]", x, y))
                .collect::<Vec<String>>()
                .join(";");

            let stage_string: String = format!("{}", roamer.infection_stage);

            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                iteration,
                roamer.roamer_id,
                roamer.individual_id,
                roamer.age,
                roamer.age_class,
                roamer.sex,
                roamer.health_status,
                roamer.origin_group_id,
                roamer.roamer_x,
                roamer.roamer_y,
                known_groups_str,
                roamer.target_group.unwrap_or(0),
                roamer.daily_distance,
                roamer.target_group_id.unwrap_or(0),
                roamer.reached_target,
                roamer.stay_time,
                roamer.staying_with_target_group,
                target_cell_string,
                roamer.initial_dispersal,
                stage_string,
                
            )?;
        }
    }

    println!("Roamers saved to: {}", filename);
    Ok(())

}
 */

 
pub fn save_roamers_as_csv(filename: &str, roamer_states: &[(usize, Vec<RoamingIndividual>)]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "iteration,roamer_id,individual_id,age,age_class,sex,health_status,origin_group_id,roamer_x,roamer_y,known_groups,target_group,daily_distance,target_group_id,reached_target,stay_time,staying_with_target_group,target_cell,initial_dispersal,infection_stage")?;

    // Collect all the lines in parallel
    let csv_lines: Vec<String> = roamer_states
        .par_iter()
        .flat_map(|(iteration, roamers)| {
            roamers.par_iter().map(move |roamer| {
                let known_groups_str = roamer
                    .known_groups
                    .iter()
                    .map(|&id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(";");

                let target_cell_string = roamer
                    .target_cell
                    .iter()
                    .map(|&(x, y)| format!("[{}_{}]", x, y))
                    .collect::<Vec<String>>()
                    .join(";");

                let stage_string = format!("{}", roamer.infection_stage);

                format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    iteration,
                    roamer.roamer_id,
                    roamer.individual_id,
                    roamer.age,
                    roamer.age_class,
                    roamer.sex,
                    roamer.health_status,
                    roamer.origin_group_id,
                    roamer.roamer_x,
                    roamer.roamer_y,
                    known_groups_str,
                    roamer.target_group.unwrap_or(0),
                    roamer.daily_distance,
                    roamer.target_group_id.unwrap_or(0),
                    roamer.reached_target,
                    roamer.stay_time,
                    roamer.staying_with_target_group,
                    target_cell_string,
                    roamer.initial_dispersal,
                    stage_string,
                )
            })
        })
        .collect();

    // Write all the collected lines sequentially
    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("Roamers saved to: {}", filename);
    Ok(())
}
/* 
pub fn save_interaction_layer_as_csv(filename: &str, interaction_layer_states: &[(usize, InteractionLayer)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,x,y,time,individual_id,group_id,individual_type,time_left,duration,interaction_strength")?;

    // Write each cell's data for each iteration
    for (iteration, interaction_layer) in interaction_layer_states {
        for (&(x, y, time), cell) in interaction_layer {
            for entity in &cell.entities {
                writeln!(
                    file,
                    "{},{},{},{},{},{},{},{},{},{}",
                    iteration,
                    x,
                    y,
                    entity.time,
                    entity.individual_id,
                    entity.group_id,
                    entity.individual_type,
                    entity.time_left,
                    entity.duration,
                    entity.interaction_strength
                )?;
            }
        }
    }

    println!("Interaction layer saved to: {}", filename);
    Ok(())
}
*/


pub fn save_interaction_layer_as_csv(filename: &str, interaction_layer_states: &[(usize, InteractionLayer)]) -> io::Result<()> {
    // Create or open the CSV file
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "iteration,x,y,time,individual_id,group_id,individual_type,time_left,duration,interaction_strength")?;

    // Collect all the lines in parallel
    let csv_lines: Vec<String> = interaction_layer_states
        .par_iter()
        .flat_map(|(iteration, interaction_layer)| {
            let entities: Vec<_> = interaction_layer.iter_entities().collect();  // Collect entities into a Vec
            entities.into_par_iter().map(move |entity| {  // Use `into_par_iter()` to take ownership of `entities`
                format!(
                    "{},{},{},{},{},{},{},{},{},{}",
                    iteration,
                    entity.x,
                    entity.y,
                    entity.time,
                    entity.individual_id,
                    entity.group_id,
                    entity.individual_type,
                    entity.time_left,
                    entity.duration,
                    entity.interaction_strength
                )
            }).collect::<Vec<_>>()  // Collect the inner iterator into a `Vec` so it can be returned
        })
        .collect();

    // Write all the collected lines sequentially
    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("Interaction layer saved to: {}", filename);
    Ok(())
}
/* 
pub fn save_interaction_layer_as_csv(filename: &str, interaction_layer_states: &[(usize, InteractionLayer)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,x,y,time,individual_id,group_id,individual_type,time_left,duration,interaction_strength")?;

    // Write each entity's data for each iteration
    for (iteration, interaction_layer) in interaction_layer_states {
        for entity in interaction_layer.iter_entities() {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{}",
                iteration,
                entity.x,
                entity.y,
                entity.time,
                entity.individual_id,
                entity.group_id,
                entity.individual_type,
                entity.time_left,
                entity.duration,
                entity.interaction_strength
            )?;
        }
    }

    println!("Interaction layer saved to: {}", filename);
    Ok(())
}
*/

/* 
pub fn save_interaction_layer_as_csv(filename: &str, interaction_layer_states: &[(usize, InteractionLayer)]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "iteration,x,y,time,individual_id,group_id,individual_type,time_left,duration,interaction_strength")?;

    // Collect all the lines in parallel
    let csv_lines: Vec<String> = interaction_layer_states
        .par_iter()
        .flat_map(|(iteration, interaction_layer)| {
            interaction_layer.iter_entities().par_iter().map(move |entity| {
                format!(
                    "{},{},{},{},{},{},{},{},{},{}",
                    iteration,
                    entity.x,
                    entity.y,
                    entity.time,
                    entity.individual_id,
                    entity.group_id,
                    entity.individual_type,
                    entity.time_left,
                    entity.duration,
                    entity.interaction_strength
                )
            })
        })
        .collect();

    // Write all the collected lines sequentially
    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("Interaction layer saved to: {}", filename);
    Ok(())
}
*/
/* 
pub fn save_carcasses_as_csv(filename: &str, carcass_states: &[(usize, Vec<Carcass>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,carcass_id,carcass_x,carcass_y,creation_time,is_infected,lifetime,age_class")?;

    // Write each carcass's data for each iteration
    for (iteration, carcasses) in carcass_states {
        for carcass in carcasses {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{}",
                iteration,
                carcass.carcass_id,
                carcass.carcass_x,
                carcass.carcass_y,
                carcass.creation_time,
                carcass.is_infected,
                carcass.lifetime,
                carcass.age_class
            )?;
        }
    }

    println!("Carcasses saved to: {}", filename);
    Ok(())
}
*/

pub fn save_carcasses_as_csv(filename: &str, carcass_states: &[(usize, Vec<Carcass>)]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "iteration,carcass_id,carcass_x,carcass_y,creation_time,is_infected,lifetime,age_class")?;

    // Collect all the lines in parallel
    let csv_lines: Vec<String> = carcass_states
        .par_iter()
        .flat_map(|(iteration, carcasses)| {
            carcasses.par_iter().map(move |carcass| {
                format!(
                    "{},{},{},{},{},{},{},{}",
                    iteration,
                    carcass.carcass_id,
                    carcass.carcass_x,
                    carcass.carcass_y,
                    carcass.creation_time,
                    carcass.is_infected,
                    carcass.lifetime,
                    carcass.age_class
                )
            })
        })
        .collect();

    // Write all the collected lines sequentially
    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("Carcasses saved to: {}", filename);
    Ok(())
}

//save highseats as csv

/* 
pub fn save_high_seats_as_csv(filename: &str, high_seat_states: &[(usize, Vec<HighSeat>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,hs_id,x_hs,y_hs,is_occupied,range")?;

    // Write each high seat's data for each iteration
    for (iteration, high_seats) in high_seat_states {
        for high_seat in high_seats {
            writeln!(
                file,
                "{},{},{},{},{},{}",
                iteration,
                high_seat.hs_id,
                high_seat.x_hs,
                high_seat.y_hs,
                high_seat.is_occupied,
                high_seat.range
            )?;
        }
    }

    println!("High seats saved to: {}", filename);
    Ok(())
}
*/

pub fn save_high_seats_as_csv(filename: &str, high_seat_states: &[(usize, Vec<HighSeat>)]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "iteration,hs_id,x_hs,y_hs,is_occupied,range")?;

    // Collect all the lines in parallel
    let csv_lines: Vec<String> = high_seat_states
        .par_iter()
        .flat_map(|(iteration, high_seats)| {
            high_seats.par_iter().map(move |high_seat| {
                format!(
                    "{},{},{},{},{},{}",
                    iteration,
                    high_seat.hs_id,
                    high_seat.x_hs,
                    high_seat.y_hs,
                    high_seat.is_occupied,
                    high_seat.range
                )
            })
        })
        .collect();

    // Write all the collected lines sequentially
    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("High seats saved to: {}", filename);
    Ok(())
}

/*
pub fn save_hunting_statistics_as_csv(filename: &str, hunting_statistics: &[(usize, HuntingStatistics)], gv:   &[GlobalVariables]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

               
    writeln!(file, "x,y,sx,age,age_class,id,origin_group,type_individual,iteration,month,year")?;

    // Write each hunted individual's data for each iteration
    for (iteration, stats) in hunting_statistics {
        let month = gv.iter().nth(*iteration).map_or("NA".to_string(), |v| v.month.to_string());
        let year = gv.iter().nth(*iteration).map_or("NA".to_string(), |v| v.year.to_string());
        for hunted_individual in &stats.hunted_individuals {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{}",
                hunted_individual.x,
                hunted_individual.y,
                hunted_individual.sx,
                hunted_individual.age,
                hunted_individual.age_class,
                hunted_individual.id,
                hunted_individual.origin_group.unwrap_or(0),
                hunted_individual.type_individual,
                iteration,
                month,
                year,
            )?;
        }
    }

    println!("Hunting statistics saved to: {}", filename);
    Ok(())
}
*/

pub fn save_hunting_statistics_as_csv(filename: &str, hunting_statistics: &[(usize, HuntingStatistics)], gv: &[GlobalVariables]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "x,y,sx,age,age_class,id,origin_group,type_individual,iteration,month,year")?;

    // Collect all the lines in parallel
    let csv_lines: Vec<String> = hunting_statistics
        .par_iter()
        .flat_map(|(iteration, stats)| {
            // Retrieve the month and year for the current iteration
            let month = gv.get(*iteration).map_or("NA".to_string(), |v| v.month.to_string());
            let year = gv.get(*iteration).map_or("NA".to_string(), |v| v.year.to_string());

            // Process each hunted individual in parallel
            stats.hunted_individuals.par_iter().map(move |hunted_individual| {
                format!(
                    "{},{},{},{},{},{},{},{},{},{},{}",
                    hunted_individual.x,
                    hunted_individual.y,
                    hunted_individual.sx,
                    hunted_individual.age,
                    hunted_individual.age_class,
                    hunted_individual.id,
                    hunted_individual.origin_group.unwrap_or(0),
                    hunted_individual.type_individual,
                    iteration,
                    month,
                    year,
                )
            })
        })
        .collect();

    // Write all the collected lines sequentially
    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("Hunting statistics saved to: {}", filename);
    Ok(())
}

/*
pub fn save_sim_meta_data_row_output_as_csv(filename: &str, mdata: &Vec<(usize, SimMetaData)> ) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,current_time,day,month,year,good_year,n_individuals,n_roamers,n_dispersers,n_infected,n_incubating,n_symptomatic,n_highly_infectious,n_recovered,n_dead,n_susceptible")?;

    // Iterate through the metadata and write each row
    for (iteration, sim_meta_data) in mdata.iter() {
        for row in &sim_meta_data.iteration_output {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                iteration,
                row[0],
                row[1],
                row[2],
                row[3],
                row[4],
                row[5],
                row[6],
                row[7],
                row[8],
                row[9],
                row[10],
                row[11],
                row[12],
                row[13],
                row[14],
            )?;
        }
    }
    println!("Row output saved to: {}", filename);
    Ok(())
}
*/

pub fn save_sim_meta_data_row_output_as_csv(filename: &str, mdata: &Vec<(usize, SimMetaData)>) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Write the header line
    writeln!(writer, "iteration,current_time,day,month,year,good_year,n_individuals,n_roamers,n_dispersers,n_infected,n_incubating,n_symptomatic,n_highly_infectious,n_recovered,n_dead,n_susceptible")?;

    // Collect all the lines in parallel
    let csv_lines: Vec<String> = mdata
        .par_iter()
        .flat_map(|(iteration, sim_meta_data)| {
            sim_meta_data.iteration_output.par_iter().map(move |row| {
                format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    iteration,
                    row[0],
                    row[1],
                    row[2],
                    row[3],
                    row[4],
                    row[5],
                    row[6],
                    row[7],
                    row[8],
                    row[9],
                    row[10],
                    row[11],
                    row[12],
                    row[13],
                    row[14],
                )
            })
        })
        .collect();

    // Write all the collected lines sequentially
    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("Row output saved to: {}", filename);
    Ok(())
}

/* 
pub fn save_interaction_layer_as_bson(
    file_path: &str,
    interaction_layers: &[(usize, InteractionLayer)],
) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    // Convert the interaction layers to a serializable format
    let mut serializable_layers: Vec<Document> = Vec::new();
    for (iteration, layer) in interaction_layers {
        let mut layer_doc = Document::new();
        for (&(x, y, t), cell) in layer {
            let key = format!("{},{},{}", x, y, t);
            layer_doc.insert(key, to_bson(cell).unwrap());
        }
        let doc = doc! {
            "iteration": *iteration as i64,
            "layer": layer_doc
        };
        serializable_layers.push(doc);
    }

    // Serialize the data to BSON
    let top_level_doc = doc! { "interaction_layers": serializable_layers };
    let bson_data = to_bson(&top_level_doc)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    file.write_all(&bson::to_vec(&bson_data).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)?;

    Ok(())
}

*/
//pub fn save_individuals_as_csv(filename: &str, group_states: &[(usize, Vec<Groups>)]) -> io::Result<()> {
//
//    // Create or open the CSV file
//    let mut file = File::create(filename)?;
//
//    // Write the header line
//    writeln!(file, "iteration,id,group_id,x,y,sex,age,age_class,known_cells,group_member_ids")?;
//
//    // Write each individual's data for each iteration
//    for (iteration, groups) in group_states {
//        for group in groups {
//        // Convert variables to strings for CSV output
//            let known_cells_str: String = individual
//                .memory
//                .known_cells_order
//                .iter()
//                .map(|&(x, y)| format!("[{}_{}]", x, y))
//                .collect::<Vec<String>>()
//                .join(";");
//            
//                let group_member_ids_str: String = format!(
//                    "[{}]",
//                    individual
//                        .memory
//                        .group_member_ids
//                        .iter()
//                        .map(|&id| id.to_string())
//                        .collect::<Vec<String>>()
//                        .join(";")
//                );
//            
//            //let last_three_cells_str: String = individual
//            //    .memory
//            //    .last_visited_cells_order
//            //    .iter()
//            //    .map(|&(x, y)| format!("[{}_{}]", x, y))
//            //    .collect::<Vec<String>>()
//            //    .join(";");
//
//            let age_class_str: String = format!("{}", individual.age_class);
//            let sex_str: String = format!("{}", individual.sex);
//             
//            writeln!(
//                file,
//                "{},{},{},{},{},{},{},{},{},{}",
//                iteration,
//                individual.id,
//                individual.group_id,
//                individual.x,
//                individual.y,
//                sex_str,
//                individual.age,
//                age_class_str,
//                known_cells_str,
//                group_member_ids_str,
//                //last_three_cells_str
//            )?;
//    }
//}
//
//
//    println!("Individuals saved to: {}", filename);
//    Ok(())
//}




pub fn save_groups_as_csv_parallel(filename: &str, group_states: &[(usize, Vec<Groups>)]) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "iteration,individual_id,group_id,x,y,sex,age,age_class,known_cells,target_cell,core_cell,movement_type,remaining_stay_time,origin_group,ap_list,infection_stage,health_status")?;

    let csv_lines: Vec<String> = group_states
        .par_iter()
        .flat_map(|(iteration, groups)| {
            groups.par_iter().flat_map(move |group| {
                group.group_members.par_iter().map(move |group_members| {
                    let known_cells_str = group.memory.known_cells_order
                        .iter()
                        .map(|&(x, y)| format!("[{}_{}]", x, y))
                        .collect::<Vec<String>>()
                        .join(";");

                    let ap_list_str = group.current_ap
                        .iter()
                        .map(|&(x, y)| format!("[{}_{}]", x, y))
                        .collect::<Vec<String>>()
                        .join(";");

                    let target_cell_str = group.target_cell
                        .iter()
                        .map(|&(x, y)| format!("[{}_{}]", x, y))
                        .collect::<Vec<String>>()
                        .join(";");

                    let core_cell_str = group.core_cell
                        .iter()
                        .map(|&(x, y)| format!("[{}_{}]", x, y))
                        .collect::<Vec<String>>()
                        .join(";");

                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        iteration,
                        group_members.individual_id,
                        group.group_id,
                        group.x,
                        group.y,
                        group_members.sex,
                        group_members.age,
                        group_members.age_class,
                        known_cells_str,
                        target_cell_str,
                        core_cell_str,
                        group.movement,
                        group.remaining_stay_time,
                        group_members.origin_group_id,
                        ap_list_str,
                        group_members.infection_stage,
                        group_members.health_status,
                    )
                })
            })
        })
        .collect();

    for line in csv_lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    println!("Groups saved to: {}", filename);
    Ok(())
}