use activitypub_federation::fetch::object_id::ObjectId;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    AppError,
    entities::{
        impls::JsonUserFeedItem,
        user::Model as DbUser,
        user_feed::Model as DbUserFeed,
    }
};

impl DbUserFeed {
    // 转换为 JSON
    pub async fn into_json(self) -> Result<JsonUserFeed, AppError> {
        Ok(JsonUserFeed {
            hatsu: self.hatsu.and_then(|hatsu| serde_json::from_str(&hatsu).unwrap()),
            feed_url: Url::parse(&self.feed_url)?,
            next_url: self.next_url.and_then(|url| Some(Url::parse(&url).unwrap())),
            title: self.title,
            description: self.description,
            icon: self.icon.and_then(|url| Some(Url::parse(&url).unwrap())),
            language: self.language,
            items: serde_json::from_str(&self.items)?
        })
    }

    // 从 JSON 转换为本地格式
    pub async fn from_json(
        json: JsonUserFeed,
        user_id: ObjectId<DbUser>
    ) -> Result<Self, AppError> {
        Ok(Self {
            user_id: user_id.inner().to_string(),
            hatsu: json.hatsu.and_then(|hatsu| Some(serde_json::to_string(&hatsu).unwrap())),
            feed_url: json.feed_url.to_string(),
            next_url: json.next_url.and_then(|url| Some(url.to_string())),
            title: json.title,
            description: json.description,
            icon: json.icon.and_then(|url| Some(url.to_string())),
            language: json.language,
            items: serde_json::to_string(&json.items)?
        })
    }

    // 从字符串转换为本地格式
    pub async fn from_str(
        str: String,
        user_id: ObjectId<DbUser>
    ) -> Result<Self, AppError> {
        let json: JsonUserFeed = serde_json::from_str(&str)?;

        Self::from_json(json, user_id).await
    }
}

/// JSON Feed 1.1
/// 
/// https://www.jsonfeed.org/version/1.1/#top-level-a-name-top-level-a
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct JsonUserFeed {
    #[serde(rename = "_hatsu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatsu: Option<JsonUserFeedHatsu>,
    pub feed_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<Url>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub items: Vec<JsonUserFeedItem>,
}

/// Hatsu JSON Feed Extension
/// 
/// https://github.com/importantimport/hatsu/issues/1
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct JsonUserFeedHatsu {
    pub about: Option<Url>,
    pub banner_image: Option<Url>,
}
