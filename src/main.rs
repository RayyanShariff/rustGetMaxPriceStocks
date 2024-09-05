use std::error::Error;
use std::fs::File;
use csv::Reader;
use csv::StringRecord;

// fn main() {
//     println!("Hello, world!");
// }
// use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open("NVDA.csv")?;
    let mut reader = Reader::from_reader(file);

    let mut max_price = 0.0;
    let mut max_price_row = StringRecord::new();
    // Iterate over each record in the CSV file
    for result in reader.records() {
        // Unwrap the record
        let record = result?;
        // Get the price value from the record
        let price: f64 = record.get(2).unwrap().parse()?;
        // Update the max_price if the current price is greater
        if price > max_price {
            max_price = price;
            max_price_row = record;
        }
        // Process the record
        // println!("{:?}", record);
        
    }

    println!("{:?}", max_price_row);

    Ok(())
}
