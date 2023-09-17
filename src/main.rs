use anyhow::{Context, Result};
use byteorder::{BigEndian, ByteOrder};
// use protobuf::Message;
// use protos::events;
use bytes::{Buf, BytesMut};
use prost::Message;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

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

#[tokio::main]
fn main() {
    let cwd = std::env::current_dir().unwrap()
    let db = Surreal::new::<SpeeDb>(cwd.join("data")).await?;

    let event_file = cwd.join("testdata/eventlog.evt");

    let f = File::open(event_file).unwrap();
    let mut reader = BufReader::new(f);
    while let Ok(be) = next_event(&mut reader) {
        println!("{:?}", be);
    }
    println!("Hello, world!");
}
