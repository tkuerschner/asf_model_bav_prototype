use crate::*;

// Define a enum to represent the movement mode 
#[derive(Debug, Clone, PartialEq)]
pub enum MovementMode{
    ApTransition,
    Foraging,
    NotSet,
}

// Implement the Display trait for MovementMode
impl fmt::Display for MovementMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MovementMode::ApTransition => write!(f, "apTransition"),
            MovementMode::Foraging => write!(f, "foraging"),
            MovementMode::NotSet => write!(f, "not set"),
        }
    }
}

pub fn move_groups<R: Rng>(grid: &Vec<Vec<Cell>>, group: &mut Vec<Groups>, rng: &mut R, mut i_layer: &mut InteractionLayer, time: usize) {
    for group in group.iter_mut() {

        //println!("Movement called"); //<------ DEBUG print

        let realign_time = 3; //number of steps before realigning towards the target

        while group.daily_movement_distance > 0  {

            

            //check if a target cell is needed and assign a stay time for the ap
            if group.target_cell.is_none() {
                let territory_ap = get_attraction_points_in_territory(grid, group.group_id);
                let new_target_cell = territory_ap
                    .choose(rng)
                    .cloned()
                    .expect("No attraction points in territory");
            
                group.set_target_cell(new_target_cell);
                group.remaining_stay_time = rng.gen_range(MIN_STAY_TIME..MAX_STAY_TIME);
            }

            // Steps
            // 25% chance to move randomly
            if rng.gen_range(0..100) < 25 { // <-----------------------------------------------DEBUG FIX ME percentage
                //move_to_random_adjacent_cells(grid.len(), individual, rng);
                move_to_random_adjacent_cells_2(grid, group, rng);
                //record_movement_in_interaction_layer(  &mut i_layer,  group.x, group.y, time, group.group_id, "group",  0);
                i_layer.add_entity_and_record_movement(
                    group.group_id, 
                    "group", 
                    time, 
                    0, 
                    0, 
                    group.group_members.last().unwrap().individual_id, // id of the first individual in the group
                    1.0, 
                    group.x as f64, 
                    group.y as f64,
                    group.infected_member_present(),


                );
                group.daily_movement_distance -= 1;
            } else {
                // Move towards the cell with the highest quality
               // move_towards_highest_quality(grid, group, rng);
               //move_within_territory(grid, group, rng);

               // move_to_random_adjacent_cells_2(grid, group, rng);
                if group.movement == MovementMode::ApTransition {

                    if group.x == group.target_cell.unwrap().0 && group.y == group.target_cell.unwrap().1 {
                        group.movement = MovementMode::Foraging;

                        break; // if target location reached flit to foraging
                    }
                    
                   // if realign_time > 0 { // every 3rd step we realign to the target
                   // correlated_random_walk_towards_target(grid, group, rng);
                   // realign_time -= 1;
                   // }
                   // if realign_time == 0 {
                   //     move_to_closest_adjacent_cell_to_target(grid, group);
                   //     realign_time = 3;
                   // }

                  // move_to_closest_adjacent_cell_to_target(grid, group);

                  //move_towards_target_cell(group);

                  //move_one_step_towards_target_cell(group);
                  move_one_step_towards_target_cell_with_random(group,rng,grid);
                  //record_movement_in_interaction_layer(  &mut i_layer,  group.x, group.y, time, group.group_id, "group",  0);
                  i_layer.add_entity_and_record_movement(
                    group.group_id, 
                    "group", 
                    time, 
                    0, 
                    0, 
                    group.group_members.last().unwrap().individual_id, // id of the first individual in the group
                    1.0, 
                    group.x as f64, 
                    group.y as f64,
                    group.infected_member_present(),
                );

                    group.daily_movement_distance -= 1;

                    if group.distance_to_target() <= 3 {

                        group.movement = MovementMode::Foraging;
                        //print!("Engage forage mode"); // DEBUG

                    }

                } else if group.movement == MovementMode::Foraging {
                    
                    if group.remaining_stay_time <= 0 { //if stay time around ap is used up get a new ap to move towards
                        
                        let new_target_cell;
                        if rng.gen_range(1..100) > 100 { // 1% chance to choose a new ap outside the territory // DEBUG TEMPORARILY DEACTIVATED
                           
                           let outside_ap = get_closest_attraction_points_outside_territory(grid, group);

                            new_target_cell = outside_ap
                            .choose(rng)
                            .cloned()
                            .expect("No other attraction points found");
                        } else {
                        let territory_ap = get_attraction_points_in_territory(grid, group.group_id);
                        let closest_ap = get_closest_attraction_point(group, &territory_ap);
                        let other_aps: Vec<(usize, usize)> = territory_ap
                            .into_iter()
                            .filter(|&ap| ap != closest_ap)
                            .collect();

                        // Choose a random target cell from the remaining attraction points
                             new_target_cell = other_aps
                            .choose(rng)
                            .cloned()
                            .expect("No other attraction points in territory");
                        }
                        group.set_target_cell(new_target_cell);
                        group.remaining_stay_time = rng.gen_range(MIN_STAY_TIME..MAX_STAY_TIME);

                        group.movement = MovementMode::ApTransition;
                       // group.target_cell = None;
                        
                        break;
                    }

                    
                    // if distance to current ap is more than 3 cells individuals more back towards the ap
                    if ((group.x as isize) - (group.target_cell.unwrap().0 as isize)).abs() <= 3
                    && ((group.y as isize) - (group.target_cell.unwrap().1 as isize)).abs() <= 3
                    {
                        move_towards_highest_quality(grid, group, rng);
                        //record_movement_in_interaction_layer(  &mut i_layer,  group.x, group.y, time, group.group_id, "group",  0);
                        i_layer.add_entity_and_record_movement(
                            group.group_id, 
                            "group", 
                            time, 
                            0, 
                            0, 
                            group.group_members.last().unwrap().individual_id, // id of the first individual in the group
                            
                            1.0, 
                            group.x as f64, 
                            group.y as f64,
                            group.infected_member_present(),
                        );
                        group.daily_movement_distance -= 1;
                    }else {
                        
                       // correlated_random_walk_towards_target(grid, group, rng);
                        //move_one_step_towards_target_cell(group);
                        move_one_step_towards_target_cell_with_random(group,rng,grid);
                        //record_movement_in_interaction_layer(  &mut i_layer,  group.x, group.y, time, group.group_id, "group",  0);
                        i_layer.add_entity_and_record_movement(
                            group.group_id, 
                            "group", 
                            time, 
                            0, 
                            0, 
                            group.group_members.last().unwrap().individual_id, // id of the first individual in the group
                            1.0, 
                            group.x as f64, 
                            group.y as f64,
                            group.infected_member_present(),
                        );
                        group.daily_movement_distance -= 1;
                    }
                    
                    
                   // println!("Movement left: {}", group.daily_movement_distance); // DEBUG PRINT

                      // Update presence timer
                    //group.memory.presence_timer += 1;
                    //
                    //// Check if presence time limit is reached or 5% chance to move
                    //if group.memory.presence_timer >= PRESENCE_TIME_LIMIT || rng.gen_range(0..100) < MOVE_CHANCE_PERCENTAGE {
                    //    // Reset presence timer and force movement to a random cell
                    //    group.memory.presence_timer = 0;
                    //    //move_to_random_adjacent_cells(grid.len(), group, rng);
                    //
                    //    group.movement = MovementMode::ApTransition;
                    //}
                }
            }
        }
        // Reset movement distance
        group.daily_movement_distance =  DEFAULT_DAILY_MOVEMENT_DISTANCE;

        // update the stay time around the ap
        group.update_remaining_stay_time();
    }

}


