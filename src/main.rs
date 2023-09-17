// pub mod vega {
//     include!(concat!(env!("OUT_DIR"), "/vega.rs"));
//     pub mod commands {
//         include!(concat!(env!("OUT_DIR"), "/vega.commands.v1.rs"));
//     }
//     pub mod data {
//         include!(concat!(env!("OUT_DIR"), "/vega.data.v1.rs"));
//     }
//     pub mod events {
//         include!(concat!(env!("OUT_DIR"), "/vega.events.v1.rs"));
//     }
// }

// use vega::events;

// fn next_event<T: Read>(reader: &mut T) -> Result<events::BusEvent> {
//     let mut size_arr = [0u8; 4];
//     reader.read_exact(&mut size_arr)?;
//     let size = BigEndian::read_u32(&size_arr);

//     let mut msg_vec = vec![0u8; size.try_into().unwrap()];
//     reader.read_exact(&mut msg_vec)?;

//     let be = events::BusEvent::parse_from_bytes(&msg_vec)?;
//     Ok(be)
// }

fn main() {
    println!("Hello, world!");
}
