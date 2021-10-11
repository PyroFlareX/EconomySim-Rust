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
}