pub fn move_to_closest_adjacent_cell_to_target(grid: &Vec<Vec<Cell>>, group: &mut Groups) {
    // Find the closest adjacent cell to the target
    if let Some((new_x, new_y)) = find_closest_adjacent_cell_to_target(group) {
        // Update known cells
        update_memory(&mut group.memory.known_cells, &mut group.memory.known_cells_order, (group.x, group.y), MAX_KNOWN_CELLS);

        // Update individual's position
        group.x = new_x;
        group.y = new_y;
    }
}

pub fn move_to_new_ap(grid: &Vec<Vec<Cell>>, group: &mut Groups, rng: &mut impl Rng) { // UNSUSED
    // If remaining_stay_time is 0 or there is no target_cell, select a new target_cell from attraction points
    if group.remaining_stay_time == 0 || group.target_cell.is_none() {
        let territory_ap = get_attraction_points_in_territory(grid, group.group_id);
        let new_target_cell = territory_ap
            .choose(rng)
            .cloned()
            .expect("No attraction points in territory");

        group.set_target_cell(new_target_cell);
        group.remaining_stay_time = rng.gen_range(MIN_STAY_TIME..MAX_STAY_TIME);
    }

    // Move towards the target_cell using move_towards_highest_quality
    //move_towards_highest_quality(grid, group, rng);

    // Use CRW to move to target
    correlated_random_walk_towards_target(grid, group, rng);


    // Decrement remaining_stay_time
    group.update_remaining_stay_time();
}


