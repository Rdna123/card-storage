use std::{
    collections::{HashMap, HashSet},
    fs,
};

use chrono::NaiveDate;
use clap::{Arg, ArgAction, Command};
// use rusqlite::{Connection, Result};
use scryfall::card::Card;
use sqlx::{types, Connection, Executor, Result, SqliteConnection, SqlitePool};

#[derive(Debug)]
struct CardBase {
    name: String,
    set_code: String,
    price: f32,
    formats: Vec<String>,
}

struct CardUser {
    card: CardBase,
    amount: i32,
}

struct CardCol {
    card: CardBase,
    date: NaiveDate,
}

struct KnownCards {
    cards: HashSet<String>,
}

impl KnownCards {
    fn get(path: &str) -> Self {
        let file = std::fs::File::open(path);
        return Self {
            cards: HashSet::new(),
        };
    }
}

async fn check_for_table(conn: &SqlitePool) -> Result<()> {
    conn.fetch(sqlx::query(
        "CREATE TABLE cardbase IF NOT EXISTS(
                name TEXT NOT NULL PRIMARY KEY,
                set_code NOT NULL,
                price TEXT,
                formats BLOB
                )",
    ));

    conn.fetch(sqlx::query(
        "CREATE TABLE cardcol IF NOT EXISTS (
                FOREIGN KEY (card) REFERENCES cardbase,
                date BLOB
            )",
    ));

    conn.fetch(sqlx::query(
        "CREATE TABLE carduser IF NOT EXISTS(
                FOREIGN KEY (card) REFERENCES cardbase,
                amount NOT NULL INT
            )",
    ));
    //println!("created table");

    Ok(())
}

async fn look_up_card(card_name: String, amount: i32, conn: &SqlitePool) -> scryfall::Card {
    //TODO: Add fuzz search of cardcol
    println!("{:?} {:?}", card_name, &amount);
    let card: Card;
    let result = Card::named_fuzzy(&card_name).await;
    match result {
        Ok(c) => card = c,
        Err(_) => panic!("network error"),
    }
    conn.fetch(
        sqlx::query("INSERT INTO cardbase (name, set_code, price, formats) VALUES ($1,$2,$3,$4)")
            .bind(&card.name.clone())
            .bind(&card.set.to_string().clone())
            .bind(&card.prices.usd.clone())
            .bind("formats"),
    );
    eprintln!("done");
    card
}

//TODO: Add detailed search for cards in multiple sets

struct Batch {
    cards: Vec<Card>,
    amounts: Vec<i32>,
}

async fn batch_lookup(file: String, conn: &SqlitePool) -> Batch {
    let card_list = fs::read_to_string(file).unwrap();
    let cards_list_split = card_list.split("\n");
    println!("{:#?}", cards_list_split);
    // let mut set = JoinSet::new();
    let mut amounts: Vec<i32> = vec![];
    let mut cards = vec![];
    for card in cards_list_split {
        let index = shlex::split(card).unwrap();
        let mut iter_index = index.iter();
        let name = iter_index.next().unwrap().to_owned();
        let amount = match iter_index.next() {
            Some(a) => a.parse().unwrap(),
            None => {
                println!("No amount of {name} was give assigning amount to 1");
                1
            }
        };
        amounts.push(amount);
        println!("card");
        let c = look_up_card(name.clone(), amount, conn).await;
        cards.push(c);
    }
    amounts.reverse();
    return Batch { cards, amounts };
}

#[tokio::main]
async fn main() -> Result<()> {
    // Known card database
    let mut card_conn = SqlitePool::connect("./card_database").await?;
    // User's cards

    check_for_table(&card_conn).await?;

    let prog = Command::new("card-storage")
        // TODO: Turn sub commands into flags
        .arg(
            Arg::new("file")
                .short('f')
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .subcommand(
            Command::new("add").args([
                Arg::new("card").help("card name or file if file flag"),
                Arg::new("amount")
                    .default_value("1")
                    .help("number of cards default is 1"),
            ]),
        )
        .subcommand(
            Command::new("remove").args([
                Arg::new("card").help("card name or file if file flag"),
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
                let amount = args
                    .get_one::<String>("amount")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                let card = look_up_card(card_name.to_string(), amount, &mut card_conn).await;
                println!("{}", card.prices.usd.unwrap());
            } else {
                let file = args.get_one::<String>("card").unwrap().to_owned();
                let batch = batch_lookup(file, &mut card_conn).await;
                let mut amounts_iter = batch.amounts.iter();
                for card_result in batch.cards {
                    let card = card_result;
                    println!(
                        "{} - {} - {} - {}",
                        card.name,
                        card.set_name,
                        card.prices.usd.unwrap(),
                        amounts_iter.next().unwrap()
                    );
                }
            }
            // TODO: Database lookup and entry
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
