use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use bytes::BytesMut;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

use protos::vega::events;

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

    let schema = include_str!("schema.surql");
    db.query(schema).await?.check()?;

    let event_file = cwd.join("testdata/eventlog.evt");

    let f = File::open(event_file).unwrap();
    let mut reader = BufReader::new(f);
    while let Ok(be) = next_event(&mut reader) {
        if let Some(event) = be.event {
            match event {
                events::v1::bus_event::Event::Order(oe) => {
                    println!("{:?}", &oe);

                    let sql = "update $id SET 
                            market = $mid,
                            party = $pid,
                            side = $side,
                            price = type::decimal($price), 
                            size = type::decimal($size),
                            remaining = type::decimal($remaining),
                            time_in_force = $tif,
                            type = $type,
                            created_at = time::from::micros($created_at),
                            status = $status,
                            expires_at = time::from::micros($expires_at),
                            reference = $reference,
                            reason = $reason,
                            updated_at =  time::from::micros($updated_at),
                            version = $version,
                            batch_id = $batch_id,
                            // pegged_order = $pegged_order,
                            liquidity_provision_id = $liquidity_provision_id,
                            post_only = $post_only,
                            reduce_only = $reduce_only
                            // iceberg_order = $iceberg_order
                            ;
                        ";

                    let result = db
                        .query(sql)
                        .bind(("id", Thing::from(("order", Id::from(&oe.id)))))
                        .bind(("mid", Thing::from(("market", Id::from(&oe.market_id)))))
                        .bind(("pid", Thing::from(("party", Id::from(&oe.party_id)))))
                        .bind(("side", oe.side().as_str_name()))
                        .bind(("price", &oe.price))
                        .bind(("size", &oe.size))
                        .bind(("remaining", &oe.remaining))
                        .bind(("tif", &oe.time_in_force().as_str_name()))
                        .bind(("type", oe.r#type().as_str_name()))
                        .bind(("created_at", oe.created_at / 1000))
                        .bind(("status", oe.status().as_str_name()))
                        .bind(("expires_at", oe.expires_at / 1000))
                        .bind(("reference", &oe.reference))
                        .bind(("reason", oe.reason().as_str_name()))
                        .bind(("updated_at", oe.updated_at / 1000))
                        .bind(("version", oe.version))
                        .bind(("batch_id", oe.batch_id))
                        // .bind(("pegged_order", &oe.pegged_order))
                        .bind(("liquidity_provision_id", &oe.liquidity_provision_id)) // TODO linky
                        .bind(("post_only", oe.post_only))
                        .bind(("reduce_only", oe.reduce_only))
                        // .bind(("iceberg_order", oe.iceberg_order))
                        .await?;
                    result.check()?;
                }
                _ => {}
            }
        }
    }
    println!("Hello, world!d");
    Ok(())
}