pub fn move_towards_highest_quality(grid: &Vec<Vec<Cell>>, group: &mut Groups, rng: &mut impl Rng) {
    // Generate a list of adjacent cells
    let adjacent_cells = vec![
        (group.x.saturating_sub(1), group.y),
        (group.x.saturating_add(1), group.y),
        (group.x, group.y.saturating_sub(1)),
        (group.x, group.y.saturating_add(1)),
    ];

    // Calculate the quality score for each adjacent cell and find the cell with the highest quality
    let (new_x, new_y) = adjacent_cells.iter()
    .filter(|&&(x, y)| x < grid.len() && y < grid[0].len())  // Check bounds
    .map(|&(x, y)| (x, y, calculate_quality_score(grid, x, y)))
    .max_by(|&(_, _, quality1), &(_, _, quality2)| quality1.partial_cmp(&quality2).unwrap_or(std::cmp::Ordering::Equal))
    .map(|(x, y, _)| (x, y))
    .unwrap_or_else(|| random_cell(grid.len(), rng));

   

    // Update known cells and last visited cells
    //update_memory(&mut individual.memory.known_cells, &mut individual.memory.known_cells_order, (individual.x, individual.y), MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);

    update_memory(&mut group.memory.known_cells, &mut group.memory.known_cells_order, (group.x, group.y), MAX_KNOWN_CELLS);
    //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);


    // Update individual's position
    group.x = new_x;
    group.y = new_y;
}

pub fn move_one_step_towards_target_cell(group: &mut Groups) {
    // Check if there is a target cell set
    if let Some(target_cell) = group.target_cell {
        // Calculate the movement direction towards the target cell
        let direction = (
            target_cell.0 as isize - group.x as isize,
            target_cell.1 as isize - group.y as isize,
        );

        // Update individual's position by one step
        group.x = (group.x as isize + direction.0.signum()).max(0) as usize;
        group.y = (group.y as isize + direction.1.signum()).max(0) as usize;

        // Update known cells
        update_memory(
            &mut group.memory.known_cells,
            &mut group.memory.known_cells_order,
            (group.x, group.y),
            MAX_KNOWN_CELLS,
        );
    }
}

pub fn move_one_step_towards_target_cell_with_random(group: &mut Groups, rng: &mut impl Rng, grid: &Vec<Vec<Cell>>) {
    // Check if there is a target cell set
    if let Some(target_cell) = group.target_cell {
        // Randomly decide whether to move towards the target or move randomly
        if rng.gen_range(0..100) < 90 {
            // Calculate the movement direction towards the target cell
            let direction = (
                target_cell.0 as isize - group.x as isize,
                target_cell.1 as isize - group.y as isize,
            );

            // Update individual's position by one step
            group.x = (group.x as isize + direction.0.signum()).max(0) as usize;
            group.y = (group.y as isize + direction.1.signum()).max(0) as usize;

            // Update known cells
            update_memory(
                &mut group.memory.known_cells,
                &mut group.memory.known_cells_order,
                (group.x, group.y),
                MAX_KNOWN_CELLS,
            );
        } else {
            // Move randomly
            move_to_random_adjacent_cells_2(grid, group, rng);
        }
    }
}

