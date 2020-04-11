use crate::responses::BasicListing;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct CommentsData {
    pub total_awards_received: i32,
    pub approved_at_utc: i32,
    pub edited: bool,
    pub link_id: String,
    pub author_flair_template_id: String,
    pub likes: Option<bool>,
    pub saved: bool,
    pub id: String,
    pub gilded: i32,
    pub archived: bool,
    pub no_follow: bool,
    pub author: String,
    pub num_comments: i32,
    pub can_mod_post: bool,
    pub created_utc: i64,
    pub send_replies: bool,
    pub parent_id: String,
    pub score: i32,
    pub author_fullname: String,
    pub over_18: bool,
    pub approved_by: Option<String>,
    pub subreddit_id: String,
    pub body: String,
    pub link_title: String,
    pub name: String,
    pub author_patreon_flair: bool,
    pub downs: i32,
    pub is_submitter: bool,
    pub body_html: String,
    pub distinguished: Option<bool>,
    pub stickied: bool,
    pub author_premium: bool,
    pub can_gild: bool,
    pub subreddit: String,
    pub author_flair_text_color: String,
    pub score_hidden: bool,
    pub permalink: String,
    pub num_reports: Option<i32>,
    pub link_permalink: String,
    pub link_author: String,
    pub subreddit_name_prefixed: String,
    pub author_flair_text: String,
    pub link_url: String,
    pub created: i64,
    pub collapsed: bool,
    pub controversiality: i32,
    pub locked: bool,
    pub quarantine: bool,
    pub subreddit_type: String,
    pub ups: i32,
}

pub type Comments = BasicListing<CommentsData>;
