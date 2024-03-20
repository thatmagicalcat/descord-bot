use super::BOT_ID;

#[descord::event_handler(ready)]
pub async fn ready(data: ReadyData) {
    println!(
        "Logged in as: {}#{}",
        data.user.username, data.user.discriminator
    );

    *BOT_ID.lock().await = data.user.id;
}
