use crate::*;

static mut HIGH_SEAT_COUNTER: usize = 0;

pub fn generate_high_seat_id() -> usize {
    unsafe {
        HIGH_SEAT_COUNTER += 1;
        HIGH_SEAT_COUNTER
    }
}


pub struct HighSeat {
    pub x_hs: usize,
    pub y_hs: usize,
    pub hs_id: usize,
    pub is_occupied: bool,
    pub range: usize, // range of the high seat - mean 150m
}

impl HighSeat {
    pub fn new(x_hs: usize, y_hs: usize, hs_id: usize, is_occupied: bool, range: usize) -> HighSeat {
        HighSeat {
            x_hs,
            y_hs,
            hs_id,
            is_occupied,
            range,
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
        [self.x_hs, self.y_hs]
    }


}




//1 hs per 10 ha / 10 ha is 100000 m^2 cell size is 50mx50m i.e. 2500 m^2 so 40 cells per 10 ha so 1 hs per 40 cells

pub fn place_high_seats(model: &mut Model){
// place one high seat per 40 cells in valid cells
    let mut hs_counter = 0;
    for i in 0..model.grid.len(){
        for j in 0..model.grid[i].len(){
            if model.grid[i][j].is_valid{
                if hs_counter % 40 == 0{
                    let hs = HighSeat{
                        x_hs: i,
                        y_hs: j,
                        hs_id: generate_high_seat_id(),
                        is_occupied: false,
                        range: 150,
                    };
                    model.high_seats.push(hs);
                }
                hs_counter += 1;
            }
        }
    }
}


