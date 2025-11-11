use colored::Colorize;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::option::Option;

#[derive(Debug, Deserialize)]
struct User {
    id: i64,
}

#[derive(Debug, Deserialize)]
struct MessageDetails {
    text: Option<String>,
    chat: ChatMemberDetails,
    from: Option<User>,
}

#[derive(Debug, Deserialize)]
struct ChatMemberDetails {
    id: i64,
    title: String,
}

#[derive(Debug, Deserialize)]
struct ChatMember {
    chat: ChatMemberDetails,
}

#[derive(Debug, Deserialize)]
struct MessageBody {
    update_id: i32,
    message: Option<MessageDetails>,
    my_chat_member: Option<ChatMember>,
}

#[derive(Debug, Deserialize)]
struct TelegramResponse {
    result: Vec<MessageBody>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let bot_token: &str = api_key.as_str();

    let tg_get_updates = format!("https://api.telegram.org/bot{}/getUpdates", bot_token);

    let client = Client::new();
    let response = client.get(tg_get_updates).send().await?;
    let status = response.status();

    if !status.is_success() {
        return Err(format!("Request failed with status: {}", status).into());
    }

    let response_as_text = response.text().await?;
    let response_data: TelegramResponse = serde_json::from_str(&response_as_text)?;

    let mut i = 1;
    let mut chat_ids = vec![];
    let mut user_ids = vec![];
    for data in response_data.result.iter() {
        let chat_data = data.my_chat_member.as_ref();
        if chat_data.is_some() {
            let chat_details = chat_data.unwrap();
            println!(
                "{}{}\n  Update: {}, Chat_Id: {}, Chat_Title: {}",
                " Message #".custom_color((99, 122, 185)),
                i.to_string().custom_color((99, 122, 185)),
                data.update_id,
                chat_details.chat.id,
                chat_details.chat.title,
            );
        } else {
            let message = data.message.as_ref();
            if message.is_some() {
                let msg_details = message.unwrap();

                let chat_id = msg_details.chat.id;
                if !chat_ids.contains(&chat_id) {
                    println!(
                        "\n\nChannel: {}, {}",
                        chat_id.to_string().custom_color((168, 251, 211)).bold(),
                        msg_details
                            .chat
                            .title
                            .to_string()
                            .custom_color((168, 251, 211))
                            .bold()
                    );
                    chat_ids.push(chat_id);
                }

                let user = msg_details.from.as_ref();
                if user.is_some() {
                    let user_id = user.unwrap().id;
                    if !user_ids.contains(&user_id) {
                        println!(
                            " User:{}",
                            user_id.to_string().custom_color((79, 183, 179)).bold()
                        );
                        chat_ids.push(chat_id);
                        user_ids.push(user_id);
                    }
                }
                println!(
                    "  {}{}\n   {}",
                    " Message #".custom_color((99, 122, 185)),
                    i.to_string().custom_color((99, 122, 185)),
                    msg_details.text.as_deref().unwrap_or("None")
                );
            }
        }
        i += 1;
    }
    println!("\nChats Vec: {:?}", chat_ids);
    println!("Users Vec: {:?}", user_ids);
    Ok(())
}
