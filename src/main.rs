#![warn(unused)]
use rusqlite::{Connection, Result};


#[derive(Debug)]
struct Category {
    _id: i16,
    collage_name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let connect = Connection::open_in_memory()?;

    connect.execute(
        "CREATE TABLE category (
        id  INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;


    let selector = Category {
        _id: 0,
        collage_name: "Georgia Tech".to_string(),
        data: None,
    };
    let selector_2 = Category {
        _id: 2,
        collage_name: "Massachusetts Institute of Technology".to_string(),
        data: None,
    };

    connect.execute(
        "INSERT INTO category (name, data) VALUES (?1, ?2)",
        (&selector.collage_name, &selector.data),

    )?;
    connect.execute(
        "INSERT INTO category (name, data) VALUES (?1, ?2)",
        (&selector_2.collage_name, &selector_2.data),
    )?;

    let mut stmt = connect.prepare("SELECT id, name, data FROM Category")?;
    let category_iter = stmt.query_map([], |row| {
        Ok(Category {
            _id: row.get(0)?,
            collage_name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for category in category_iter {
        println!("Found collage {:?}", category?);
    }
    Ok(())
}
