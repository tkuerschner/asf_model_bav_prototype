use std::default;

use crate::*;

// Static counter for carcass_id
static mut CARCASS_COUNTER: usize = 0;

// Function to generate a unique carcass_id
pub fn generate_carcass_id() -> usize {
    unsafe {
        CARCASS_COUNTER += 1;
        CARCASS_COUNTER
    }
}

// Define the trait that will be implemented by RoamingIndividual and GroupMember TODO add implementation for disperser
pub trait CarcassSource {
    fn individual_id(&self) -> usize;
    fn age_class(&self) -> AgeClass;
    fn position(&self, model: &Model) -> (usize, usize);
    fn creation_time(&self, model: &Model) -> usize;
    fn is_infected(&self) -> bool;
}

impl CarcassSource for RoamingIndividual {
    fn individual_id(&self) -> usize {
        self.individual_id
    }

    fn age_class(&self) -> AgeClass {
        self.age_class
    }

    fn position(&self, _model: &Model) -> (usize, usize) {
        (self.roamer_x, self.roamer_y)
    }

    fn creation_time(&self, model: &Model) -> usize {
        model.global_variables.current_time
    }

    fn is_infected(&self) -> bool {
        self.health_status == HealthStatus::Infected
    }
}

impl CarcassSource for GroupMember {
    fn individual_id(&self) -> usize {
        self.individual_id
    }

    fn age_class(&self) -> AgeClass {
        self.age_class
    }

    fn position(&self, model: &Model) -> (usize, usize) {
        get_group_position(model, self.current_group_id)
    }

    fn creation_time(&self, model: &Model) -> usize {
        model.global_variables.current_time
    }

    fn is_infected(&self) -> bool {
        self.health_status == HealthStatus::Infected
    }
}

impl CarcassSource for DispersingIndividual {
    fn individual_id(&self) -> usize {
        self.individual_id
    }

    fn age_class(&self) -> AgeClass {
        self.age_class
    }

    fn position(&self, _model: &Model) -> (usize, usize) {
        (self.disp_indiv_x, self.disp_indiv_y)
    }

    fn creation_time(&self, model: &Model) -> usize {
        model.global_variables.current_time
    }

    fn is_infected(&self) -> bool {
        self.health_status == HealthStatus::Infected
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Carcass {
    pub carcass_id: u32,
    pub carcass_x: usize,
    pub carcass_y: usize,
    pub creation_time: usize,
    pub is_infected: bool,
    pub lifetime: u32,
    pub age_class: AgeClass,
}


pub fn create_carcass<T: CarcassSource>(
    source: T,
    model: &mut Model,
    
) {
    let default_lifetime = 10;
    let carcass_id = generate_carcass_id() as u32;
    let (x, y) = source.position(model);
    let carcass = Carcass {
        carcass_id,
        carcass_x: x,
        carcass_y: y,
        creation_time: source.creation_time(model),
        is_infected: source.is_infected(),
        lifetime: default_lifetime,
        age_class: source.age_class(),
    };
    model.carcasses.push(carcass);
}

//implementation : create_carcass(roaming_individual, 10, &model);


