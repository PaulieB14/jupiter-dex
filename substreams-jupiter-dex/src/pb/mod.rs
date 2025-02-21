pub mod sf {
    pub mod solana {
        pub mod r#type {
            pub mod v1 {
                include!("sf.solana.type.v1.rs");
            }
        }
    }
    pub mod substreams {
        pub mod v1 {
            include!("sf.substreams.v1.rs");
        }
    }
}
