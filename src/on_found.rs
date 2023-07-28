use std::env;

use bitcoin::PrivateKey;

pub(crate) type OnFound = dyn Fn(u128, PrivateKey, String);

pub fn on_found(_value: u128, _private_key: PrivateKey, _address: String) {
    send_message_to_telegram(&format!("{}", _private_key));
}

fn send_message_to_telegram(message: &str) {
    let telegram_setup = env::var("TELEGRAM_TOKEN").is_ok() && env::var("TELEGRAM_CHAT_ID").is_ok();
    if telegram_setup {
        let telegram_token = env::var("TELEGRAM_TOKEN").unwrap();
        let telegram_chat_id = env::var("TELEGRAM_CHAT_ID").unwrap();

        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}&parse_mode=Markdown",
            telegram_token, telegram_chat_id, message
        );
        let _ = client.get(&url).send();
    } else {
        //print out message to console
        println!("{}", message);
    }
}
