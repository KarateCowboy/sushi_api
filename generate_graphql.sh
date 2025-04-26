sea-orm-cli generate entity -o src/entities -u sqlite://sushi.db --seaography
seaography-cli ./ src/entities sqlite://sushi.db seaography-sqlite-example
cargo run
