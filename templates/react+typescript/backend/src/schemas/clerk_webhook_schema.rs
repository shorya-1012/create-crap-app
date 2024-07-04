use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClerkWebHookEvent {
    #[serde(rename = "type")]
    pub webhook_type: String,
    pub object: String,
    pub data: ClerkUserData,
}

#[derive(Deserialize)]
pub struct ClerkUserData {
    pub id: String,
    pub last_name: String,
    pub first_name: String,
    pub email_addresses: Vec<ClerkEmailAddress>,
    pub image_url: String,
}

#[derive(Deserialize)]
pub struct ClerkEmailAddress {
    pub email_address: String,
}
