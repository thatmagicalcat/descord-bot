use std::collections::HashMap;

use super::BOT_ID;

#[descord::event_handler(reaction_add)]
pub async fn reaction_add(data: ReactionData) {
    if &data.user_id == BOT_ID.lock().await.as_str() {
        return;
    }

    let msg = data.get_message().await;

    if data.emoji.name == "➡" {
        tokio::spawn(async move { data.remove_reaction().await });

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

        msg.edit(MessageEditData {
            embeds: vec![embed].into(),
            ..Default::default()
        })
        .await;
    }
}
