use bevy::prelude::Entity;
///Tells us who owns this Building
pub struct BuildingOwner(pub Entity);

///Ruler of the Settlement
pub struct SettlementRuler(pub Entity);

///Tells us whose vassal this Ruler is
pub struct VassalOf(pub Entity);
