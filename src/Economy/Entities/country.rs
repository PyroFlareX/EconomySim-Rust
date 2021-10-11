use super::{EcoEntity, EntityType};
use crate::Economy::Market;

pub struct Country {
    id: u16,
    tag: String,

    local_market: Market,

    held_states: Vec<u16>,

    money: f32,
    income_history: [f32; 32],
}

impl Country {
    pub fn new(country_id: u16, country_tag: &str) -> Self {
        Self {
            id: country_id,
            held_states: vec![0; 0],
            tag: country_tag.to_string(),
            local_market: Market::new(),
            money: 10000.0, //10k
            income_history: [0.0; 32],
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_tag(&self) -> &String {
        &self.tag
    }

    pub fn get_market(&self) -> &Market {
        &self.local_market
    }

    pub fn get_market_mut(&mut self) -> &mut Market {
        &mut self.local_market
    }

    pub fn get_income_history(&self) -> &[f32; 32] {
        &self.income_history
    }

    pub fn create_demand_lists(&mut self)
    {
        
    }
}

impl EcoEntity for Country {
    fn add_money(&mut self, amount: f32) {
        self.money += amount;
        self.income_history[31] += amount;
    }

    fn remove_money(&mut self, amount: f32) {
        self.money -= amount;
        self.income_history[31] -= amount;
    }

    fn get_money(&self) -> &f32 {
        &self.money
    }

    fn get_type(&self) -> EntityType {
        EntityType::Country
    }
}
