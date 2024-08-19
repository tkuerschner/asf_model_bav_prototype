use crate::*;


#[derive(Debug)]
pub enum GroupBehaviour {
  Homerange_movement,
  Exploratory_movement,
}
#[derive(Debug)]
pub enum DisperserBehaviour {
  Dispersal,
}
#[derive(Debug)]
pub enum RoamerBehaviour {
  Roaming,
  Rutting,
}


impl GroupBehaviour {
pub fn set_behaviour(&self, behaviour: GroupBehaviour) {
    
    match behaviour {
        GroupBehaviour::Homerange_movement => {
            println!("Homerange movement");
        }
        GroupBehaviour::Exploratory_movement => {
            println!("Exploratory movement");
        }
    }
}
    
}

