//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub preferred_username: String,
    pub summary: Option<String>,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub local: bool,
    pub public_key: String,
    pub private_key: Option<String>,
    pub feed_json: Option<String>,
    pub feed_atom: Option<String>,
    pub feed_rss: Option<String>,
    pub last_refreshed_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
