pub struct entity_tag
{
	countryID : u16,
	stateID : u16,

	provinceID : u16,

	indexID : u16,
}

pub struct amount_recipt
{
	tag : entity_tag,
	amount : f32,
}

pub struct entity_count
{
	tag : entity_tag,
	count : u32,
}

trait eco_entity {
	pub fn addMoney(&mut self, amount : f32);
	pub fn removeMoney(&mut self, amount : f32);
	pub fn getMoney(&self) -> &f32;

	pub fn getType(&self) -> u8;

	
	
}