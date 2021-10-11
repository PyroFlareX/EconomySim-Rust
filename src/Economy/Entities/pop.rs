use super::{EcoEntity, EntityType};

pub struct Pop {
    numpops: u32,

    literacy: f32,
    militancy: f32,
    consciousness: f32,

    money: f32,
    income: f32,
    spending: f32,

    pop_type: u8,

    lifeneeds: Vec<f32>,
    dailyneeds: Vec<f32>,
    luxuryneeds: Vec<f32>,
}

impl Pop {
    pub fn new(size: u32, pop_type: u8) -> Self {
        Self {
            numpops: size,
            literacy: 0.0,
            militancy: 0.0,
            consciousness: 0.0,
            money: 0.0,
            income: 0.0,
            spending: 0.0,

            pop_type: pop_type,

            lifeneeds: vec![0.0; 256],
            dailyneeds: vec![0.0; 256],
            luxuryneeds: vec![0.0; 256],
        }
    }

    fn demand_unit_multiplier(&self) -> f32 {
        let count_per_unit_define: f32 = 1.0 / 125000.0;

        (self.numpops as f32) * count_per_unit_define
    }
}

impl EcoEntity for Pop {
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
        /*if self.pop_type == poptype::artisan
        {
            return EntityType::artisan;
        }*/
        EntityType::Pop
    }
}
