// Saving Output functions

use crate::*;
use bson::{to_bson, Bson, doc,Document};
use std::fs::File;
use std::io::Write;


// Fix me to work with groups

pub fn save_groups_as_csv(filename: &str, group_states: &[(usize, Vec<Groups>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,individual_id,group_id,x,y,sex,age,age_class,known_cells,target_cell,core_cell,movement_type,remaining_stay_time,origin_group,ap_list")?;//,group_member_ids")?;

    // Write each individual's data for each iteration
    for (iteration, groups) in group_states {
        for group in groups {
            for group_members in &group.group_members {
                // Convert variables to strings for CSV output
                let known_cells_str: String = group
                    .memory
                    .known_cells_order
                    .iter()
                    .map(|&(x, y)| format!("[{}_{}]", x, y))
                    .collect::<Vec<String>>()
                    .join(";");

                let ap_list_str: String = group
                    .current_ap
                    .iter()
                    .map(|&(x, y)| format!("[{}_{}]", x, y))
                    .collect::<Vec<String>>()
                    .join(";");

                //let group_member_ids_str: String = format!(
                //    "[{}]",
                //    group_members
                //        .memory
                //        .group_member_ids
                //        .iter()
                //        .map(|&id| id.to_string())
                //        .collect::<Vec<String>>()
                //        .join(";")
                //);

                //let last_three_cells_str: String = individual
                //    .memory
                //    .last_visited_cells_order
                //    .iter()
                //    .map(|&(x, y)| format!("[{}_{}]", x, y))
                //    .collect::<Vec<String>>()
                //    .join(";");

                let target_cell_str: String = group
                    .target_cell
                    .iter()
                    .map(|&(x, y)| format!("[{}_{}]", x, y))
                    .collect::<Vec<String>>()
                    .join(";");

                let core_cell_str: String = group
                    .core_cell
                    .iter()
                    .map(|&(x, y)| format!("[{}_{}]", x, y))
                    .collect::<Vec<String>>()
                    .join(";");

                let age_class_str: String = format!("{}", group_members.age_class);
                let sex_str: String = format!("{}", group_members.sex);
                //let target_cell_str: String = format!("[{:?}]", group.target_cell);
                //let core_cell_str: String = format!("[{:?}]", group.core_cell);
                //let remaining_stay_stime_str: String = format!("{}", group.remaining_stay_time);
                
                writeln!(
                    file,
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    iteration,
                    group_members.individual_id,
                    group.group_id,
                    group.x,
                    group.y,
                    sex_str,
                    group_members.age,
                    age_class_str,
                    known_cells_str,
                    target_cell_str,
                    core_cell_str,
                    group.movement,
                    group.remaining_stay_time,
                    group_members.origin_group_id,
                    ap_list_str,
                    //remaining_stay_stime_str,
                    //group_member_ids_str,
                    //last_three_cells_str
                )?;
            }
        }
    }

    println!("Groups saved to: {}", filename);
    Ok(())
}



pub fn save_grid_as_csv(filename: &str, grid_states: &[(usize, Vec<Vec<Cell>>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,x,y,quality,counter,x_grid_corrected,y_grid_corrected,is_ap,is_territory,territory_of_group")?;

    // Write each cell's data for each iteration
    for (iteration, grid) in grid_states {
        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                writeln!(file, "{},{},{},{},{},{},{},{},{},{}", iteration, x, y, cell.quality, cell.counter, cell.x_grid, cell.y_grid, cell.territory.is_ap, cell.territory.is_taken, cell.territory.taken_by_group)?;
            }
        }
    }

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



pub fn save_disperser_group_as_csv(filename: &str, disperser_states: &[(usize, Vec<DispersingFemaleGroup>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,individual_id,disperser_id,age,age_class,sex,health_status,origin_group_id,disperser_group_x,disperser_group_y,disperser_group_id")?;

    // Write each disperser's data for each iteration
    for (iteration, disperser_groups) in disperser_states {
        for disperser_group in disperser_groups {
            for disperser_group_member in disperser_group.dispersing_individuals.iter() {
                writeln!(
                    file,
                    "{},{},{},{},{},{},{},{},{},{},{}",
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
                    disperser_group.disp_grp_id                    
          
            )?;
        }
    }
}

    println!("Disperser states saved to: {}", filename);
    Ok(())
}


pub fn save_roamers_as_csv(filename: &str, roamer_states: &[(usize, Vec<RoamingIndividual>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    writeln!(file, "iteration,roamer_id,individual_id,age,age_class,sex,health_status,origin_group_id,roamer_x,roamer_y,known_groups,target_group,daily_distance,target_group_id,reached_target,stay_time,staying_with_target_group,target_cell,initial_dispersal")?;

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

            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
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
                roamer.initial_dispersal
                
            )?;
        }
    }

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

//save highseats as csv

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