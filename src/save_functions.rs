// Saving Output functions

use crate::*;

pub fn save_individuals_as_csv(filename: &str, individuals_states: &[(usize, Vec<Individual>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,id,group_id,x,y,sex,age,age_class,known_cells,group_member_ids, last_three_cells")?;

    // Write each individual's data for each iteration
    for (iteration, individuals) in individuals_states {
        for individual in individuals {
        // Convert variables to strings for CSV output
            let known_cells_str: String = individual
                .memory
                .known_cells
                .iter()
                .map(|&(x, y)| format!("[{}_{}]", x, y))
                .collect::<Vec<String>>()
                .join(";");
            
                let group_member_ids_str: String = format!(
                    "[{}]",
                    individual
                        .memory
                        .group_member_ids
                        .iter()
                        .map(|&id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(";")
                );
            
            let last_three_cells_str: String = individual
                .memory
                .last_visited_cells_order
                .iter()
                .map(|&(x, y)| format!("[{}_{}]", x, y))
                .collect::<Vec<String>>()
                .join(";");

            let age_class_str: String = format!("{}", individual.age_class);
            let sex_str: String = format!("{}", individual.sex);
             
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{}",
                iteration,
                individual.id,
                individual.group_id,
                individual.x,
                individual.y,
                sex_str,
                individual.age,
                age_class_str,
                known_cells_str,
                group_member_ids_str,
                last_three_cells_str
            )?;
    }
}


    println!("Individuals saved to: {}", filename);
    Ok(())
}

pub fn save_grid_as_csv(filename: &str, grid_states: &[(usize, Vec<Vec<Cell>>)]) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "iteration,x,y,quality,counter,x_grid_corrected,y_grid_corrected")?;

    // Write each cell's data for each iteration
    for (iteration, grid) in grid_states {
        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                writeln!(file, "{},{},{},{},{},{},{}", iteration, x, y, cell.quality, cell.counter, cell.x_grid, cell.y_grid)?;
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


