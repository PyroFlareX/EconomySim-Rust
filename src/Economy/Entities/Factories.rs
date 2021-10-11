use super::{EcoEntity, EntityType};

struct Factory {
    state_id: u16,
    index: u8,

    good_temp: u8,

    // employee_list : vec
    money: f32,
    income: f32,
    spending: f32,

    goods_inventory: Vec<f32>,
}

impl Factory {
    pub fn new(state_id: u16, index: u8, temp_arg: u8) -> Self {
        Self {
            state_id: state_id,
            index: index,

            good_temp: temp_arg,

            money: 100.0,
            income: 0.0,
            spending: 0.0,

            goods_inventory: vec![0.0; 256],
        }
    }

    pub fn get_index(&self) -> u8 {
        self.index
    }

    pub fn get_state(&self) -> u16 {
        self.state_id
    }
}

impl EcoEntity for Factory {
    fn add_money(&mut self, amount: f32) {
        self.money += amount;
        self.income += amount;
    }

    fn remove_money(&mut self, amount: f32) {
        self.money -= amount;
        self.spending += amount;
    }

    fn get_money(&self) -> &f32 {
        &self.money
    }

    fn get_type(&self) -> EntityType {
        EntityType::Factory
    }
}
