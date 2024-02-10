use std::error::Error;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{// we take the file as a path, it's reference to a string
    let mut reader = csv::Reader::from_path(path)?; // Instead of using match which is a little more verbose, you can use the '?' which is just a little less verbose

    for result in reader.records(){ // we're working with result and box errors
        let record = result?;// we're creating a record reader with the help of csv reader which will use the path, and it will go through all the records of the csv file

         println!("{:?}", record);
     }
     Ok(())
 }

fn main(){ // the main function will call read_from file
    if let Err(e) = read_from_file("./customers.csv"){
        eprintln!("{}", e); // if there is an error, it will print it here
    }
}    