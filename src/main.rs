use colored::Colorize;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
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
    date: i64,
}

#[derive(Debug, Deserialize)]
struct ChatMemberDetails {
    id: i64,
    title: String,
}

#[derive(Debug, Deserialize)]
struct ChatMember {
    chat: ChatMemberDetails,
    date: i64,
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
    let bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("API_KEY must be set");
    let bot_token_string: &str = bot_token.as_str();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("Successfully connected to the database!");

    let tg_get_updates = format!(
        "https://api.telegram.org/bot{}/getUpdates",
        bot_token_string
    );

    let client = Client::new();
    let response = client.get(tg_get_updates).send().await?;
    let status = response.status();

    if !status.is_success() {
        return Err(format!("Request failed with status: {}", status).into());
    }

    let response_as_text = response.text().await?;
    let response_data: TelegramResponse = serde_json::from_str(&response_as_text)?;

    for (index, data) in response_data.result.iter().enumerate() {
        let my_chat_member = data.my_chat_member.as_ref();
        println!("\n#{}\nUpdateID: {}", index, data.update_id);
        if my_chat_member.is_some() {
            let chat_details = my_chat_member.unwrap();
            println!(
                "EntryType: {}\nChannel: {} - {}",
                "my_chat_member ".custom_color((240, 168, 208)),
                chat_details
                    .chat
                    .id
                    .to_string()
                    .custom_color((168, 251, 211)),
                chat_details
                    .chat
                    .title
                    .to_string()
                    .custom_color((168, 251, 211)),
            );
            let mut tx = pool.begin().await?;

            sqlx::query(
                r#"INSERT INTO updates (entry_type, chat_id, user_id, message_text, date) VALUES ($1, $2, $3, $4, $5)"#,
            )
            .bind("my_chat_member")
            .bind(chat_details.chat.id)
            .bind("none")
            .bind("none")
            .bind(chat_details.date)
            .execute(&mut *tx)
            .await?;
            tx.commit().await?;
        } else {
            let message = data.message.as_ref();
            let message_data = message.unwrap();

            if message.is_some() {
                let message_text = message_data.text.as_deref().unwrap_or("No Text Available");
                println!(
                    "EntryType: {}\nText: {}",
                    "message".custom_color((250, 218, 122)),
                    message_text.custom_color((201, 104, 104))
                );

                let chat_id = message_data.chat.id;
                println!(
                    "Channel: {}, {}",
                    chat_id.to_string().custom_color((168, 251, 211)),
                    message_data
                        .chat
                        .title
                        .to_string()
                        .custom_color((168, 251, 211))
                );

                let user = message_data.from.as_ref();
                if user.is_some() {
                    let user_id = user.unwrap().id;
                    println!("User: {}", user_id.to_string().custom_color((79, 183, 179)));
                }
                let mut tx = pool.begin().await?;

                sqlx::query(
                r#"INSERT INTO updates (entry_type, chat_id, user_id, message_text, date) VALUES ($1, $2, $3, $4, $5)"#,
            )
            .bind("message")
            .bind(chat_id)
            .bind("None")
            .bind(message_text)
            .bind(message_data.date)
            .execute(&mut *tx)
            .await?;
                tx.commit().await?;
            }
        }
    }
    Ok(())
}
