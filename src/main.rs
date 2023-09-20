use anyhow::{Context, Result};
use byteorder::{BigEndian, ByteOrder};
// use protobuf::Message;
// use protos::events;
use bytes::{Buf, BytesMut};
use num::BigInt;
use prost::Message;
// use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::thing;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

pub mod vega {
    include!(concat!(env!("OUT_DIR"), "/vega.rs"));
    pub mod commands {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.commands.v1.rs"));
        }
    }
    pub mod data {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.data.v1.rs"));
        }
    }
    pub mod events {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/vega.events.v1.rs"));
        }
    }
}

use vega::events;

fn next_event<T: Read>(reader: &mut T) -> Result<events::v1::BusEvent> {
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

    let be = events::v1::BusEvent::decode(buf).unwrap();
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

    let event_file = cwd.join("testdata/eventlog.evt");

    let f = File::open(event_file).unwrap();
    let mut reader = BufReader::new(f);
    while let Ok(be) = next_event(&mut reader) {
        if let Some(event) = be.event {
            match event {
                events::v1::bus_event::Event::Order(oe) => {
                    println!("{:?}", &oe);
                    // let id = format!("order:{}", oe.id);
                    let sql = "update $id SET price = type::decimal($price), 
                            size = type::decimal($size),
                            market = $mid,
                            party = $pid;
                        ";
                    // let t = thing("order:aa").unwrap();
                    let oid = Thing::from(("order", Id::String(oe.id)));
                    let pid = Thing::from(("party", Id::String(oe.party_id)));
                    let mid = Thing::from(("market", Id::String(oe.market_id)));
                    let mut result = db
                        .query(sql)
                        .bind(("id", oid))
                        .bind(("price", oe.price))
                        .bind(("size", oe.size))
                        .bind(("pid", pid))
                        .bind(("mid", mid))
                        .await?;
                    let created: Option<Record> = result.take(0)?;
                    // let prel: Option<Record> = result.take(1)?;

                    // let pricedec = Decimal::from_str(&oe.price).unwrap();
                    // let arse = dec!(23423432234324324243243242423);
                    // let arse = BigInt::try_from("12323.232").unwrap();
                    // let record: Option<Record> = db
                    //     .update(("order", oe.id.clone()))
                    //     .merge(Order {
                    //         price: oe.price.clone(),
                    //         // dave: arse,
                    //     })
                    //     .await?;
                }
                _ => {}
            }
        }
    }
    println!("Hello, world!");
    Ok(())
}
