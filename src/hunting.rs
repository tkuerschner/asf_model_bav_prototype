use std::option;

use crate::*;

static mut HIGH_SEAT_COUNTER: usize = 0;

pub fn generate_high_seat_id() -> usize {
    unsafe {
        HIGH_SEAT_COUNTER += 1;
        HIGH_SEAT_COUNTER
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HighSeat {
    pub x_hs: usize,
    pub y_hs: usize,
    pub hs_id: usize,
    pub is_occupied: bool,
    pub range: usize, // range of the high seat - mean 150m
    pub success_rate: f64, // success rate of the high seat
    pub num_hunted: usize, // number of individuals hunted
    pub successful_hunt: bool,
}

impl HighSeat {
    pub fn new(x_hs: usize, y_hs: usize, hs_id: usize, is_occupied: bool, range: usize) -> HighSeat {
        HighSeat {
            x_hs,
            y_hs,
            hs_id,
            is_occupied,
            range,
            success_rate: 0.25,
            num_hunted: 0,
            successful_hunt: false,
        }
    }

    pub fn occupy_high_seat(&mut self) {
        self.is_occupied = true;
    }

    pub fn leave_high_seat(&mut self) {
        self.is_occupied = false;
    }

    pub fn set_range(&mut self, range: usize) {
        self.range = range;
    }

    pub fn as_point(&self) -> [f64; 2] {
        [self.x_hs as f64, self.y_hs as f64]
    }


}
#[derive(Debug, Clone)]
pub struct HuntingStatistics {
  pub hunted_individuals: Vec<HuntedIndividuals>,
}
#[derive(Debug, Clone)]
pub struct HuntedIndividuals {
    pub x: usize,
    pub y: usize,
    pub sx: Sex,
    pub age: u32,
    pub age_class: AgeClass,
    pub id: usize,
    pub origin_group: option::Option<usize>,
    pub type_individual: IndividualType,
    pub time: usize,
}
#[derive(Debug, Clone)]
pub enum IndividualType {
    Roamer,
    GroupMember,
    Disperser, 
}

impl fmt::Display for IndividualType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IndividualType::Roamer => write!(f, "Roamer"),
            IndividualType::GroupMember => write!(f, "GroupMember"),
            IndividualType::Disperser => write!(f, "Disperser"),
        }
    }
}

impl HuntingStatistics {
    pub fn new() -> HuntingStatistics {
        HuntingStatistics {
            hunted_individuals: Vec::new(),
        }
    }

    pub fn add_hunted_individual(&mut self, x: usize, y: usize, sx: Sex, age: u32, age_class: AgeClass, id: usize, origin_group: option::Option<usize>, type_individual: IndividualType, time: usize) {
        let hunted_individual = HuntedIndividuals {
            x,
            y,
            sx,
            age,
            age_class,
            id,
            origin_group,
            type_individual,
            time
        };
        self.hunted_individuals.push(hunted_individual);
    }

}


pub fn handle_high_seats_initial(model: &mut Model, rng: &mut impl Rng, initial_high_seats_occupancy: f64) {

    place_high_seats(model);
    occupy_high_seats(model, rng, initial_high_seats_occupancy);
    create_hunting_zone(model);

}


//1 hs per 10 ha / 10 ha is 100000 m^2 cell size is 50mx50m i.e. 2500 m^2 so 40 cells per 10 ha so 1 hs per 40 cells

//pub fn place_high_seats(model: &mut Model) {
//    let mut hs_counter = 0;
//    for i in 0..model.grid.len() {
//        for j in 0..model.grid[i].len() {
//            if model.grid[i][j].is_valid() {
//                if hs_counter % 40 == 0 {
//                    let hs = HighSeat::new(
//                        i , 
//                        j , 
//                        generate_high_seat_id(), 
//                        false, 
//                        3,
//                    );
//                    model.high_seats.push(hs.clone());
//                }
//                hs_counter += 1;
//            }
//        }
//    }
//}

