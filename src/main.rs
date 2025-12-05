use chrono::DateTime;
use colored::Colorize;
use docx_rs::*;
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
    from: User,
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
    message: Option<MessageDetails>,
    my_chat_member: Option<ChatMember>,
    edited_message: Option<MessageDetails>,
}

#[derive(Debug, Deserialize)]
struct TelegramResponse {
    result: Vec<MessageBody>,
}

async fn get_message(
    msg: &MessageDetails,
    pool: Pool<Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let message_text = msg.text.as_deref().unwrap_or("No Text Available");
    println!(
        "EntryType: {}\nText: {}",
        "message".custom_color((250, 218, 122)),
        message_text.custom_color((201, 104, 104))
    );

    let chat_id = msg.chat.id;
    println!(
        "Channel: {}, {}",
        chat_id.to_string().custom_color((168, 251, 211)),
        msg.chat.title.to_string().custom_color((168, 251, 211))
    );

    let user_id = msg.from.id;
    let mut tx = pool.begin().await?;

    sqlx::query(
    r#"INSERT INTO updates (entry_type, chat_id, user_id, message_text, date_msg) VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (chat_id, user_id, message_text, date_msg) DO NOTHING"#,
    )
    .bind("message")
    .bind(chat_id)
    .bind(user_id)
    .bind(message_text)
    .bind(msg.date)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

async fn fetch_messages() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let bot_token =
        env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN environment variable is not set");
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

    for data in response_data.result.iter() {
        match data {
            MessageBody {
                message: Some(msg), ..
            } => {
                println!("msg    ----- {:?}", data);
                let _ = get_message(msg, pool.clone()).await;
            }
            MessageBody {
                edited_message: Some(msg),
                ..
            } => {
                let _ = get_message(msg, pool.clone()).await;
            }
            MessageBody {
                my_chat_member: Some(chat),
                ..
            } => {
                println!(
                    "EntryType: {}\nChannel: {} - {}",
                    "my_chat_member ".custom_color((240, 168, 208)),
                    chat.chat.id.to_string().custom_color((168, 251, 211)),
                    chat.chat.title.to_string().custom_color((168, 251, 211)),
                );
                let mut tx = pool.begin().await?;

                sqlx::query(
                    r#"INSERT INTO updates (entry_type, chat_id, user_id, message_text, date_msg) VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (chat_id, user_id, message_text, date_msg) DO NOTHING"#,
                )
                .bind("my_chat_member")
                .bind(chat.chat.id)
                .bind("none")
                .bind("none")
                .bind(chat.date)
                .execute(&mut *tx)
                .await?;
                tx.commit().await?;
            }
            _ => {
                println!("Unknown update type");
            }
        }
    }
    Ok(())
}

async fn get_conversations(date_arg: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("Successfully connected to the database!");
    let _ = pool.begin().await?;

    let updates = sqlx::query!(
        r#"
         SELECT
            u.chat_id,
            u.user_id,
            u.message_text,
            u.date_msg,
            c.chat_title as "chat_title?"
        FROM updates u
        LEFT JOIN chats c ON u.chat_id = c.chat_id
        ORDER BY u.chat_id, u.date_msg ASC
        "#
    )
    .fetch_all(&pool)
    .await?;

    for update in updates {
        let timestamp = &update.date_msg.parse::<i64>().unwrap();
        let dt = DateTime::from_timestamp(*timestamp, 0).unwrap();
        let date = dt.format("%Y-%m-%d").to_string();
        let chat_title = update
            .chat_title
            .unwrap_or_else(|| "Unknown Chat".to_string());
        if date == date_arg {
            println!(
                "Chat: {} - {}, Message: {}, Date: {} - {}",
                update.chat_id.custom_color((168, 251, 211)),
                chat_title.custom_color((168, 251, 211)),
                update.message_text.custom_color((201, 104, 104)),
                update.date_msg,
                date
            );
        }
    }
    print_report().await?;
    Ok(())
}

async fn print_report() -> Result<(), DocxError> {
    let path = std::path::Path::new("./report.docx");
    let file = std::fs::File::create(path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(
            Run::new().add_text("Can you summarise all the following interaction and give:"),
        ))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("- Issue Title ")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("- Issue Description")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("- Resolution Summary")))
        .add_paragraph(Paragraph::new().add_run(
            Run::new().add_text("- Suggest a Category where the Issue should Fall under?"),
        ))
        .build()
        .pack(file)?;
    Ok(())
}

async fn print_help() {
    println!(
        "\n{}\n",
        "Telegram Support Analyzer".custom_color((250, 218, 122))
    );
    println!("Options");
    println!(
        " {}: fetches messages from the Telegram chats and saves the new ones in the database.",
        "sync-messages".custom_color((168, 251, 211)).bold()
    );
    println!(
        " {}: retrieves conversations.",
        "get-conversations".custom_color((168, 251, 211)).bold()
    );
    println!(
        " {}: prints this help page.",
        "help".custom_color((168, 251, 211)).bold()
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "sync-messages" => fetch_messages().await?,
        "get-conversations" => get_conversations(args[2].as_str()).await?,
        "help" => print_help().await,
        _ => (),
    }

    Ok(())
}
