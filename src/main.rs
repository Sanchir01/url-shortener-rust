use std::fmt::format;
use std::sync::Arc;
use dotenvy::dotenv;
use crate::app::config::Config;
use crate::app::handlers::Handlers;
use crate::app::repositories::Repositories;
use crate::app::services::Services;
use crate::servers::http::server::run_http_server;
use crate::utils::db::init_primary_db;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
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
use crate::feature::url::service::UrlServiceTrait;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullUrl,
    ReceiveAge {
        full_name: String,
    },
   SaveUrl {
       full_url: String,
   }
}
#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;
type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("dotenv init failed");
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    let bot = Bot::from_env();
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
async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! What's your full url?").await?;
    dialogue.update(State::ReceiveFullUrl).await?;
    Ok(())
}
async fn receive_full_url(bot: Bot, dialogue: MyDialogue, msg: Message, services: Arc<Services>) -> HandlerResult {
    match msg.text() {
        Some(full_url) => {
            bot.send_message(msg.chat.id, format!("I am {}!", full_url)).await?;
            let urls =services.url_service.get_all_url().await?;
            bot.send_message(msg.chat.id, format!("all urls: {:?}",urls)).await?;
            dialogue.update(State::SaveUrl { full_url: full_url.to_string() }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }
    Ok(())
}





