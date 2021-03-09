use bevy::prelude::*;
///Tells us in what settlement this building is located
pub struct InSettlement(pub Entity);
///Tells us in what building's inventory this Item is located
pub struct BuildingInventory(pub Entity);
