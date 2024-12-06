//! Struct models for API requests and responses.

#![allow(missing_docs)]

use serde::Deserialize;
use serde_json::Value;

/// Error struct from Mattermost.
///
/// See [here] for more information.
///
/// [here]: https://api.mattermost.com/#tag/errors
#[derive(Debug, Deserialize)]
pub struct MattermostError {
    pub id: String,
    pub message: String,
    pub request_id: String,
    pub status_code: i16,
    pub is_oauth: bool,
}

/// Response struct from /teams/name/{name}
#[derive(Debug, Deserialize)]
pub struct TeamInformation {
    pub id: String,
    pub create_at: i64,
    pub update_at: i64,
    pub delete_at: i64,
    pub display_name: String,
    pub name: String,
    pub description: String,
    pub email: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub allowed_domains: String,
    pub invite_id: String,
    pub allow_open_invite: bool,
    pub policy_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BoolishFlag {
    True,
    False,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationFlag {
    All,
    Mention,
    None,
}

#[derive(Debug, Deserialize)]
pub struct UserNotifyProps {
    pub email: BoolishFlag,
    pub push: NotificationFlag,
    pub desktop: NotificationFlag,
    pub desktop_sound: BoolishFlag,
    pub mention_keys: String,
    pub channel: BoolishFlag,
    pub first_name: BoolishFlag,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timezone {
    pub use_automatic_timezone: bool,
    pub manual_timezone: String,
    pub automatic_timezone: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub create_at: i64,
    pub update_at: i64,
    pub delete_at: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub email_verified: String,
    pub auth_service: String,
    pub roles: String,
    pub locale: String,
    pub notify_props: UserNotifyProps,
    pub props: Value,
    pub last_password_update: i64,
    pub last_picture_update: i64,
    pub failed_attemps: i64,
    pub mfa_active: bool,
    pub timezone: Timezone,
    pub terms_of_service_id: Option<String>,
    pub terms_of_service_create_at: i64,
}

/// Response struct from /users/{user_id}/teams/unread
#[derive(Debug, Deserialize)]
pub struct TeamsUnreadInformation {
    pub teams_id: String,
    pub msg_count: u64,
    pub mention_count: u64,
}

/// Information about a single channel on the instance.
#[derive(Debug, Deserialize)]
pub struct ChannelInformation {
    //
}
