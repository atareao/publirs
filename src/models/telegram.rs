use reqwest::Client;
use serde_json::json;
use tracing::{info, error};

#[derive(Debug)]
struct Telegram {
    token: String,
}

impl Telegram {
    pub fn new(token: &str) -> Self{
        Self {
            token: token.to_string(),
        }
    }

    pub async fn send_message(&self, chat_id: &str, thread_id: i64, message: &str) -> Result<String, String>{
        let url = format!("https://api.telegram.org/bot{}/sendMessage",
            self.token);
        let message = json!({
            "chat_id": chat_id,
            "message_thread_id": thread_id,
            "text": message,
            "parse_mode": "HTML",
        });
        match Client::new()
            .post(url)
            .json(&message)
            .send()
            .await{
                Ok(response) => {
                    info!("Mensaje envíado a Telegram: {}",
                        response.status().to_string());
                    Ok(response.status().to_string())
                },
                Err(error) => {
                    error!("No he podido enviar el mensaje a Telegram: {}",
                        error.to_string());
                    Err(error.to_string())
                },
            }
    }

    pub async fn send_poll(&self, chat_id: &str, thread_id: i64, question: &str, options: Vec<&str>, correct_option_id: i64) -> Result<String, String>{
        let url = format!("https://api.telegram.org/bot{}/sendPoll",
            self.token);
        let message = json!({
            "chat_id": chat_id,
            "message_thread_id": thread_id,
            "question": question,
            "options": options,
            "is_anonymous": true,
            "type": "quiz",
            "allows_ultiple_answers": false,
            "correct_option_id": correct_option_id,
            "parse_mode": "HTML",
        });
        match Client::new()
            .post(url)
            .json(&message)
            .send()
            .await{
                Ok(response) => {
                    info!("Mensaje envíado a Telegram: {}",
                        response.status().to_string());
                    Ok(response.status().to_string())
                },
                Err(error) => {
                    error!("No he podido enviar el mensaje a Telegram: {}",
                        error.to_string());
                    Err(error.to_string())
                },
            }
    }
}
