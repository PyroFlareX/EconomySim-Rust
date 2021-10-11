use std::array;

use crate::Economy::Entities::*;

pub struct WorldMarket {
    //Could do a single buffered market or a double buffered market
    // ironically doing the markets as demand oriented, ig Keynesian per se, is far more optimizable and less error-prone
    // to the earlier design which was production oriented (supply side if you really like the metaphor)
    global_market: Market,

    prices: Vec<f32>,
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

    pub fn addDemandRequest(&mut self, recipt: AmountRecipt, good_type: u8) {
        self.demand_recipt_list[good_type as usize].push(recipt);
    }

    pub fn addSupplyAmount(&mut self, recipt: AmountRecipt, good_type: u8) {
        self.supply_recipt_list[good_type as usize].push(recipt);
    }
}
