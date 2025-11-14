### Overview
A Rust application that monitors Telegram channels (where a designated bot has been added) and retrieves all new messages, currently displaying them in the console.

### In Development
- Save messages to a database
- Reconstruct complete conversation threads
- Leverage an LLM to generate:
  - Conversation summaries
  - Issue titles
  - Issue descriptions
  - Resolution summaries

### Requirements
- Add the Telegram bot as an `admin` to all target channels
- PostgreSQL database running on localhost
- Rust toolchain installed
- Create a `.env` file with required credentials

### Getting Started
- Clone this repository
- Run `cargo run`

### Configuration
Set up your `.env` file with:
- `TELEGRAM_BOT_TOKEN` - Your Telegram bot token
- `DATABASE_URL` - PostgreSQL connection string

