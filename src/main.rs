use descord::prelude::*;

mod commands;
mod events;

use commands::*;
use events::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let mut client = Client::new(
        &std::env::var("DISCORD_TOKEN").unwrap(),
        GatewayIntent::ALL,
        "!",
    )
    .await;

    client.register_commands(vec![ping(), meme()]);
    client.register_events(vec![ready(), reaction_add()]);

    client.login().await;
}
