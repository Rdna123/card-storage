use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Card {
    name: String,
    set_code: String,
    amount: i32,
    price: f32,
    formats: Vec<String>,
}

fn main() -> Result<()> {
    let conn = Connection::open("./card_database")?;

    let mut db_check = conn.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='card'")?;
    let mut check = db_check.query([])?;

    if let Some(row) = check.next()? {
        let x:i32 = row.get(0)?;
        if x == 0{
        conn.execute(
            "CREATE TABLE card (
            name TEXT NOT NULL PRIMARY KEY,
            set_code NOT NULL,
            amount INTEGAR NOT NULL,
            prince FLOAT,
            formats BLOB
            )",
         (),
        )?;
        //println!("created table");
        }
    }
    //println!("done");

    

    // conn.execute(
    //     "CREATE TABLE person (
    //         id   INTEGER PRIMARY KEY,
    //         name TEXT NOT NULL,
    //         data BLOB
    //     )",
    //     (), // empty list of parameters.
    // )?;
    // let me = Person {
    //     id: 2,
    //     name: "Steaven".to_string(),
    //     data: Some(vec![1,22,5,2,3,5,1]),
    // };
    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     (&me.name, &me.data),
    // )?;

    // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;

    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }
    Ok(())
}