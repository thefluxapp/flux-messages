use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct StreamsSettings {
    pub messaging: MessagingSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingSettings {
    pub subjects: MessagingSubjectsSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessagingSubjectsSettings {
    pub request: String,
    pub response: String,
}
