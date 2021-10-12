use super::{EntityCount, Pop};

pub struct Province {
    pops_list: Vec<Pop>,

    employee_list: Vec<EntityCount>,

    province_id: u16,
    owner_state: u16,
    controller: u16,

    rgo_good_id: u8,

    railroad: u8,
    fort: u8,
    liferating: u8,
}

impl Province {
    pub fn new(id: u16, state_id: u16) -> Self {
        Self {
            pops_list: Vec::new(),
            employee_list: Vec::new(),
            province_id: id,
            owner_state: state_id,
            controller: 0,

            rgo_good_id: 0,

            railroad: 0,
            fort: 0,
            liferating: 0,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.province_id
    }
    fn get_pop(&self, index: u8) -> &Pop {
        &self.pops_list[index as usize]
    }
    fn get_pop_mut(&mut self, index: u8) -> &mut Pop {
        &mut self.pops_list[index as usize]
    }
    pub fn get_pops(&self) -> &Vec<Pop> {
        &self.pops_list
    }
    pub fn get_pops_mut(&mut self) -> &mut Vec<Pop> {
        &mut self.pops_list
    }
}
