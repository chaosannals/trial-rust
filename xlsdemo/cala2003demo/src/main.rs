use std::env;

use calamine::{open_workbook, Error, Xls, Reader};

fn main() -> Result<(), Error> {
    let here = env::current_dir()?;
    let here_string = String::from(here.to_string_lossy());
    let path = format!("{}/2003.xls", here_string);
    println!("1997-2003 xls: {}", path);

    let mut workbook: Xls<_> = open_workbook(path)?;
    let range = workbook.worksheet_range("Sheet1")?;

    for row in range.rows() {
        println!("row={:?}, row[0]={:?}", row, row[0]);
    }

    Ok(())
}