pub fn place_high_seats(model: &mut Model) {
    
//1 hs per 10 ha / 10 ha is 100000 m^2 cell size is 50mx50m i.e. 2500 m^2 so 40 cells per 10 ha so 1 hs per 40 cells
    let cells_per_hs = 240;//TEST go back to 40
    let mut hs_counter = 0;

    let mut i = 0;
    while i < model.grid.len() {
        let mut j = 0;
        while j < model.grid[i].len() {
            if model.grid[i][j].is_valid() {
                if hs_counter == 0 {
                    // Place a high seat at this grid position
                    let hs = HighSeat::new(
                        i,
                        j,
                        generate_high_seat_id(),
                        false,
                        3,
                    );
                    model.high_seats.push(hs.clone());

                    // Skip the next `cells_per_hs` cells to ensure spacing
                    hs_counter = cells_per_hs;
                } else {
                    hs_counter -= 1;
                }
            }
            j += 1; // Increment `j` to move to the next cell in the row
        }
        i += 1; // Increment `i` to move to the next row in the grid
    }
}





pub fn create_hunting_zone(model: &mut Model) {
    for hs in &model.high_seats {
        if hs.is_occupied {
        let x = hs.x_hs;
        let y = hs.y_hs;
        let range = hs.range;
        for i in 0..model.grid.len() {
            for j in 0..model.grid[i].len() {
                if (i as i32 - x as i32).abs() + (j as i32 - y as i32).abs() <= range as i32 {
                    model.grid[i][j].set_hunting_zone();
                    model.grid[i][j].associated_high_seat = Some(hs.hs_id);
                    }
                }
            }
        }
    }
}


pub fn hunting_check(grid: &mut Vec<Vec<Cell>>, hs_vec: &mut Vec<HighSeat> , rng: &mut impl Rng, my_x: usize, my_y: usize) -> bool {
    // check if the grid cell is a hunting zone
    if grid[my_x][my_y].hunting_zone {
        // get the high seat id
        let hs_id = grid[my_x][my_y].associated_high_seat.unwrap();
        // get the high seat
        let hs = hs_vec.iter_mut().find(|hs| hs.hs_id == hs_id).unwrap();
             
        // check if the hunt is successful
        let success = rng.gen_bool(hs.success_rate);
        if success {
            hs.num_hunted += 1;
            hs.successful_hunt = true;
            // leave the high seat after successful hunt
            leave_high_seats_after_success(hs_id, hs_vec);
            remove_hunting_zone_of_hs(grid, hs_id);
            true
        } else {
            false
        }
    }
    else {
        false
    }
}

pub fn occupy_high_seats(model: &mut Model, rng: &mut impl Rng, percentage: f64) {
   //occupy 10% of high seats
    let num_occupied = (model.high_seats.len() as f64 * 0.1) as usize;
    let mut occupied_hs = 0;
    while occupied_hs < num_occupied {
        let idx = rng.gen_range(0..model.high_seats.len());
        if !model.high_seats[idx].is_occupied {
            model.high_seats[idx].occupy_high_seat();
            occupied_hs += 1;
        }
    }
}

pub fn leave_all_high_seats(model: &mut Model) {
    //leave all high seats
    for hs in &mut model.high_seats {
        hs.leave_high_seat();
    }
}

pub fn leave_high_seats_after_success(hs_id: usize, hs_vec: &mut Vec<HighSeat>) {
    //leave high seat after hunt
    for hs in hs_vec {
        if hs.hs_id == hs_id {
            hs.leave_high_seat();
            
        }
    }
}

pub fn remove_hunting_zone_of_hs(grid: &mut Vec<Vec<Cell>>, hs_id: usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j].associated_high_seat == Some(hs_id) {
                grid[i][j].hunting_zone = false;
                grid[i][j].associated_high_seat = None;
            }
        }
    }
}

pub fn remove_all_hunting_zones(grid: &mut Vec<Vec<Cell>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j].hunting_zone = false;
            grid[i][j].associated_high_seat = None;
        }
    }
}


pub fn shuffle_high_seat_occupancy(model: &mut Model, rng: &mut impl Rng, occupancy_rate: f64) {
    //leave all high seats
    leave_all_high_seats(model);
    remove_all_hunting_zones(&mut model.grid);
    //occupy 10% of high seats
    occupy_high_seats(model, rng, occupancy_rate);
    create_hunting_zone(model);
}

