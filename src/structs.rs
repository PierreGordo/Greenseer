//this file will hold all the different structs I will be using
//for the DateTime struct, I will be using the Local() time method
//this could cause problems in the future if some parts of the servers were in different timezones
//hovewer this is only a prototype, so I think I can get away with it
use chrono::{ DateTime, Local };
//for ISO standart coordinates format
use iso6709parse::ISO6709Coord;


pub enum AssetStatus {
	Online, Offline,
}

//All types of options - i can add them anytime i want, but so far this is it
pub enum AssetTypes {
	Truck,
	Car,
}


pub struct Asset{
	//vehicle, plane, tank, bmp...
	asset_type: AssetTypes,
	//bmp-2, tatra 815...
	asset_name: &'static str,
	//status - offline, online
	asset_status: AssetStatus, // Either AssetStatus::Online or AssetStatus::Offline,
	//since these fields depend, they are options
	//last GPS ping - could be some, could be none
	last_ping: Option<DateTime<Local>>,
	//coordinates
	last_coords: Option<ISO6709Coord>,
}




//hands off of that for now, will have to look into authentication
/*
pub struct User {
	(),
}
*/
