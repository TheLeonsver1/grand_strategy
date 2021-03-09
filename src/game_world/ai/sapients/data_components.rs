use bevy::prelude::Entity;
pub enum Renown {
    Peasant,
    KnightHood,
    Baron,
    Count,
    Duke,
    King,
    Emperor,
}
pub enum Fame {
    No,
    Local,
    Regional,
    National,
    MultiNational,
}
///Tells us who are the family of this entity
pub struct OfDynasty(pub Entity);
///Who created this Dynasty/branch/Item
pub struct DynastyCreator(pub Entity);
///Tells us what this actor consideres home
pub struct LivingIn(pub Entity);
