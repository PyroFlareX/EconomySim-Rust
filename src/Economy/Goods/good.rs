pub struct GoodData {
    name: String,
    base_cost: f32,
    id: u8,

    properties: u8,
}

impl GoodData {
    pub fn new(good_id: u8, cost: f32, good_name: &str, good_properties: u8) -> Self {
        Self {
            name: good_name.to_owned(),
            base_cost: cost,
            id: good_id,
            properties: good_properties,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_cost(&self) -> f32 {
        self.base_cost
    }

    //Add the bool getter functions for each type (for property) + make the enum?
}
