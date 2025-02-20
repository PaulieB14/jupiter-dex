fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(::prost::Message)]");

    tonic_build::configure()
        .out_dir("src/pb")
        .compile_with_config(
            config,
            &[
                "proto/sf/substreams/v1/entities.proto",
                "proto/sf/solana/type/v1/type.proto",
                "proto/sf/solana/type/v1/block.proto",
            ],
            &["proto"],
        )?;

    println!("cargo:rerun-if-changed=proto");
    Ok(())
}
