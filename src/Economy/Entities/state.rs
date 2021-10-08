pub struct state
{
	ID : u16,
	countryOwner : u16,

	heldProvinces : vec<u16>,

	name : String,

	// Cache stuff?
}

impl state
{
	pub fn new(id : u16, owner : u16, state_name : &str, provinceids : vec<u16>) -> Self
	{
		Self
		{
			ID : id,
			countryOwner : owner,
			heldProvinces : provinceids,
			name : state_name.to_owned(),

		}
	}
}