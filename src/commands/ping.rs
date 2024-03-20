use std::time::Instant;

#[descord::command]
pub async fn ping(data: MessageData) {
    let clock = Instant::now();
    let msg = data.reply("Pong!").await;

    msg.edit(format!(
        "Pong! :ping_pong: `{}ms`",
        clock.elapsed().as_millis()
    )).await;
}
