pub trait BuildingMarker {}
#[derive(Default)]
pub struct ManorMarker;

impl BuildingMarker for ManorMarker {}
#[derive(Default)]
pub struct FarmMarker;

impl BuildingMarker for FarmMarker {}
#[derive(Default)]
pub struct ToolShackMarker;
impl BuildingMarker for ToolShackMarker {}
#[derive(Default)]
pub struct SmithyMarker;

impl BuildingMarker for SmithyMarker {}
#[derive(Default)]
pub struct PharmacyMarker;

impl BuildingMarker for PharmacyMarker {}
#[derive(Default)]
pub struct GeneralShopMarker;

impl BuildingMarker for GeneralShopMarker {}
#[derive(Default)]
pub struct ClothingStoreMarker;

impl BuildingMarker for ClothingStoreMarker {}
