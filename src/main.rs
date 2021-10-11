use std::collections::HashMap;

use crate::Economy::{Country, GoodData, Province, State, WorldMarket};

mod Economy;

#[allow(non_snake_case)]

fn main() {
    println!("Hello, world!");

    let x: f32 = 0.4;

    println!("{}", x);

    let mut country_map: HashMap<u16, Country> = HashMap::new();
    let mut state_map: HashMap<u16, State> = HashMap::new();
    let mut province_map: HashMap<u16, Province> = HashMap::new();

    let mut goods_map: HashMap<u8, GoodData> = HashMap::new();


    //Gotta load the goods, country, state, provinces, pops
    //Goods for now:
    for i in 0..255
    {
        let good = GoodData::new(i, 2.5, "good_name", 0);
        goods_map.insert(i, good);
    }
    //Countries

    //States

    //Provinces

    // Including pops

    //Other init

    let mut world_market = WorldMarket::new();

    //Now, begin the economy loop

    let mut cont = true;

    while cont
    {
        // First Clear from previous
        world_market.reset();

        // Then (in parallel too), create the demand lists for each country
        

        // Then (in parallel too) execute the local country economy tick


        // Then move the local economies into the world market (by rank)


        // Match remaining supply and demand inside the world economy (Pretty sure required to be sequential sadly)
        //      Within this, also do the pop ticks for growth, migration, assimilation, etc

        
        // Do any other thing that is required that I forgot
    }
}
