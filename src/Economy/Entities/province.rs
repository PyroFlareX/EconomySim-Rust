pub struct province
{
	pops_list : vec<pop>,

	provinceID : u16,
	ownerState : u16,

	controller : u16,

	railroad : u8,
	fort : u8,
	liferating : u8,

}

impl province
{
	pub fn new(ID : u16, stateID : u16) -> Self
	{
		Self
		{
			pops_list : vec<pop>::new(),
			provinceID : ID,
			ownerState : stateID,

			controller : 0,

			railroad : 0,
			fort : 0,
			liferating : 0,
		}
	}
}