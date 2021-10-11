pub struct State {
    ID: u16,
    country_owner: u16,

    held_provinces: Vec<u16>,

    name: String,
    // Cache stuff?
}

impl State {
    pub fn new(id: u16, owner: u16, state_name: &str, provinceids: Vec<u16>) -> Self {
        Self {
            ID: id,
            country_owner: owner,
            held_provinces: provinceids,
            name: state_name.to_owned(),
        }
    }
}
