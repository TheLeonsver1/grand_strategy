use super::{ai_hierarchy::data_components::SettlementRuler, data_components::*};
use bevy::prelude::*;
#[derive(Bundle)]
pub struct AutocraticSettlementBundle {
    pub name: Name,
    pub ruler: SettlementRuler,
}
