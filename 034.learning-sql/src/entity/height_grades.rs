//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "height_grades")]
pub struct Model {
    pub height_level: String,
    #[sea_orm(column_type = "Float(Some(2))")]
    pub height_lowest: f32,
    #[sea_orm(column_type = "Float(Some(2))")]
    pub height_highest: f32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}