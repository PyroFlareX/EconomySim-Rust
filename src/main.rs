use std::collections::HashMap;

use Economy::EntityTag;

use crate::Economy::{Country, EcoEntity, GoodData, Province, State, WorldMarket};

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
    for i in 0..255 {
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

    while cont {
        // First Clear from previous
        world_market.reset();

        // Then (in parallel too), create the demand lists for each country
        for (_country_id, country) in &mut country_map {
            country.create_demand_lists();
        }

        // Then (in parallel too) execute the local country economy tick

        // Then move the local economies into the world market (by rank)
        //      Calculate the rank, this is not going to be done for this, since this is only partial, but /shrug
        for (_country_id, country) in &mut country_map {
            world_market.merge_country_market(country.get_market_mut());
        }

        // Match remaining supply and demand inside the world economy (Pretty sure required to be sequential sadly)
        //      Within this, also do the pop ticks for growth, migration, assimilation, etc
        for good_id in 0..255 {
            for demand_recipt in world_market
                .get_market()
                .get_good_demand_list(good_id)
                .iter()
            {
                if demand_recipt.get_amount() <= 0.0 as f32 {
                    continue;
                }

                let mut remaining_demand = demand_recipt.get_amount();

                let mut buyer_country = country_map
                    .get_mut(&demand_recipt.get_tag().get_country_id())
                    .expect("The country ID passed for the demand recipt is not valid.");

                let mut buyer: Box<dyn EcoEntity>;// = Box::new(buyer_country); // = // This is the buyer implemented through an EcoEntity Trait

                for supply_recipt in world_market
                    .get_market()
                    .get_good_supply_list(good_id)
                    .iter()
                {
                    if remaining_demand <= 0.0 {
                        break;
                    }
                    if supply_recipt.get_amount() <= 0.0 {
                        continue;
                    }

                    let mut supplier: Box<dyn EcoEntity>; // This is for the supplier, impled through an Eco Entity Trait

                    //Now here, I cross check between the available supply and the demand to match them to one another
                    let buyable = (buyer.get_money() / world_market.get_good_price(good_id))
                        .min(supply_recipt.get_amount());
                    // supply_recipt.get_amount()
                }
            }
        }
        // Do the pop stuff
        for (_country_id, country) in &mut country_map {
            //The pop ticks for growth, immigration, etc. will happen here
        }

        // Do any other thing that is required that I forgot
    }
}
/*
fn getEconomicEntity(tag : &EntityTag) -> &Box<dyn EcoEntity>
{
    let p = Province::new(0, 0);

    Box::new(&p)
}*/
