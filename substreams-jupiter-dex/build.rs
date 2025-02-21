fn main() {
    let mut config = prost_build::Config::new();
    config.btree_map(["."])
        .bytes(["."])
        .type_attribute(".", "#[derive(::prost::Message)]");

    tonic_build::configure()
        .out_dir("src/pb")
        .compile_with_config(
            config,
            &[
                "proto/sf/solana/type/v1/block.proto",
                "proto/sf/solana/type/v1/type.proto",
                "proto/sf/substreams/v1/entities.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
