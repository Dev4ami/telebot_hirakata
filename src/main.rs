mod message;
mod api;

use telebot_hirakata::*;
use crate::message::*;
use crate::api::*;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::ParseMode;



#[tokio::main]
async fn main() {
    check_folder("cache");
    check_setting("setting.json");
    let token = get_token();
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::new(token);
    println!("\n\nBOT RUNNING\n");
//    Command::repl(bot, answer).await;

    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(answer))
        .branch(dptree::endpoint(handle_non_command_message));
        
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    Start,
    Help,
    Kanji,
    Hiragana,
    Katakana,
    Start_hiragana_moji,    
    Start_katakana_moji,        
    Start_hiragana_kotoba,
    Start_kanji_angka,
    Start_kanji_waktu,
    Add(String),
    
    #[command(description = "", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, message_help())
                               .parse_mode(ParseMode::Html)
                               .await?,
        Command::Kanji => bot.send_message(msg.chat.id, message_kanji())
                               .parse_mode(ParseMode::Html)
                               .await?,                       
        Command::Hiragana => bot.send_message(msg.chat.id, message_hiragana())
                               .parse_mode(ParseMode::Html)
                               .await?,                       
        Command::Katakana => bot.send_message(msg.chat.id, message_katakana())
                               .parse_mode(ParseMode::Html)
                               .await?,                       
        Command::Start => {
            if let Some(user) = msg.from() {
                let user_id = user.id.0;
                let username = user.username.as_deref().unwrap_or("<no username>");
                let text = message_start(username, user_id);
                bot.send_message(msg.chat.id, text)
                   .parse_mode(ParseMode::Html)
                   .await?
            } else {
                bot.send_message(msg.chat.id, "Tidak dapat mengambil info user.").await?
            }
        },
        Command::Start_hiragana_moji => {
            if let Some(user) = msg.from() {
                let user_id = user.id.0;
                let generate = generate_hiragana_moji();
                check_user(&format!("cache/{}.json", user_id), "hiragana_moji", &generate.0, &generate.1, &generate.2, &generate.3);
                bot.send_message(msg.chat.id, message_request_generate_moji(&generate.0))
                    .parse_mode(ParseMode::Html)
                    .await?
            } else {
                bot.send_message(msg.chat.id, "Tidak dapat mengambil info user.").await?
            }
        },
        Command::Start_katakana_moji => {
            if let Some(user) = msg.from() {
                let user_id = user.id.0;
                let generate = generate_katakana_moji();
                check_user(&format!("cache/{}.json", user_id), "katakana_moji", &generate.0, &generate.1, &generate.2, &generate.3);
                bot.send_message(msg.chat.id, message_request_generate_moji(&generate.0))
                    .parse_mode(ParseMode::Html)
                    .await?
            } else {
                bot.send_message(msg.chat.id, "Tidak dapat mengambil info user.").await?
            }
        },        
        Command::Start_hiragana_kotoba => {
            if let Some(user) = msg.from() {
                let user_id = user.id.0;
                let generate = generate_hiragana_kotoba();
                check_user(&format!("cache/{}.json", user_id), "hiragana_kotoba", &generate.0, &generate.1, &generate.2, &generate.3);
                bot.send_message(msg.chat.id, message_request_generate_kotoba(&generate.0, &generate.2, &generate.3))
                    .parse_mode(ParseMode::Html)
                    .await?
            } else {
                bot.send_message(msg.chat.id, "Tidak dapat mengambil info user.").await?
            }
        },
        Command::Add(japan) => {
            let message = if get_super_admin() != true {
                message_admin_only()
            } else if &japan == "" {
                message_invalid_input()
            } else if check_duplicate_data(&japan, "kotoba.json") != true {
                message_duplicate_data()
            } else {
                let indonesia = translate_japan_to_indonesia(&japan).await.to_lowercase();
                let english = translate_japan_to_english(&japan).await.to_lowercase();
                let add = add_list_kotoba(&japan, &indonesia, &english);             
                message_add(&japan, &add, &indonesia, &english)
            };
            bot.send_message(msg.chat.id, message)
                .parse_mode(ParseMode::Html)
                .await?
        }
        Command::Start_kanji_angka => {
            if let Some(user) = msg.from() {
                let user_id = user.id.0;
                let (kanji, romaji, furigana, indonesia, english) = generate_kanji_angka();
                let answer = &format!("{}/{}/{}", romaji, indonesia, english);
                check_user(&format!("cache/{}.json", user_id), "kanji_angka", &kanji, answer, &indonesia, &english);
                bot.send_message(msg.chat.id, message_request_generate_kanji(&kanji, &furigana))
                    .parse_mode(ParseMode::Html)
                    .await?
            } else {
                bot.send_message(msg.chat.id, "Tidak dapat mengambil info user.").await?
            }
        },
        Command::Start_kanji_waktu => {
            if let Some(user) = msg.from() {
                let user_id = user.id.0;
                let (kanji, romaji, furigana, indonesia, english) = generate_kanji_waktu();
                let answer = &format!("{}/{}/{}", romaji, indonesia, english);
                check_user(&format!("cache/{}.json", user_id), "kanji_waktu", &kanji, answer, &indonesia, &english);
                bot.send_message(msg.chat.id, message_request_generate_kanji(&kanji, &furigana))
                    .parse_mode(ParseMode::Html)
                    .await?
            } else {
                bot.send_message(msg.chat.id, "Tidak dapat mengambil info user.").await?
            }
        },
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
    };

    Ok(())
}


