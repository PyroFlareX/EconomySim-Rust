use super::Pop;

pub struct Province {
    pops_list: Vec<Pop>,

    provinceID: u16,
    ownerState: u16,

    controller: u16,

    railroad: u8,
    fort: u8,
    liferating: u8,
}

impl Province {
    pub fn new(ID: u16, stateID: u16) -> Self {
        Self {
            pops_list: Vec::new(),
            provinceID: ID,
            ownerState: stateID,

            controller: 0,

            railroad: 0,
            fort: 0,
            liferating: 0,
        }
    }
}
