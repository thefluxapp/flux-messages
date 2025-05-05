use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct StreamsSettings {
    pub messaging: MessagingSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingSettings {
    pub name: String,
    pub consumer: String,
    pub subjects: MessagingSubjectsSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingSubjectsSettings {
    pub response: String,
}
