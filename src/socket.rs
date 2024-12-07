//! Websocket client and trait for interacting with the websocket API.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

use crate::models::Post;

/// Websocket event broadcast information
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketEventBroadcast {
    /// Users who were omitted from receiving the event
    pub omit_users: Option<HashMap<String, bool>>,
    /// Event recipient
    pub user_id: Option<String>,
    #[allow(missing_docs)]
    pub channel_id: String,
    #[allow(missing_docs)]
    pub team_id: String,
}

/// Event data from the websocket API
#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketEvent {
    /// Event type and data
    #[serde(flatten)]
    pub event: WebsocketEventType,
    /// Event recipient information
    pub broadcast: WebsocketEventBroadcast,
    /// Sequence number
    pub seq: usize,
}

/// Handler trait for receiving websocket messages.
///
/// Implement on a struct you create, and pass to
/// `connect_to_websocket` to connect to your
/// Mattermost instance's websocket API.
///
/// # Example
///
/// ```rust,no_run
/// use async_trait::async_trait;
/// use mattermost_api::prelude::*;
///
/// struct Handler {}
///
/// #[async_trait]
/// impl WebsocketHandler for Handler {
///     async fn callback(&self, message: WebsocketEvent) {
///         println!("{:?}", message);
///     }
/// }
#[async_trait]
pub trait WebsocketHandler: Send + Sync {
    /// Function to implement to receive websocket messages.
    async fn callback(&self, _message: WebsocketEvent) {}
}

#[async_trait]
impl<T: WebsocketHandler> WebsocketHandler for Arc<T> {
    async fn callback(&self, message: WebsocketEvent) {
        self.as_ref().callback(message).await
    }
}

/// Websocket event names.
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum WebsocketEventType {
    AddedToTeam(Value),
    AuthenticationChallenge(Value),
    ChannelConverted(Value),
    ChannelCreated(Value),
    ChannelDeleted(Value),
    ChannelMemberUpdated(Value),
    ChannelUpdated(Value),
    ChannelViewed(Value),
    ConfigChanged(Value),
    DeleteTeam(Value),
    DirectAdded(Value),
    EmojiAdded(Value),
    EphemeralMessage(Value),
    GroupAdded(Value),
    Hello(Value),
    LeaveTeam(Value),
    LicenseChanged(Value),
    MemberroleUpdated(Value),
    NewUser(Value),
    PluginDisabled(Value),
    PluginEnabled(Value),
    PluginStatusesChanged(Value),
    PostDeleted(Value),
    PostEdited(Value),
    PostUnread(Value),
    Posted { post: Post, mentions: Vec<String> },
    PreferenceChanged(Value),
    PreferencesChanged(Value),
    PreferencesDeleted(Value),
    ReactionAdded(Value),
    ReactionRemoved(Value),
    Response(Value),
    RoleUpdated(Value),
    StatusChange(Value),
    Typing(Value),
    UpdateTeam(Value),
    UserAdded(Value),
    UserRemoved(Value),
    UserRoleUpdated(Value),
    UserUpdated(Value),
    DialogOpened(Value),
    ThreadUpdated(Value),
    ThreadFollowChanged(Value),
    ThreadReadChanged(Value),
}
