#[derive(Clone, Copy, Debug)]
pub struct EntityTag {
    countryID: u16,
    stateID: u16,

    provinceID: u16,

    indexID: u16,
}

impl EntityTag {
    pub fn new(country_ID: u16, state_ID: u16, province_ID: u16, index_ID: u16) -> Self {
        Self {
            countryID: country_ID,
            stateID: state_ID,
            provinceID: province_ID,
            indexID: index_ID,
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

trait EcoEntity {
    fn addMoney(&mut self, amount: f32);
    fn removeMoney(&mut self, amount: f32);
    fn getMoney(&self) -> &f32;

    fn getType(&self) -> u8;
}
