use std::fmt::format;
use std::sync::Arc;
use dotenvy::dotenv;
use crate::app::config::Config;
use crate::app::handlers::Handlers;
use crate::app::repositories::Repositories;
use crate::app::services::Services;
use crate::servers::http::server::run_http_server;
use crate::utils::db::init_primary_db;
use teloxide::{dispatching::dialogue::InMemStorage,utils::command::BotCommands, prelude::*};
mod app;
mod domain;
mod feature;
mod servers;
mod utils;
mod bot;

#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;

#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;
use sea_query::Iden;
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::types::{BotCommand, BotCommandScope};
use url::Url;
use crate::feature::url::service::UrlServiceTrait;
use crate::utils::url::{extract_first_valid_url_from_message};

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;
type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Start")]
    Start,
    #[command(description = "All urls")]
    AllUrls,
    #[command(description = "help command")]
    Help,
}
#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullUrl,

}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_env();
    log::info!("Starting throw dice bot...");
    let bot = Bot::new(
        std::env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN env var not set"),
    );
    let config = Config::new().await;
    let http_server = config.server.clone().unwrap_or_else(|| {
        panic!("HTTP server configuration not found");
    });

    let pool = init_primary_db(&config).await.expect("Count not init db");
    let repo = Arc::new(Repositories::new(pool));
    let services = Arc::new(Services::new(repo));
    let handlers = Arc::new(Handlers::new(services.clone()));
    let http_task = async  {
        run_http_server(&http_server.host, http_server.port, handlers).await;
    };
    set_bot_commands(&bot).await.expect("Failed to set commands");

    let bot_task = async  {
        Dispatcher::builder(
            bot,
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .branch(dptree::case![State::Start].endpoint(start))
                .branch(dptree::case![State::ReceiveFullUrl].endpoint(receive_full_url))
        )
            .dependencies(dptree::deps![InMemStorage::<State>::new(),Arc::clone(&services)])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        log::warn!("Bot task ended!");
    };

    tokio::join!(bot_task, http_task);

    Ok(())
}
async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    dialogue: MyDialogue,
) -> anyhow::Result<()> {
    match cmd {
        Command::Start => {
            start(bot, dialogue, msg).await.expect("TODO: panic message");
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::AllUrls => {
                bot.send_message(msg.chat.id, format!("Получен URL: {}", "asdsad")).await?;
                dialogue.update(State::Start).await?;
        }
    }
    Ok(())
}
async fn set_bot_commands(bot: &Bot) -> anyhow::Result<()> {
    let commands = Command::bot_commands()
        .into_iter()
        .map(|cmd| BotCommand {
            command: cmd.command,
            description: cmd.description.to_string(),
        })
        .collect::<Vec<_>>();

    bot.set_my_commands(commands).scope(BotCommandScope::Default).await?;
    Ok(())
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    if let Some(txt) = msg.text() {
        if txt != "/start" {
            bot.send_message(msg.chat.id, "Please, write /start").await?;
            return Ok(());
        }
    }
    bot.send_message(msg.chat.id, "Let's start! What's your full url?").await?;
    dialogue.update(State::ReceiveFullUrl).await?;
    Ok(())
}
async fn receive_full_url(bot: Bot, dialogue: MyDialogue, msg: Message, services: Arc<Services>) -> HandlerResult {
    if let Some(valid_url) = extract_first_valid_url_from_message(&msg) {
        if let Some(user) = &msg.from() {
            let user_id = user.id;
            let username = user.username.as_deref().unwrap_or("<no username>");
            println!("valid url from {} ({}): {}", username, user_id, valid_url);
        }

        match services.url_service.create_url(valid_url.to_string()).await {
            Ok(created_url) => {
                bot.send_message(msg.chat.id, format!("✅ Saved url: {:?}", created_url)).await?;
            }
            Err(e) => {
                bot.send_message(msg.chat.id, "❌ Failed to save URL.").await?;
                log::error!("Failed to create url: {:?}", e);
            }
        }
        dialogue
            .update(State::ReceiveFullUrl )
            .await?;
    } else {
        bot.send_message(msg.chat.id, "❌ Не удалось найти корректный URL в сообщении.").await?;
    }
    Ok(())
}

fn init_env() {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
    }

    pretty_env_logger::init();
}