use clap::{builder::Str, Arg, ArgAction, ArgMatches, Command, Subcommand};
use futures::future::join;
use rusqlite::{Connection, Result};
use scryfall::card::Card;
use tokio::runtime::Runtime;

#[derive(Debug)]
struct SCard {
    name: String,
    set_code: String,
    amount: i32,
    price: f32,
    formats: Vec<String>,
}

fn check_for_table(conn: &Connection) -> Result<()> {
    let mut db_check =
        conn.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='card'")?;
    let mut check = db_check.query([])?;

    if let Some(row) = check.next()? {
        let x: i32 = row.get(0)?;
        if x == 0 {
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
    Ok(())
}

fn look_up_card(runtime: &Runtime, card_name: &str, amount: i32) -> scryfall::Card{
    println!("{:?} {:?}", card_name, &amount);
    let card: Card;
    let result = runtime.block_on(Card::named_fuzzy(card_name));
    match result {
        Ok(c) => card = c,
        Err(e) => panic!("{}", format!("{:?}", e)),
    }
    card
}

fn main() -> Result<()> {
    let conn = Connection::open("./card_database")?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    check_for_table(&conn)?;

    let prog = Command::new("card-storage")
        // TODO: Add flag file to to batch work
        .arg(
            Arg::new("file")
                .short('f')
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .subcommand(
            Command::new("add").args([
                Arg::new("card").help("card name"),
                Arg::new("amount")
                    .default_value("1")
                    .help("number of cards default is 1"),
            ]),
        )
        .subcommand(
            Command::new("remove").args([
                Arg::new("card").help("card namem"),
                Arg::new("amount")
                    .default_value("0")
                    .help("amount of cards to remove"),
            ]),
        )
        .get_matches();

    match prog.subcommand() {
        Some(("add", args)) => {
            if !args.get_flag("file") {
                let card_name = args.get_one::<String>("card").unwrap();
                let amount = args.get_one::<String>("amount").unwrap().parse::<i32>().unwrap();
                let card = look_up_card(&runtime, card_name, amount);
                println!("{}", card.prices.usd.unwrap());
            }
        }
        Some(("remove", args)) => {
            println!(
                "{:?} {:?}",
                args.get_one::<String>("card"),
                args.get_one::<String>("amount").unwrap().parse::<i32>()
            )
        }
        _ => println!("No command inputed"),
    }

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
