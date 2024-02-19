use crate::*;


pub fn group_setup(cell_info_list: &Vec<CellInfo>,  grid: &mut Vec<Vec<Cell>>, num_groups: usize) -> Vec<Groups> { 

    // Create individuals with unique IDs, group IDs, and memory
    let mut group: Vec<Groups> = Vec::with_capacity(num_groups);
    let grid_size = grid.len();  

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

        let desired_total_cells = 1600;     
 
        circular_bfs(grid, x, y, group_id, desired_total_cells); // fill the territory with cells
        
        let presence_timer = 0; 

        let memory = GroupMemory { // create memory for the group
            known_cells: HashSet::new(),
            group_member_ids: Vec::new(),
            known_cells_order: Vec::new(),
            presence_timer,
        };

        let target_cell = None; 
        let remaining_stay_time = 0;
        let movement = MovementMode::Foraging;
        let group_members = vec![];
        let daily_movement_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE; //<--------------------DEBUG FIX ME with actual values
        

        group.push(Groups { // create the group
            group_id,
            x,
            y,
            core_cell: Some(core_cell),
            target_cell,
            remaining_stay_time,
            memory,
            movement,
            group_members,
            daily_movement_distance,
        });

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

        for _ in 0..tmp_size {
            group.create_new_initial_group_member();
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

pub fn update_group_memory(group: &mut Vec<Groups>) {
    // Get the indices of individuals
   // let indices: Vec<usize> = (0..group.len()).collect();
//
   // // Iterate through indices to update group memory
   // for &index in &indices {
   //     let group_id = group[index].group_id;
//
   //     // Find indices of group members with the same group_id
   //     let group_members_ids: Vec<usize> = indices
   //         .iter()
   //         .filter(|&&i| group[i].group_id == group_id)
   //         .map(|&i| group[i].id)
   //         .collect();
//
   //     // Update group memory with the IDs of group members
   //     group[index].memory.group_member_ids = group_members_ids;
//
   //     // Print debug information
   //     //println!(
   //     //    "Individual {}: Group ID: {}, Group members: {:?}",
   //     //    index, group_id, individuals[index].memory.group_member_ids
   //     //);
   // }
}

// function to add a new group at a specific set of coordinates
pub fn add_new_group_at_location(groups: &mut Vec<Groups>, grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    let group_id = generate_group_id(); // add incrementing group id
    occupy_this_cell(&mut grid[x][y], group_id); // occupy the selected ap
    let core_cell = (x, y); // set the core cell
    make_core_cell(&mut grid[x][y], group_id);
    let desired_total_cells = 1600; // FIX ME DEBUG
    circular_bfs(grid, x, y, group_id, desired_total_cells); // fill the territory with cells
    let presence_timer = 0;
    let memory = GroupMemory {
        known_cells: HashSet::new(),
        group_member_ids: Vec::new(),
        known_cells_order: Vec::new(),
        presence_timer,
    };
    let target_cell = None;
    let remaining_stay_time = 0;
    let movement = MovementMode::Foraging;
    let group_members = vec![];
    let daily_movement_distance = DEFAULT_DAILY_MOVEMENT_DISTANCE; //<--------------------DEBUG FIX ME with actual values

    groups.push(Groups {
        group_id,
        x,
        y,
        core_cell: Some(core_cell),
        target_cell,
        remaining_stay_time,
        memory,
        movement,
        group_members,
        daily_movement_distance,
    });
}