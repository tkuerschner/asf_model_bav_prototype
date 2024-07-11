use crate::*;


pub fn group_setup(cell_info_list: &Vec<CellInfo>,  grid: &mut Vec<Vec<Cell>>, num_groups: usize) -> Vec<Groups> { 

    // Create individuals with unique IDs, group IDs, and memory
    let mut group: Vec<Groups> = Vec::with_capacity(num_groups);
    //let grid_size = grid.len();  

    for group_id in 0..num_groups {

        // Select an free attraction point as territory coe cell
        let free_ap = get_free_attraction_points(&grid);
        if free_ap.is_empty(){ // if no more free aps are available stop group creation
        println!("No more free space for additional groups, group creation halted at {}/{} groups!", group_id,num_groups);
        break;
        }else{
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..free_ap.len());
        let random_ap = free_ap[random_index]; // select a random free ap
        
        let x = random_ap.0;
        let y = random_ap.1;

        let group_id = generate_group_id(); // add incrementing group id

        occupy_this_cell(&mut grid[x][y], group_id); // occupy the selected ap

        let core_cell = (x, y); // set the core cell
        make_core_cell(&mut grid[x][y], group_id);

        //let desired_total_cells = 1600;     
 
        //circular_bfs(grid, x, y, group_id, desired_total_cells); // fill the territory with cells
        
        //let presence_timer = 0; 

        //let memory = GroupMemory { // create memory for the group
        //    known_cells: HashSet::new(),
        //    group_member_ids: Vec::new(),
        //    known_cells_order: Vec::new(),
        //    presence_timer,
        //};

        //let target_cell = None; 
        //let remaining_stay_time = 0;
        //let movement = MovementMode::Foraging;
        //let group_members = vec![];
        //let daily_movement_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE; //<--------------------DEBUG FIX ME with actual values
        //let max_size = 10000; 

        let mut new_group = Groups {
            group_id,
            x,
            y,
            core_cell: Some(core_cell),
            target_cell: None,
            remaining_stay_time: 0,
            memory: GroupMemory {
                known_cells: HashSet::new(),
                group_member_ids: Vec::new(),
                known_cells_order: Vec::new(),
                presence_timer: 0,
            },
            movement: MovementMode::Foraging,
            group_members: vec![],
            daily_movement_distance: DEFAULT_DAILY_MOVEMENT_DISTANCE, //<--------------------DEBUG FIX ME with actual values
            max_size: 10000,
            current_ap: Vec::new(),
            active: true,
        };
        //new_group.expand_territory_within_range(grid);
        //new_group.expand_territory_with_natural_shape_and_radius(grid);
        new_group.expand_territory_with_natural_shape(grid);
        new_group.current_ap = get_attraction_points_in_territory(grid, new_group.group_id);
        //new_group.expand_territory(grid); // Fill the territory with cells
        //new_group.claim_territory(grid); // Claim the territory

        group.push(new_group);


       // group.push(Groups { // create the group
       //     group_id,
       //     x,
       //     y,
       //     core_cell: Some(core_cell),
       //     target_cell,
       //     remaining_stay_time,
       //     memory,
       //     movement,
       //     group_members,
       //     daily_movement_distance,
       //     max_size,
       // });

     }
    }

    group // return the group
}

// function that fills the groups with individuals
pub fn fill_initial_groups(groups: &mut Vec<Groups>, grid: &Vec<Vec<Cell>>) {
    for group in groups.iter_mut() {
        let breed_cap = calculate_mean_quality_for_group(grid, group.group_id).round();

        // group size estimator from SwiCoIBMove 4.5
        let tmp_size = (4.5 * breed_cap - 1.0).round() as u32;
        //println!("grpsize {}", tmp_size); FIX ME  DEBUG PRINT
        group.max_size = tmp_size as usize;

        for _ in 0..tmp_size {
          let _ = group.create_new_initial_group_member();
        }
    }
}

// function to calculate the mean habitat quality for a groups territory
pub fn calculate_mean_quality_for_group(grid: &Vec<Vec<Cell>>, group_id: usize) -> f64 {
    let mut total_quality = 0.0;
    let mut num_cells = 0;

    for row in grid {
        for cell in row {
            if cell.territory.is_taken && cell.territory.taken_by_group == group_id {
                total_quality += cell.quality * 10.0; //FIX ME adjust for the raster
                num_cells += 1;
            }
        }
    }

    if num_cells == 0 {
        return 0.0; // Avoid division by zero
    }

    //println!("qual: {}, ncell: {}", total_quality, num_cells );

    total_quality / (num_cells as f64)
}

pub fn calculate_max_group_size_for_group(grid: &Vec<Vec<Cell>>, group_id: usize) -> usize {
    let breed_cap = calculate_mean_quality_for_group(grid, group_id).round();

    // group size estimator from SwiCoIBMove 4.5
    let tmp_size = (4.5 * breed_cap - 1.0).round() as u32;
    //println!("grpsize {}", tmp_size); FIX ME  DEBUG PRINT
    tmp_size as usize
}

pub fn count_group_members(group: &Groups) -> usize {
    group.group_members.len()
}

pub fn count_dispersers_in_disperser_group(disperser_group: &mut DispersingFemaleGroup) -> usize {
   disperser_group.dispersing_individuals.len()

}

// function to update the memory of a group
pub fn update_memory(memory: &mut HashSet<(usize, usize)>, order: &mut Vec<(usize, usize)>, new_cell: (usize, usize), max_size: usize) {
    memory.insert(new_cell);

    order.retain(|&cell| memory.contains(&cell)); // Remove cells that are not in the memory

    if order.len() >= max_size {
        let oldest_cell = order.remove(0); // Remove the oldest element
        memory.remove(&oldest_cell);
    }

    order.push(new_cell);
}


