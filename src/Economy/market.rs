
use Entities::eco_entity;

pub struct world_market
{
	//Could do a single buffered market or a double buffered market
	// ironically doing the markets as demand oriented, ig Keynesian per se, is far more optimizable and less error-prone
	// to the earlier design which was production oriented (supply side if you really like the metaphor)

	global_market : market,

	prices : vec<f32>,
}

pub struct market
{
	demand_recipt_list : vec<vec<amount_recipt>>,
	supply_recipt_list : vec<vec<amount_recipt>>,

	demand : vec<f32>,
	supply : vec<f32>,
	actual_bought : vec<f32>,
}

impl market
{
	pub fn new() -> Self
	{
		Self
		{
			demand_recipt_list : vec<vec<amount_recipt>>::new(),
			supply_recipt_list : vec<vec<amount_recipt>>::new(),
			demand : vec![0.0; 256],
			supply : vec![0.0; 256],
			actual_bought : vec![0.0; 256],
		}
	}

	pub fn reset(&mut self)
	{

	}

	pub fn addDemandRequest(&mut self, recipt : amount_recipt, good_type : u8)
	{
		self.demand_recipt_list
	}

	pub fn addSupplyAmount(&mut self, recipt : amount_recipt, good_type : u8)
	{

	}
}