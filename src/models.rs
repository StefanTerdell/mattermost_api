//! Struct models for API requests and responses.

#![allow(missing_docs)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Error struct from Mattermost.
///
/// See [here] for more information.
///
/// [here]: https://api.mattermost.com/#tag/errors
#[derive(Debug, Serialize, Deserialize)]
pub struct MattermostError {
    pub id: String,
    pub message: String,
    pub request_id: String,
    pub status_code: i16,
    pub is_oauth: Option<bool>,
    pub detailed_error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub create_at: usize,
    pub update_at: usize,
    pub delete_at: usize,
    pub edit_at: usize,
    pub user_id: String,
    pub channel_id: String,
    pub root_id: String,
    pub original_id: String,
    pub message: String,
    #[serde(rename = "type")]
    pub post_type: String,
    pub props: Value,
    pub hashtag: Option<String>,
    pub file_ids: Option<Vec<String>>,
    pub pending_post_id: String,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub embeds: Option<Vec<Embed>>,
    pub emojis: Option<Vec<Emoji>>,
    pub files: Option<Vec<FileMetadata>>,
    pub images: Option<Value>,
    pub reactions: Option<Vec<Reaction>>,
    pub priority: Option<Priority>,
    pub acknowledgements: Option<Vec<Acknowledgement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    #[serde(rename = "type")]
    pub embed_type: String,
    pub url: Option<String>,
    pub data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emoji {
    pub id: String,
    pub creator_id: String,
    pub name: String,
    pub create_at: usize,
    pub update_at: usize,
    pub delete_at: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: String,
    pub user_id: String,
    pub post_id: String,
    pub create_at: usize,
    pub update_at: usize,
    pub delete_at: usize,
    pub name: String,
    pub extension: String,
    pub size: usize,
    pub mime_type: String,
    pub width: usize,
    pub height: usize,
    pub has_preview_image: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub user_id: String,
    pub post_id: String,
    pub emoji_name: String,
    pub create_at: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum PriorityLevel {
    #[serde(rename = "")]
    None,
    Urgent,
    Important,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Priority {
    pub priority: PriorityLevel,
    pub requested_ack: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acknowledgement {
    pub user_id: String,
    pub post_id: String,
    pub acknowledged_at: usize,
}

/// Thread
#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    pub order: Vec<String>,
    pub posts: HashMap<String, Post>,
    pub next_post_id: String,
    pub prev_post_id: String,
    pub has_next: bool,
}

/// Response struct from /teams/name/{name}
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BoolishFlag {
    True,
    False,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationFlag {
    All,
    Mention,
    None,
}

#[derive(Debug, Serialize)]
pub struct CreatePostMetadata {
    pub priority: Option<Priority>,
}

#[derive(Default, Debug, Deserialize)]
pub struct DeleteReactionResponse {
    pub status: String,
}

#[derive(Default, Debug, Serialize)]
pub struct CreatePost {
    pub channel_id: String,
    pub message: String,
    pub root_id: Option<String>,
    pub props: Option<Value>,
    pub hashtag: Option<String>,
    pub file_ids: Option<Vec<String>>,
    pub metadata: Option<CreatePostMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserNotifyProps {
    pub email: Option<BoolishFlag>,
    pub push: Option<NotificationFlag>,
    pub desktop: Option<NotificationFlag>,
    pub desktop_sound: Option<BoolishFlag>,
    pub mention_keys: Option<String>,
    pub channel: Option<BoolishFlag>,
    pub first_name: Option<BoolishFlag>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timezone {
    pub use_automatic_timezone: String,
    pub manual_timezone: String,
    pub automatic_timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub create_at: i64,
    pub update_at: i64,
    pub delete_at: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub email_verified: Option<String>,
    pub auth_service: String,
    pub roles: String,
    pub locale: String,
    pub notify_props: Option<UserNotifyProps>,
    pub props: Option<Value>,
    pub last_password_update: Option<i64>,
    pub last_picture_update: Option<i64>,
    pub failed_attemps: Option<i64>,
    pub mfa_active: Option<bool>,
    pub timezone: Timezone,
    pub terms_of_service_id: Option<String>,
    pub terms_of_service_create_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileUploadResponse {
    pub file_infos: Vec<FileMetadata>,
    pub client_ids: Vec<String>,
}

/// Response struct from /users/{user_id}/teams/unread
#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsUnreadInformation {
    pub teams_id: String,
    pub msg_count: u64,
    pub mention_count: u64,
}

/// Information about a single channel on the instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInformation {
    //
}
