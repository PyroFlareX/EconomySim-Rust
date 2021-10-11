use crate::Economy::Entities::*;

pub struct WorldMarket {
    //Could do a single buffered market or a double buffered market
    // ironically doing the markets as demand oriented, ig Keynesian per se, is far more optimizable and less error-prone
    // to the earlier design which was production oriented (supply side if you really like the metaphor)
    global_market: Market,

    prices: Vec<f32>,

    previous_demand: Vec<f32>,
    previous_supply: Vec<f32>,
}

impl WorldMarket {
    pub fn new() -> Self {
        Self {
            global_market: Market::new(),
            prices: vec![0.0; 256],
            previous_demand: vec![0.0; 256],
            previous_supply: vec![0.0; 256],
        }
    }

    pub fn get_market(&self) -> &Market {
        &self.global_market
    }

    pub fn get_market_mut(&mut self) -> &mut Market {
        &mut self.global_market
    }

    pub fn reset(&mut self) {
        let change_amount = 0.01;

        for i in 0..255 {
            if self.global_market.demand[i as usize] > self.global_market.supply[i as usize] {
                self.prices[i] += change_amount;
            } else if self.global_market.demand[i as usize] < self.global_market.supply[i as usize]
            {
                self.prices[i] += change_amount;
            }

            //Add in a min / max for goods prices

            //Hist stuff
            self.previous_demand[i as usize] = self.global_market.demand[i as usize];
            self.previous_supply[i as usize] = self.global_market.supply[i as usize];
        }
    }

    pub fn get_good_price(&self, good_id: u8) -> &f32 {
        &self.prices[good_id as usize]
    }

    pub fn merge_country_market(&mut self, country_market: &mut Market) {
        for i in 0..255 {
            self.global_market.actual_bought[i] += country_market.actual_bought[i];
            self.global_market.demand[i] += country_market.demand[i];
            self.global_market.supply[i] += country_market.supply[i];

            //Moving the demand lists
            self.global_market.demand_recipt_list[i]
                .append(&mut country_market.demand_recipt_list[i]);
            //Moving the supply lists
            self.global_market.supply_recipt_list[i]
                .append(&mut country_market.supply_recipt_list[i]);
        }
    }
}

pub struct Market {
    demand_recipt_list: Vec<Vec<AmountRecipt>>,
    supply_recipt_list: Vec<Vec<AmountRecipt>>,

    demand: [f32; 256],
    supply: [f32; 256],
    actual_bought: [f32; 256],
}

impl Market {
    pub fn new() -> Self {
        let list: Vec<AmountRecipt> = Vec::new();
        let d_array = vec![list; 256];
        let s_array = d_array.clone();

        Self {
            demand_recipt_list: d_array,
            supply_recipt_list: s_array,
            demand: [0.0; 256],
            supply: [0.0; 256],
            actual_bought: [0.0; 256],
        }
    }

    pub fn reset(&mut self) {
        for i in 0..255 {
            self.demand[i as usize] = 0.0;
            self.supply[i as usize] = 0.0;
            self.actual_bought[i as usize] = 0.0;

            self.demand_recipt_list[i as usize].clear();
            self.supply_recipt_list[i as usize].clear();
        }
    }

    pub fn add_supply_amount(&mut self, good_type: u8, amount: f32) {
        self.supply[good_type as usize] += amount;
    }

    pub fn add_demand_request(&mut self, recipt: AmountRecipt, good_type: u8) {
        self.demand_recipt_list[good_type as usize].push(recipt);
    }

    pub fn add_supply_recept(&mut self, recipt: AmountRecipt, good_type: u8) {
        self.supply_recipt_list[good_type as usize].push(recipt);
    }

    pub fn get_good_demand_list(&self, good_type: u8) -> &Vec<AmountRecipt> {
        &self.demand_recipt_list[good_type as usize]
    }

    pub fn get_good_demand_list_mut(&mut self, good_type: u8) -> &mut Vec<AmountRecipt> {
        &mut self.demand_recipt_list[good_type as usize]
    }

    pub fn get_good_supply_list(&self, good_type: u8) -> &Vec<AmountRecipt> {
        &self.demand_recipt_list[good_type as usize]
    }

    pub fn get_good_supply_list_mut(&mut self, good_type: u8) -> &mut Vec<AmountRecipt> {
        &mut self.demand_recipt_list[good_type as usize]
    }
}
