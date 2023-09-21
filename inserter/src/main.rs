use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use bytes::BytesMut;
use prost::Message;
use protos::vega::events::v1::{bus_event::Event, BusEvent};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

mod ledger;
mod order;
mod trade;

fn next_event<T: Read>(reader: &mut T) -> Result<BusEvent> {
    // Read in the size
    let mut size_arr = [0u8; 4];
    reader.read_exact(&mut size_arr)?;
    let sizeu32 = BigEndian::read_u32(&size_arr);
    let size = usize::try_from(sizeu32).unwrap();

    // Read in the seqnum
    let mut seqnum_arr = [0u8; 8];
    reader.read_exact(&mut seqnum_arr)?;
    let _seqnum = BigEndian::read_u64(&seqnum_arr);

    // Size included 8 bytes for the sequence number
    let mut buf = BytesMut::zeroed(size - 8);
    reader.read_exact(&mut buf)?;

    let be = BusEvent::decode(buf).unwrap();
    Ok(be)
}

#[derive(Debug, Serialize)]
struct Order {
    price: String,
    // dave: Decimal,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    // let db = Surreal::new::<SpeeDb>(cwd.join("data")).await.unwrap();
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();

    db.use_ns("vega").use_db("vega").await.unwrap();

    let schema = include_str!("schema.surql");
    db.query(schema).await?.check()?;

    let event_file = cwd.join("testdata/eventlog.evt");

    let f = File::open(event_file).unwrap();
    let mut reader = BufReader::new(f);
    while let Ok(be) = next_event(&mut reader) {
        if let Some(event) = be.event {
            match event {
                Event::Order(e) => order::insert(db.clone(), e).await?,
                Event::Trade(e) => trade::insert(db.clone(), e).await?,
                Event::LedgerMovements(e) => ledger::insert(db.clone(), e).await?,
                _ => {}
            }
        }
    }
    println!("Hello, world!d");
    Ok(())
}
