use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessagesSettings {
    pub limit: u64,
    pub messaging: MessagingSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingSettings {
    pub message: MessagingMessageSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingMessageSettings {
    pub subject: String,
}
