use std::env;

use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};

fn main()-> Result<(), Error> {
    let here = env::current_dir()?;
    let here_string = String::from(here.to_string_lossy());
    let path = format!("{}/2007.xlsx", here_string);
    println!("start: {}", path);
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = workbook.worksheet_range("Sheet1");

    if range.is_ok() {
        let a = range.unwrap();
        let mut iter = RangeDeserializerBuilder::new().from_range(&a)?;

        if let Some(result) = iter.next() {
            let (label, value): (String, f64) = result?;
            println!("label: {}  value: {}", label, value);
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    } else {
        //Error::Msg("Cannot find 'Sheet1'");
        Err(From::from("Cannot find 'Sheet1'"))
    }
}