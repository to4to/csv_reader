
use std::error::Error;

use csv;


fn main() {
if let Err(e) = read_from_file("./SampleCSVFile_11kb.csv"){
    eprintln!("{}",e)
}
}



fn read_from_file(path :&str)->Result<(),Box<dyn Error>>{

let mut reader=csv::Reader::from_path(path);



for result in reader{



}
    Ok(())

}