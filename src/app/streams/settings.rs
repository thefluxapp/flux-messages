use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct StreamsSettings {
    pub messaging: MessagingSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingSettings {
    pub stream: MessagingStreamSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingStreamSettings {
    pub subjects: Vec<String>,
    pub consumer: String,
}
