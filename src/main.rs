use std::{cell::RefCell, cell::RefMut, collections::HashMap, ops::AddAssign};

use Economy::EntityTag;

use crate::Economy::{Country, EcoEntity, GoodData, Province, State, WorldMarket};

mod Economy;

#[allow(non_snake_case)]

fn main() {
    println!("Hello, world!");

    let x: f32 = 0.4;

    println!("{}", x);

    let mut country_map: HashMap<u16, RefCell<Country>> = HashMap::new();
    let mut state_map: HashMap<u16, RefCell<State>> = HashMap::new();
    let mut province_map: HashMap<u16, RefCell<Province>> = HashMap::new();

    // let mut goods_map: HashMap<u8, GoodData> = HashMap::new(); //OR
    let mut goods_list: Vec<Option<GoodData>> = Vec::new(); // or (not working) vec![&Option::None; 256];
    for _i in 0..256 {
        goods_list.push(None);
    }
    assert_eq!(goods_list.len(), 256);

    //Gotta load the goods, country, state, provinces, pops
    //Goods for now:
    let good_example = GoodData::new(0, 2.5, "good_name", 0);
    goods_list[0] = Some(good_example);

    //Countries
    for i in 0..16 {
        let mut tag = "T".to_string();
        tag.push_str(i.to_string().as_str());

        country_map
            .get_mut(&i)
            .insert(&mut RefCell::new(Country::new(i, tag.as_str())));
    }

    //States
    for i in 0..16 {
        let mut name = "State ".to_string();
        name.push_str(i.to_string().as_str());

        let prov_ids = vec![i, i + 1, i + 2, i + 3];

        state_map.get_mut(&i).insert(&mut RefCell::new(State::new(
            i,
            i / 2,
            name.as_str(),
            prov_ids,
        )));
    }

    //Provinces
    for i in 0..64 {
        province_map
            .get_mut(&i)
            .insert(&mut RefCell::new(Province::new(i, i / 4)));
    }

    // Including pops

    //Other init

    let mut world_market = WorldMarket::new();

    //Now, begin the economy loop

    let mut cont = true;
    let mut num_iterations = 0;

    while cont {
        // First Clear from previous
        world_market.reset();

        // Then (in parallel too), create the demand lists for each country
        for (_country_id, country) in &country_map {
            country.borrow_mut().create_demand_lists();
        }

        // Then (in parallel too) execute the local country economy tick

        // Then move the local economies into the world market (by rank)
        //      Calculate the rank, this is not going to be done for this, since this is only partial, but /shrug
        for (_country_id, country) in &mut country_map {
            world_market.merge_country_market(country.borrow_mut().get_market_mut());
        }

        // Match remaining supply and demand inside the world economy (Pretty sure required to be sequential sadly)
        //      Within this, also do the pop ticks for growth, migration, assimilation, etc

        //Iterates for each good
        for (index, good) in goods_list.iter().enumerate() {
            let good_id = index as u8;
            let demand_list = world_market
                .get_market()
                .get_good_demand_list(good_id)
                .clone();

            let good_price = world_market.get_good_price(good_id);

            for demand_recipt in demand_list.iter() {
                if demand_recipt.get_amount() <= 0.0 as f32 {
                    continue;
                }

                //This is so it doesn't have to be borrowed mutably, makes a copy
                let mut remaining_demand = demand_recipt.get_amount();

                for supply_recipt in world_market
                    .get_market_mut()
                    .get_good_supply_list_mut(good_id)
                    .iter_mut()
                {
                    if remaining_demand <= 0.0 {
                        break;
                    }
                    if supply_recipt.get_amount() <= 0.0 {
                        continue;
                    }

                    //For the Buyer
                    let mut buyer_country = country_map
                        .get(&demand_recipt.get_tag().get_country_id())
                        .expect("The country ID passed for the demand recipt is not valid.")
                        .borrow_mut();

                    let mut buyer_province = province_map
                        .get(&demand_recipt.get_tag().get_province_id())
                        .expect("The province ID passed for the demand recipt is not valid.")
                        .borrow_mut();

                    let mut buyer = &mut buyer_province.get_pops_mut()
                        [demand_recipt.get_tag().get_index_id() as usize];

                    // This is for the supplier, impled through an Eco Entity Trait
                    let mut supplier_prov = province_map
                        .get(&demand_recipt.get_tag().get_province_id())
                        .expect("The province ID passed for the supply recipt is not valid.")
                        .borrow_mut();

                    let mut supplier = &mut supplier_prov.get_pops_mut()
                        [demand_recipt.get_tag().get_index_id() as usize];

                    //Now here, I cross check between the available supply and the demand to match them to one another
                    let buyable = (buyer.get_money() / good_price).min(supply_recipt.get_amount());
                    let spending = buyable * good_price;

                    supply_recipt.get_amount_mut().add_assign(-buyable);
                    remaining_demand -= buyable;

                    from_seller_to_buyer(good_id, buyable, spending, supplier, buyer);
                    //Other function that I wish I could use but I don't fully understand traits yet, so /shrug
                    //supplier.good_transaction(&mut buyer, good_id, buyable, spending);
                }
            }
        }
        // Do the pop stuff
        for (_country_id, country) in &mut country_map {
            //The pop ticks for growth, immigration, etc. will happen here
        }

        // Do any other thing that is required that I forgot

        if num_iterations > 100 {
            cont = false;
        }
        num_iterations += 1;
    }
}

fn from_seller_to_buyer<T, U>(
    good_id: u8,
    good_amount: f32,
    money_spending: f32,
    seller: &mut T,
    buyer: &mut U,
) where
    T: EcoEntity,
    U: EcoEntity,
{
    buyer.remove_money(money_spending);
    buyer.add_to_inventory(good_id, good_amount);

    //Have to add taxes and other stuff
    seller.add_money(money_spending);
    //seller.remove_from_inventory(good_id, good_amount);
}

fn get_info_tuple(
    country_map: &HashMap<u16, RefCell<Country>>,
    state_map: &HashMap<u16, RefCell<State>>,
    province_map: &HashMap<u16, RefCell<Province>>,
    entity: &EntityTag,
) -> (RefMut<Country>, RefMut<State>, RefMut<Province>) {

    let mut country = country_map
        .get(&entity.get_country_id())
        .expect("The country ID passed for the demand recipt is not valid.")
        .borrow_mut();
    
    let mut province = province_map
        .get(&entity.get_province_id())
        .expect("The province ID passed for the demand recipt is not valid.")
        .borrow_mut();
    
    let mut state = state_map
        .get(&entity.get_state_id())
        .expect("The province ID passed for the demand recipt is not valid.")
        .borrow_mut();

    (country, state, province)
}
