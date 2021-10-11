#[derive(Clone, Copy, Debug)]
pub struct EntityTag {
    country_id: u16,
    state_id: u16,

    province_id: u16,

    index_id: u16,
}

impl EntityTag {
    pub fn new(country_id: u16, state_id: u16, province_id: u16, index_id: u16) -> Self {
        Self {
            country_id: country_id,
            state_id: state_id,
            province_id: province_id,
            index_id: index_id,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct AmountRecipt {
    tag: EntityTag,
    amount: f32,
}

impl AmountRecipt {
    pub fn new(tag: EntityTag, amount: f32) -> Self {
        Self {
            tag: tag,
            amount: amount,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct EntityCount {
    tag: EntityTag,
    count: u32,
}

impl EntityCount {
    pub fn new(tag: EntityTag, amount: u32) -> Self {
        Self {
            tag: tag,
            count: amount,
        }
    }
}

pub enum EntityType {
    Pop = 0,
    Artisan = 1,
    Country = 2,
    Factory = 3,
    Building = 4,
}

pub trait EcoEntity {
    fn add_money(&mut self, amount: f32);
    fn remove_money(&mut self, amount: f32);
    fn get_money(&self) -> &f32;

    fn get_type(&self) -> EntityType;
}
