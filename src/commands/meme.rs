use std::collections::HashMap;

#[descord::command]
pub async fn meme(data: MessageData) {
    let resp = reqwest::get("https://api.popcat.xyz/meme")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap();

    let image = &resp["image"];
    let title = &resp["title"];
    let embed = EmbedBuilder::new()
        .title(&title)
        .color(Color::Orange)
        .image(EmbedImage {
            url: image.to_string(),
            proxy_url: None,
            height: None,
            width: None,
        })
        .build();

    let msg = data.reply(CreateMessageData {
        embeds: vec![embed],
        ..Default::default()
    })
    .await;

    msg.react("➡").await;
}