// Function for correlated random walk towards the target // NOT WORKING
fn correlated_random_walk_towards_target(grid: &Vec<Vec<Cell>>, group: &mut Groups, rng: &mut impl Rng) {
    // Autoregressive parameters 
    let alpha = 0.5; // Persistence parameter

    // Placeholder for storing the last movement direction
    let mut last_direction = (0, 0);

    // Generate a list of adjacent cells
    let adjacent_cells = vec![
        (group.x.saturating_sub(1), group.y),
        (group.x.saturating_add(1), group.y),
        (group.x, group.y.saturating_sub(1)),
        (group.x, group.y.saturating_add(1)),
    ];

    // Calculate the quality score for each adjacent cell
    let quality_scores: Vec<_> = adjacent_cells.iter()
        .filter(|&&(x, y)| x < grid.len() && y < grid[0].len())
        .map(|&(x, y)| (x, y, calculate_quality_score(grid, x, y)))
        .collect();

    // Sort cells by quality in descending order
    let sorted_cells: Vec<_> = quality_scores.iter()
        .cloned()
        .filter(|&(x, y, _)| x < grid.len() && y < grid[0].len())
        .collect();
    let sorted_cells: Vec<_> = sorted_cells.iter()
        .cloned()
        .filter(|&(x, y, _)| x < grid.len() && y < grid[0].len())
        .collect();
    let sorted_cells: Vec<_> = sorted_cells.iter()
        .cloned()
        .filter(|&(x, y, _)| x < grid.len() && y < grid[0].len())
        .collect();

    // Select the first cell with quality > 0
    let target_cell = sorted_cells
        .first()
        .map(|&(x, y, _)| (x, y))
        .unwrap_or_else(|| random_cell(grid.len(), rng));

    // Update known cells
    update_memory(&mut group.memory.known_cells, &mut group.memory.known_cells_order, (group.x, group.y), MAX_KNOWN_CELLS);

    // Calculate the movement direction towards the target cell
    let direction = (target_cell.0.saturating_sub(group.x), target_cell.1.saturating_sub(group.y));

    // Update the movement direction using autoregressive model
    let correlated_direction = (
        (alpha * direction.0 as f64 + (1.0 - alpha) * last_direction.0 as f64).round() as isize,
        (alpha * direction.1 as f64 + (1.0 - alpha) * last_direction.1 as f64).round() as isize,
    );

    // Update individual's position
    group.x = (group.x as isize + correlated_direction.0).max(0) as usize;
    group.y = (group.y as isize + correlated_direction.1).max(0) as usize;

    // Update last_direction for the next iteration
    last_direction = correlated_direction;
}

pub fn move_to_random_adjacent_cells_2(grid: &Vec<Vec<Cell>>, group: &mut Groups, rng: &mut impl Rng){
    // Get the current position of the individual
    let current_x = group.x;
    let current_y = group.y;

      // Generate a list of adjacent cells
    let mut adjacent_cells = vec![
      (current_x.saturating_sub(1), current_y),
      (current_x.saturating_add(1), current_y),
      (current_x, current_y.saturating_sub(1)),
      (current_x, current_y.saturating_add(1)),
    ];

        // Shuffle the list of adjacent cells
        adjacent_cells.shuffle(rng);


          // Select the first cell (randomized) with quality > 0
     let target_cell = adjacent_cells
     .into_iter()
     .filter(|&(x, y)| x < grid.len() && y < grid[0].len() && grid[x][y].quality > 0.0)
     .next()
     .unwrap_or_else(|| {
         // If no valid adjacent cells with quality > 0, move randomly within the grid
         // TEMP FIX ME 
         //andom_cell_with_quality(grid, rng)
         reset_group_coordinates_to_core_cell(group) // TEMP FIX ME
     });

     update_memory(&mut group.memory.known_cells, &mut group.memory.known_cells_order, (group.x, group.y), MAX_KNOWN_CELLS);
     //update_memory(&mut individual.memory.last_visited_cells, &mut individual.memory.last_visited_cells_order, (individual.x, individual.y), MAX_LAST_VISITED_CELLS);
 
     // Update individual's position
     group.x = target_cell.0;
     group.y = target_cell.1;
 }