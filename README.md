cargo add diesel dotenv actix --features "diesel/postgres diesel/r2d2 diesel/chrono"
cargo install diesel_cli --no-default-features --features postgres
cargo install diesel_cli_ext
diesel setup
rm -r migrations
$PSDefaultParameterValues['Out-File:Encoding'] = 'utf8'
diesel print-schema > src/schema.rs
diesel_ext > src/db_models.rs
diesel-rs/diesel#2947 (comment)
