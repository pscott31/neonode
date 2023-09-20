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
