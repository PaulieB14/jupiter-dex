pub mod sf {
    pub mod substreams {
        pub mod v1 {
            include!("sf.substreams.v1.rs");
        }
    }
    pub mod solana {
        pub mod accounts {
            pub mod v1 {
                include!("sf.solana.accounts.v1.rs");
            }
        }
    }
}
