extern crate csv;
extern crate serde;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;


use clap::App;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    country: String,
    city: String,
    accent_city: Option<String>,
    region: Option<String>,//Option<u32>, //actually we need to handle this error. TODO
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}
// struct Record {
//     #[serde(rename = "Country")]
//     country: String,
//     #[serde(rename = "City")]
//     city: String,
//     #[serde(rename = "AccentCity")]
//     accentcity: String,
//     #[serde(rename = "Region")]
//     region: u32,
//     #[serde(rename = "Population")]
//     population: Option<f64>,
//     #[serde(rename = "Latitude")]
//     latitude: f64,
//     #[serde(rename = "Longitude")]
//     longitude: f64,
// }

// type Record = (String, String, String, u32, Option<f64>, f64, f64);

fn run() -> Result<(), Box<Error>> {
	//TODO Add arg handler
    let version = 0.1;
    let input_file_path = get_first_arg()?;
    let output_file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(input_file_path)?;
    // let mut wtr = csv::Writer::from_path(output_file_path)?;
    {
        // We nest this call in its own scope because of lifetimes.
        let headers = rdr.headers()?;
        println!("{:?}", headers);
    }
    for result in rdr.deserialize() {
        let record: Record = result?;
        if version == 0.1{
	    	let maximum_pop = 1000000;
			let minimum_pop = 100000;
	    	if record.population.map_or(false, |pop| ((pop >= minimum_pop) & (pop <= maximum_pop))) {
	        	println!("{:?}", record);
	        	// wtr.serialize(record)?;
	        }
	    }
	}
    Ok(())
}


#[cfg(feature = "yaml")]
/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}


fn get_option_and_target_file() -> (String, String){
	let args: Vec<String> = env::args().collect();
	let source_filename = &args[1];
	let query = &args[2];
	let target_filename = &args[3];
	println!("Searching for {}", query);
	println!("In file {}", source_filename);
	println!("Writing to {}", target_filename);
	return (query.to_string(),target_filename.to_string())
}


fn main() {
	
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}