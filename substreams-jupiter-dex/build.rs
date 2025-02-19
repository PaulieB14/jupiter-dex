fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(
            &[
                "proto/sf/substreams/v1/entities.proto",
                "proto/sf/solana/accounts/v1/accounts.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
