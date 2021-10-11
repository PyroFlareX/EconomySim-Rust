pub struct State {
    id: u16,
    country_owner: u16,

    held_provinces: Vec<u16>,

    name: String,
    // Cache stuff?
    constructed: bool,
}

impl State {
    pub fn new(id: u16, owner: u16, state_name: &str, provinceids: Vec<u16>) -> Self {
        Self {
            id: id,
            country_owner: owner,
            held_provinces: provinceids,
            name: state_name.to_owned(),
            constructed: false,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_provinces(&self) -> &Vec<u16> {
        &self.held_provinces
    }
}
