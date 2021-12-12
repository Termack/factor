use fcmp::file_handler::FileHandler;
use std::error;

type Result<T> = std::result::Result<T,Box<dyn error::Error>>;

fn main() -> Result<()>{
    let mut r = FileHandler::new("resources/test/test.txt");
    let num = r.read_int()?;
    r.write_from_int(num)?;
    Ok(())
}