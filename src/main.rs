
use std::error::Error




fn main() {
if let Err(e) = read_from_file("./SampleCSVFile_11kb.csv"){
    eprintln!("{}",e)
}
}






fn read_from_file(path :&String){

}