// Saving Output functions

use crate::*;


// Fix me to work with groups

pub fn save_groups_as_csv(filename: &str, group_states: &[(usize, Vec<Groups>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,individual_id,group_id,x,y,sex,age,age_class,known_cells,target_cell,core_cell,movement_type,remaining_stay_time,origin_group")?;//,group_member_ids")?;

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
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
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
    writeln!(file, "iteration,day,month,year,n_individuals,age_mortality, random_mortality")?;

    // Write each iteration's global variables
    for (iteration, globals) in global_variables.iter().enumerate() {
        writeln!(file, "{},{},{},{},{},{},{}", iteration + 1, globals.day, globals.month, globals.year, globals.n_individuals, globals.age_mortality, globals.random_mortality)?;
        // Add more variables as needed
    }

    println!("Global variables saved to: {}", filename);
    Ok(())
}



pub fn save_disperser_as_csv(filename: &str, disperser_states: &[(usize, Vec<DispersingIndividual>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,individual_id,disperser_id,x,y,age,age_class,sex,health_status,target_x,target_y,origin_group_id")?;

    // Write each disperser's data for each iteration
    for (iteration, dispersers) in disperser_states {
        for disperser in dispersers {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{},{}",
                iteration,
                disperser.individual_id,
                disperser.disperser_id,
                disperser.x,
                disperser.y,
                disperser.age,
                disperser.age_class,
                disperser.sex,
                disperser.health_status,
                disperser.target_cell.unwrap().0,
                disperser.target_cell.unwrap().1,
                disperser.origin_group_id,
            )?;
        }
    }

    println!("Disperser states saved to: {}", filename);
    Ok(())
}
















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