// function to add a new group at a specific set of coordinates
pub fn add_new_group_at_location(groups: &mut Vec<Groups>, grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
   //let group_id = generate_group_id(); // add incrementing group id
   //occupy_this_cell(&mut grid[x][y], group_id); // occupy the selected ap
   //let core_cell = (x, y); // set the core cell
   //make_core_cell(&mut grid[x][y], group_id);
   //let desired_total_cells = 1600; // FIX ME DEBUG

   //circular_bfs(grid, x, y, group_id, desired_total_cells); // fill the territory with cells
   //
   //let presence_timer = 0;
   //let memory = GroupMemory {
   //    known_cells: HashSet::new(),
   //    group_member_ids: Vec::new(),
   //    known_cells_order: Vec::new(),
   //    presence_timer,
   //};
   //let target_cell = None;
   //let remaining_stay_time = 0;
   //let movement = MovementMode::Foraging;
   //let group_members = vec![];
   //let daily_movement_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE; //<--------------------DEBUG FIX ME with actual values
   //let max_size = calculate_max_group_size_for_group(grid, group_id);

   //groups.push(Groups {
   //    group_id,
   //    x,
   //    y,
   //    core_cell: Some(core_cell),
   //    target_cell,
   //    remaining_stay_time,
   //    memory,
   //    movement,
   //    group_members,
   //    daily_movement_distance,
   //    max_size,
   //});
   let group_id = generate_group_id(); // add incrementing group id
   occupy_this_cell(&mut grid[x][y], group_id); // occupy the selected ap
   let core_cell = (x, y); // set the core cell

   // Expand territory around the core cell
   let mut new_group = Groups {
       group_id,
       x,
       y,
       core_cell: Some(core_cell),
       target_cell: None,
       remaining_stay_time: 0,
       memory: GroupMemory {
           known_cells: HashSet::new(),
           group_member_ids: Vec::new(),
           known_cells_order: Vec::new(),
           presence_timer: 0,
       },
       movement: MovementMode::Foraging,
       group_members: Vec::new(),
       daily_movement_distance: DEFAULT_DAILY_MOVEMENT_DISTANCE,
       max_size: calculate_max_group_size_for_group(grid, group_id),
       current_ap: Vec::new(),
       active: true,
   };

   new_group.expand_territory_with_natural_shape(grid); // fill the territory with cells
   
   groups.push(new_group);
    
}

// function to check if a group has no members 
pub fn group_has_no_members(group: &Groups) -> bool { // TESTER FUNCTION to be removed for the one below
    if group.group_members.is_empty() {
        //println!("Group {} has no members", group.group_id);
        return true;
    
    } else {
        return false;
    }

}
//pub fn group_has_no_members(group: &Groups) -> bool { 
//    group.group_members.is_empty()
//}


// take the groupid of the groups that have no members, find all cells taken by that group and free them
pub fn free_group_cells(groups: &mut Vec<Groups>, grid: &mut Vec<Vec<Cell>>) {
    let group_ids: Vec<usize> = groups.iter().filter(|group| group_has_no_members(group)).map(|group| group.group_id).collect();

    for group_id in group_ids {
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                if cell.territory.is_taken && cell.territory.taken_by_group == group_id {
                    free_this_cell(cell);
                }
            }
        }
    }

}


pub fn free_this_cell(cell: &mut Cell) {
    cell.territory.is_taken = false;
    cell.territory.taken_by_group = 0;
    cell.territory.is_ap = false;
    cell.territory.core_cell_of_group = 0;
}

//function to delete groups without members
pub fn delete_groups_without_members(groups: &mut Vec<Groups>) {
    groups.retain(|group| !group_has_no_members(group));
}

pub fn check_for_empty_groups(groups: &Vec<Groups>)  {
    let empty_groups: Vec<usize> = groups.iter().filter(|group| group_has_no_members(group)).map(|group| group.group_id).collect();
    //if !empty_groups.is_empty() {
    if empty_groups.len() > 0 {
       //println!("Empty groups found: {:?}", empty_groups);
        log::info!("Empty groups found: {:?}", empty_groups);
        log::info!("Groups will be deleted {:?}", get_empty_group_ids(groups));
    }
}

// function to return all group ids of groups without members
pub fn get_empty_group_ids(groups: &Vec<Groups>) -> Vec<usize> {
    groups.iter().filter(|group| group_has_no_members(group)).map(|group| group.group_id).collect()
}


// function that takes the vector of group ids of empty groups and goes though the grid freeing all cells taken by those groups
pub fn free_cells_of_empty_groups(groups: &Vec<Groups>, grid: &mut Vec<Vec<Cell>>) {
    let empty_group_ids = get_empty_group_ids(groups);
    if empty_group_ids.len() > 0 {
    log::info!("Empty groups found: {:?}, groups will be deleted", empty_group_ids);
    }
    for group_id in empty_group_ids {
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                if cell.territory.is_taken && cell.territory.taken_by_group == group_id {
                    free_this_cell(cell);
                }
            }
        }
    }
}

pub fn handle_empty_groups(groups: &mut Vec<Groups>, grid: &mut Vec<Vec<Cell>>) {
    delete_groups_without_members(groups);
    free_cells_of_empty_groups(groups, grid);
}

//get all group ids
pub fn get_all_group_ids(groups: &Vec<Groups>) -> Vec<usize> {
    groups.iter().map(|group| group.group_id).collect()
}



   // function called from the persepective of a groupmember that retruns the groupmembers group position using the group id

pub fn get_group_position(model: &Model, my_group: usize) -> (usize, usize) {

    let group = model.groups.iter().find(|group| group.group_id == my_group).unwrap();
    (group.x, group.y)
    
}