async fn handle_non_command_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let answer = msg.text().unwrap().to_lowercase();
    if let Some(user) = msg.from() {
        let user_id = user.id.0;
        let chat_id_user = msg.chat.id;
        let message_id_user = msg.id;        
        let validate = validate_answer(user_id, answer);
        let (status, response) = if validate.0 == true {
            let generate = match validate.1.as_str() {
                "hiragana_moji" => generate_hiragana_moji(), 
                "katakana_moji" => generate_katakana_moji(),
                "hiragana_kotoba" => generate_hiragana_kotoba(),
                "katakana_kotoba" => generate_hiragana_moji(),
                "kanji_angka" => generate_kanji_angka(),
                "kanji_waktu" => generate_kanji_waktu(),
                _ => generate_hiragana_moji(),
            };
            let message = if validate.1 == "hiragana_moji" || validate.1 == "katakana_moji" {
                check_user(&format!("cache/{}.json", user_id), &validate.1, &generate.0, &generate.1, &generate.2, &generate.3);
                format!("{}{}", message_correct(), message_request_generate_moji(&generate.0))
            } else if validate.1 == "kanji_angka" || validate.1 == "kanji_waktu"{
                let answer = &format!("{}/{}/{}", &generate.1, &generate.3, &generate.4);            
                check_user(&format!("cache/{}.json", user_id), &validate.1, &generate.0, answer, &generate.3, &generate.4);
                format!("{}{}", message_correct(), message_request_generate_kanji(&generate.0, &generate.2))
            } else {
                check_user(&format!("cache/{}.json", user_id), &validate.1, &generate.0, &generate.1, &generate.2, &generate.3);
                format!("{}{}", message_correct(), message_request_generate_kotoba(&generate.0, &generate.2, &generate.3))
            };
            (true, message)
        } else {
            (false, message_wrong())
        };
        if status {
            let message_id_bot = get_log_message_id(user_id);
            bot.delete_message(chat_id_user, message_id_bot).await.ok();
            bot.delete_message(chat_id_user, message_id_user).await?;
            let send = bot.send_message(msg.chat.id, response)
                .parse_mode(ParseMode::Html)
                .await?;
            let message_id = send.id.to_string();
            update_message_id(user_id, &message_id);
        } else {
            bot.send_message(msg.chat.id, response)
                .parse_mode(ParseMode::Html)
                .await?;
        }
    } else {
        bot.send_message(msg.chat.id, "Tidak dapat mengambil info user.").await?;
    }
    Ok(())
}