

pub struct good_data
{
	name : String,
	baseCost : f32,
	id : u8,
	
	properties : u8,

	//Maybe add:
	/*
	color : vec3
	textureID : u16
	*/
}

impl good_data
{
	pub fn new(good_id : u8, cost : f32, good_name : &str, good_properties : u8) -> Self
	{
		Self
		{
			good_name.to_owned(),
			cost,
			good_id,
			good_properties,
		}
	}

	pub fn get_name(&self) -> &String
	{
		self.name
	}

	pub fn get_id(&self) -> u8
	{
		self.id
	}

	pub fn get_cost(&self) -> f32
	{
		self.baseCost
	}

	//Add the bool getter functions for each type + make the enum?
}