use sonnerie::*;
use std::path::Path;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DatabaseReader::new(Path::new("./target/database"))?;
    let a = db.get_range("1"..);

    for record in a {
        println!("{:?}", &record);
    }

    Ok(())
}
