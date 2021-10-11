use std::collections::HashMap;

use crate::Economy::{Country, GoodData, Province, State};

mod Economy;

#[allow(non_snake_case)]

fn main() {
    println!("Hello, world!");

    let x: f32 = 0.4;

    println!("{}", x);

    let mut country_map: HashMap<u16, Country> = HashMap::new();
    let mut state_map: HashMap<u16, State> = HashMap::new();
    let mut province_map: HashMap<u16, Province> = HashMap::new();

    let mut goods_map: HashMap<u16, GoodData> = HashMap::new();

    //Gotta load the goods, country, state, provinces, pops


    //Other init


    //Now, begin the economy loop

    
}
