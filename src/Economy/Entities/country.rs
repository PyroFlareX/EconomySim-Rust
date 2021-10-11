use crate::Economy::Market;

pub struct Country {
    ID: u16,

    held_states: Vec<u16>,

    tag: String,

    local_market: Market,
}

impl Country {
    fn new(countryID: u16, country_tag: &str) -> Self {
        Self {
            ID: countryID,
            held_states: vec![0; 0],
            tag: country_tag.to_string(),
            local_market: Market::new(),
        }
    }
}
