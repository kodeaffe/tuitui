use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use thiserror::Error;
use serde::{Deserialize, Serialize};

pub const DB_PATH: &str = "./data/db.json";


#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

pub fn read_db<'a, T>() -> Result<Vec<T>, Error>
where
    for<'b> T: Deserialize<'b>
{
    //let db_content = fs::read_to_string(DB_PATH)?;
    let mut file = OpenOptions::new()
        .read(true)// Can only read_to_string when open for reading
        .write(true) // Can only create when also open for writing
        .create(true)
        .open(DB_PATH)?;
    let mut db_content= String::new();
    file.read_to_string(&mut db_content)?;
    match serde_json::from_str::<Vec<T>>(&db_content) {
        Ok(parsed) => Ok(parsed),
        _ => Ok(Vec::<T>::new()),
    }
}

pub fn write_db<T>(data: &Vec<T>) -> Result<bool, Error>
where
    T: Serialize
{
    let db_content = serde_json::to_vec::<Vec<T>>(data)?;
    fs::write(DB_PATH, &db_content)?;
    Ok(true)
}
