use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub feed: Vec<PostEntry>,
    pub cursor: String
}

#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostEntry {
    pub post: Post,
}

#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub uri: String,
    pub cid: String,
    pub author: Author,
    pub record: Record,
    pub reply_count: u32,
    pub repost_count: u32,
    pub like_count: u32,
    #[serde(with = "time::serde::rfc3339")]
    pub indexed_at: OffsetDateTime,
    pub reply: Option<Reply>,
}

#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyPost {
    pub uri: String,
    pub cid: String,
    pub author: Author,
    pub record: Record,
    pub reply_count: u32,
    pub repost_count: u32,
    pub like_count: u32,
    #[serde(with = "time::serde::rfc3339")]
    pub indexed_at: OffsetDateTime,
}
#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub did: String,
    pub handle: String,
    pub display_name: String,
    pub avatar: String,
    pub viewer: Viewer
}


#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub text: String,
    #[serde(rename="$type")]
    pub record_type: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}


#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    #[serde(rename="$type")]
    pub embed_type: String,
    pub images: Vec<ImageEmbed>,
}


#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageEmbed {
    pub alt: String,
    pub thumb: String,
    #[serde(rename="fullsize")]
    pub full_size: String
}


#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub root: ReplyPost,
    pub parent: ReplyPost
}


#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    pub muted: bool,
    pub blocked_by: bool,
}
