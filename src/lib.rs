use std::collections::{HashMap, HashSet};

use wasm_bindgen::prelude::*;
use randomizer::{filler::{assumed_search, prefill_check_map}, Settings, Seed, world::build_world_graph, filler_item::FillerItem};
use albw::Item;
use rand::{rngs::StdRng, SeedableRng};
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[derive(Serialize, Deserialize)]
pub enum ALogicMode {
    Normal,
    Hard,
    GlitchBasic,
    GlitchAdvanced,
    GlitchHell,
    NoLogic,
}

#[derive(Serialize, Deserialize)]
pub struct ALogic {
    /// Logic to use for item placement (Normal, Hard, Glitched (Basic, Advanced, Hell), No Logic)
    pub mode: ALogicMode,
    /// Guarantees a Weapon is placed in Ravio's Shop
    pub assured_weapon: bool,
    /// Places the Bell in Ravio's Shop
    pub bell_in_shop: bool,
    /// Places the Pouch in Ravio's Shop
    pub pouch_in_shop: bool,
    /// Places the Pegasus Boots in Ravio's Shop
    pub boots_in_shop: bool,
    /// Excludes Cucco Ranch, both Rupee Rushes, Treacherous Tower, Octoball Derby, and Hyrule Hotfoot
    pub minigames_excluded: bool,
    /// Swordless Mode
    pub swordless_mode: bool,
    /// Shuffle Super Lamp and Super Net
    pub super_items: bool,
    /// Skip Trials Door in Lorule Castle
    pub skip_trials: bool,
    /// Guarantees Bow of Light will be placed in Lorule Castle
    pub bow_of_light_in_castle: bool,
    /// Lamp Requirement. If enabled, the player may have to cross dark rooms without Lamp
    pub lampless: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AOptions {
    /// Experimental: Change Hyrule to the nighttime color scheme (until visiting Lorule)
    pub night_mode: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AExclusion(pub(crate) HashMap<String, HashSet<String>>);

#[derive(Serialize, Deserialize)]
pub struct AWorld(HashMap<String, HashSet<String>>);


#[derive(Serialize, Deserialize)]
pub struct AExclude {
    hyrule: AWorld,
    lorule: AWorld,
    dungeons: AWorld,
}

#[derive(Serialize, Deserialize)]
pub struct ASettings {
    pub logic: ALogic,
    pub options: AOptions,
    pub exclusions: AExclusion,
    pub exclude: AExclude
}

#[derive(Serialize, Deserialize)]
pub struct ASubregion {
    name: String,
    world: AWorld,
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ALocationInfo {
    subregion: ASubregion,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AChecks {
    location_info: ALocationInfo,
    item: Item,
}

impl From<ASettings> for Settings {
    fn from(b: ASettings) -> Settings {
        unsafe { std::mem::transmute(b) }
    }
}

pub fn stringToFillerItem(s: &str) -> FillerItem {
    match s {
        "Bow01" => FillerItem::Bow01,
        "Bow02" => FillerItem::Bow02,
        "Boomerang01" => FillerItem::Boomerang01,
        "Boomerang02" => FillerItem::Boomerang02,
        "Hookshot01" => FillerItem::Hookshot01,
        "Hookshot02" => FillerItem::Hookshot02,
        "Bombs01" => FillerItem::Bombs01,
        "Bombs02" => FillerItem::Bombs02,
        "FireRod01" => FillerItem::FireRod01,
        "FireRod02" => FillerItem::FireRod02,
        "IceRod01" => FillerItem::IceRod01,
        "IceRod02" => FillerItem::IceRod02,
        "Hammer01" => FillerItem::Hammer01,
        "Hammer02" => FillerItem::Hammer02,
        "SandRod01" => FillerItem::SandRod01,
        "SandRod02" => FillerItem::SandRod02,
        "TornadoRod01" => FillerItem::TornadoRod01,
        "TornadoRod02" => FillerItem::TornadoRod02,
        "Bell" => FillerItem::Bell,
        "StaminaScroll" => FillerItem::StaminaScroll,
        "BowOfLight" => FillerItem::BowOfLight,
        "PegasusBoots" => FillerItem::PegasusBoots,
        "Flippers" => FillerItem::Flippers,
        "RaviosBracelet01" => FillerItem::RaviosBracelet01,
        "RaviosBracelet02" => FillerItem::RaviosBracelet02,
        "HylianShield" => FillerItem::HylianShield,
        "SmoothGem" => FillerItem::SmoothGem,
        "LetterInABottle" => FillerItem::LetterInABottle,
        "PremiumMilk" => FillerItem::PremiumMilk,
        "Pouch" => FillerItem::Pouch,
        "BeeBadge" => FillerItem::BeeBadge,
        "HintGlasses" => FillerItem::HintGlasses,
        "RupeeGreen" => FillerItem::RupeeGreen,
        "RupeeBlue" => FillerItem::RupeeBlue,
        "RupeeRed" => FillerItem::RupeeRed,
        "RupeePurple01" => FillerItem::RupeePurple01,
        "RupeePurple02" => FillerItem::RupeePurple02,
        "RupeePurple03" => FillerItem::RupeePurple03,
        "RupeePurple04" => FillerItem::RupeePurple04,
        "RupeePurple05" => FillerItem::RupeePurple05,
        "RupeePurple06" => FillerItem::RupeePurple06,
        "RupeePurple07" => FillerItem::RupeePurple07,
        "RupeePurple08" => FillerItem::RupeePurple08,
        "RupeePurple09" => FillerItem::RupeePurple09,
        "RupeePurple10" => FillerItem::RupeePurple10,
        "RupeePurple11" => FillerItem::RupeePurple11,
        "RupeePurple12" => FillerItem::RupeePurple12,
        "RupeePurple13" => FillerItem::RupeePurple13,
        "RupeePurple14" => FillerItem::RupeePurple14,
        "RupeePurple15" => FillerItem::RupeePurple15,
        "RupeePurple16" => FillerItem::RupeePurple16,
        "RupeePurple17" => FillerItem::RupeePurple17,
        "RupeePurple18" => FillerItem::RupeePurple18,
        "RupeeSilver01" => FillerItem::RupeeSilver01,
        "RupeeSilver02" => FillerItem::RupeeSilver02,
        "RupeeSilver03" => FillerItem::RupeeSilver03,
        "RupeeSilver04" => FillerItem::RupeeSilver04,
        "RupeeSilver05" => FillerItem::RupeeSilver05,
        "RupeeSilver06" => FillerItem::RupeeSilver06,
        "RupeeSilver07" => FillerItem::RupeeSilver07,
        "RupeeSilver08" => FillerItem::RupeeSilver08,
        "RupeeSilver09" => FillerItem::RupeeSilver09,
        "RupeeSilver10" => FillerItem::RupeeSilver10,
        "RupeeSilver11" => FillerItem::RupeeSilver11,
        "RupeeSilver12" => FillerItem::RupeeSilver12,
        "RupeeSilver13" => FillerItem::RupeeSilver13,
        "RupeeSilver14" => FillerItem::RupeeSilver14,
        "RupeeSilver15" => FillerItem::RupeeSilver15,
        "RupeeSilver16" => FillerItem::RupeeSilver16,
        "RupeeSilver17" => FillerItem::RupeeSilver17,
        "RupeeSilver18" => FillerItem::RupeeSilver18,
        "RupeeSilver19" => FillerItem::RupeeSilver19,
        "RupeeSilver20" => FillerItem::RupeeSilver20,
        "RupeeSilver21" => FillerItem::RupeeSilver21,
        "RupeeSilver22" => FillerItem::RupeeSilver22,
        "RupeeSilver23" => FillerItem::RupeeSilver23,
        "RupeeSilver24" => FillerItem::RupeeSilver24,
        "RupeeSilver25" => FillerItem::RupeeSilver25,
        "RupeeSilver26" => FillerItem::RupeeSilver26,
        "RupeeSilver27" => FillerItem::RupeeSilver27,
        "RupeeSilver28" => FillerItem::RupeeSilver28,
        "RupeeSilver29" => FillerItem::RupeeSilver29,
        "RupeeSilver30" => FillerItem::RupeeSilver30,
        "RupeeSilver31" => FillerItem::RupeeSilver31,
        "RupeeSilver32" => FillerItem::RupeeSilver32,
        "RupeeSilver33" => FillerItem::RupeeSilver33,
        "RupeeSilver34" => FillerItem::RupeeSilver34,
        "RupeeSilver35" => FillerItem::RupeeSilver35,
        "RupeeSilver36" => FillerItem::RupeeSilver36,
        "RupeeSilver37" => FillerItem::RupeeSilver37,
        "RupeeSilver38" => FillerItem::RupeeSilver38,
        "RupeeGold01" => FillerItem::RupeeGold01,
        "RupeeGold02" => FillerItem::RupeeGold02,
        "RupeeGold03" => FillerItem::RupeeGold03,
        "RupeeGold04" => FillerItem::RupeeGold04,
        "RupeeGold05" => FillerItem::RupeeGold05,
        "RupeeGold06" => FillerItem::RupeeGold06,
        "RupeeGold07" => FillerItem::RupeeGold07,
        "RupeeGold08" => FillerItem::RupeeGold08,
        "MonsterGuts" => FillerItem::MonsterGuts,
        "MonsterHorn" => FillerItem::MonsterHorn,
        "MonsterTail" => FillerItem::MonsterTail,
        "HeartPiece01" => FillerItem::HeartPiece01,
        "HeartPiece02" => FillerItem::HeartPiece02,
        "HeartPiece03" => FillerItem::HeartPiece03,
        "HeartPiece04" => FillerItem::HeartPiece04,
        "HeartPiece05" => FillerItem::HeartPiece05,
        "HeartPiece06" => FillerItem::HeartPiece06,
        "HeartPiece07" => FillerItem::HeartPiece07,
        "HeartPiece08" => FillerItem::HeartPiece08,
        "HeartPiece09" => FillerItem::HeartPiece09,
        "HeartPiece10" => FillerItem::HeartPiece10,
        "HeartPiece11" => FillerItem::HeartPiece11,
        "HeartPiece12" => FillerItem::HeartPiece12,
        "HeartPiece13" => FillerItem::HeartPiece13,
        "HeartPiece14" => FillerItem::HeartPiece14,
        "HeartPiece15" => FillerItem::HeartPiece15,
        "HeartPiece16" => FillerItem::HeartPiece16,
        "HeartPiece17" => FillerItem::HeartPiece17,
        "HeartPiece18" => FillerItem::HeartPiece18,
        "HeartPiece19" => FillerItem::HeartPiece19,
        "HeartPiece20" => FillerItem::HeartPiece20,
        "HeartPiece21" => FillerItem::HeartPiece21,
        "HeartPiece22" => FillerItem::HeartPiece22,
        "HeartPiece23" => FillerItem::HeartPiece23,
        "HeartPiece24" => FillerItem::HeartPiece24,
        "HeartPiece25" => FillerItem::HeartPiece25,
        "HeartPiece26" => FillerItem::HeartPiece26,
        "HeartPiece27" => FillerItem::HeartPiece27,
        "HeartContainer01" => FillerItem::HeartContainer01,
        "HeartContainer02" => FillerItem::HeartContainer02,
        "HeartContainer03" => FillerItem::HeartContainer03,
        "HeartContainer04" => FillerItem::HeartContainer04,
        "HeartContainer05" => FillerItem::HeartContainer05,
        "HeartContainer06" => FillerItem::HeartContainer06,
        "HeartContainer07" => FillerItem::HeartContainer07,
        "HeartContainer08" => FillerItem::HeartContainer08,
        "HeartContainer09" => FillerItem::HeartContainer09,
        "HeartContainer10" => FillerItem::HeartContainer10,
        "Bottle01" => FillerItem::Bottle01,
        "Bottle02" => FillerItem::Bottle02,
        "Bottle03" => FillerItem::Bottle03,
        "Bottle04" => FillerItem::Bottle04,
        "Bottle05" => FillerItem::Bottle05,
        "Lamp01" => FillerItem::Lamp01,
        "Lamp02" => FillerItem::Lamp02,
        "Sword01" => FillerItem::Sword01,
        "Sword02" => FillerItem::Sword02,
        "Sword03" => FillerItem::Sword03,
        "Sword04" => FillerItem::Sword04,
        "Glove01" => FillerItem::Glove01,
        "Glove02" => FillerItem::Glove02,
        "Net01" => FillerItem::Net01,
        "Net02" => FillerItem::Net02,
        "Mail01" => FillerItem::Mail01,
        "Mail02" => FillerItem::Mail02,
        "OreYellow" => FillerItem::OreYellow,
        "OreGreen" => FillerItem::OreGreen,
        "OreBlue" => FillerItem::OreBlue,
        "OreRed" => FillerItem::OreRed,
        "HyruleSanctuaryKey" => FillerItem::HyruleSanctuaryKey,
        "LoruleSanctuaryKey" => FillerItem::LoruleSanctuaryKey,
        "EasternCompass" => FillerItem::EasternCompass,
        "EasternKeyBig" => FillerItem::EasternKeyBig,
        "EasternKeySmall01" => FillerItem::EasternKeySmall01,
        "EasternKeySmall02" => FillerItem::EasternKeySmall02,
        "GalesCompass" => FillerItem::GalesCompass,
        "GalesKeyBig" => FillerItem::GalesKeyBig,
        "GalesKeySmall01" => FillerItem::GalesKeySmall01,
        "GalesKeySmall02" => FillerItem::GalesKeySmall02,
        "GalesKeySmall03" => FillerItem::GalesKeySmall03,
        "GalesKeySmall04" => FillerItem::GalesKeySmall04,
        "HeraCompass" => FillerItem::HeraCompass,
        "HeraKeyBig" => FillerItem::HeraKeyBig,
        "HeraKeySmall01" => FillerItem::HeraKeySmall01,
        "HeraKeySmall02" => FillerItem::HeraKeySmall02,
        "DarkCompass" => FillerItem::DarkCompass,
        "DarkKeyBig" => FillerItem::DarkKeyBig,
        "DarkKeySmall01" => FillerItem::DarkKeySmall01,
        "DarkKeySmall02" => FillerItem::DarkKeySmall02,
        "DarkKeySmall03" => FillerItem::DarkKeySmall03,
        "DarkKeySmall04" => FillerItem::DarkKeySmall04,
        "SwampCompass" => FillerItem::SwampCompass,
        "SwampKeyBig" => FillerItem::SwampKeyBig,
        "SwampKeySmall01" => FillerItem::SwampKeySmall01,
        "SwampKeySmall02" => FillerItem::SwampKeySmall02,
        "SwampKeySmall03" => FillerItem::SwampKeySmall03,
        "SwampKeySmall04" => FillerItem::SwampKeySmall04,
        "SkullCompass" => FillerItem::SkullCompass,
        "SkullKeyBig" => FillerItem::SkullKeyBig,
        "SkullKeySmall01" => FillerItem::SkullKeySmall01,
        "SkullKeySmall02" => FillerItem::SkullKeySmall02,
        "SkullKeySmall03" => FillerItem::SkullKeySmall03,
        "ThievesCompass" => FillerItem::ThievesCompass,
        "ThievesKeyBig" => FillerItem::ThievesKeyBig,
        "ThievesKeySmall" => FillerItem::ThievesKeySmall,
        "IceCompass" => FillerItem::IceCompass,
        "IceKeyBig" => FillerItem::IceKeyBig,
        "IceKeySmall01" => FillerItem::IceKeySmall01,
        "IceKeySmall02" => FillerItem::IceKeySmall02,
        "IceKeySmall03" => FillerItem::IceKeySmall03,
        "DesertCompass" => FillerItem::DesertCompass,
        "DesertKeyBig" => FillerItem::DesertKeyBig,
        "DesertKeySmall01" => FillerItem::DesertKeySmall01,
        "DesertKeySmall02" => FillerItem::DesertKeySmall02,
        "DesertKeySmall03" => FillerItem::DesertKeySmall03,
        "DesertKeySmall04" => FillerItem::DesertKeySmall04,
        "DesertKeySmall05" => FillerItem::DesertKeySmall05,
        "TurtleCompass" => FillerItem::TurtleCompass,
        "TurtleKeyBig" => FillerItem::TurtleKeyBig,
        "TurtleKeySmall01" => FillerItem::TurtleKeySmall01,
        "TurtleKeySmall02" => FillerItem::TurtleKeySmall02,
        "TurtleKeySmall03" => FillerItem::TurtleKeySmall03,
        "LoruleCastleCompass" => FillerItem::LoruleCastleCompass,
        "LoruleCastleKeySmall01" => FillerItem::LoruleCastleKeySmall01,
        "LoruleCastleKeySmall02" => FillerItem::LoruleCastleKeySmall02,
        "LoruleCastleKeySmall03" => FillerItem::LoruleCastleKeySmall03,
        "LoruleCastleKeySmall04" => FillerItem::LoruleCastleKeySmall04,
        "LoruleCastleKeySmall05" => FillerItem::LoruleCastleKeySmall05,
        "PendantOfCourage" => FillerItem::PendantOfCourage,
        "PendantOfWisdom" => FillerItem::PendantOfWisdom,
        "PendantOfPower" => FillerItem::PendantOfPower,
        "SageGulley" => FillerItem::SageGulley,
        "SageOren" => FillerItem::SageOren,
        "SageSeres" => FillerItem::SageSeres,
        "SageOsfala" => FillerItem::SageOsfala,
        "SageRosso" => FillerItem::SageRosso,
        "SageIrene" => FillerItem::SageIrene,
        "SageImpa" => FillerItem::SageImpa,
        "ScootFruit" => FillerItem::ScootFruit,
        "FoulFruit" => FillerItem::FoulFruit,
        "Shield" => FillerItem::Shield,
        "GoldBee" => FillerItem::GoldBee,
        "OpenSanctuaryDoors" => FillerItem::OpenSanctuaryDoors,
        "BigBombFlower" => FillerItem::BigBombFlower,
        "StylishWomansHouseOpen" => FillerItem::StylishWomansHouseOpen,
        "SkullEyeRight" => FillerItem::SkullEyeRight,
        "SkullEyeLeft" => FillerItem::SkullEyeLeft,
        "AccessPotionShop" => FillerItem::AccessPotionShop,
        "AccessMilkBar" => FillerItem::AccessMilkBar,
        "AccessHyruleBlacksmith" => FillerItem::AccessHyruleBlacksmith,
        "AccessLoruleCastleField" => FillerItem::AccessLoruleCastleField,
        "Triforce" => FillerItem::Triforce,
        _ => panic!("Unknown item: {}", s),
    }
}

#[wasm_bindgen]
pub fn fill_stuff(jsettings: JsValue, obtained_items_js: JsValue, seed: Seed) -> JsValue {
    console_error_panic_hook::set_once();
    let asettings: ASettings = serde_wasm_bindgen::from_value(jsettings).unwrap();
    let obtained_items_as_string : Vec<String> = serde_wasm_bindgen::from_value(obtained_items_js).unwrap();
    // converts obtainedItemsAsString to a vector of FillerItem in oneliner
    let obtained_items : Vec<FillerItem> = obtained_items_as_string.iter().map(|x| stringToFillerItem(x)).collect();
    let settings : Settings = asettings.into();

    log(&format!("Seed:                           {}", seed));
    //info!("Hash:                           {}", settings.hash().0);
    log(&format!("Logic:                          {}", match settings.logic.mode {
        Normal => "Normal",
        Hard => "Hard",
        GlitchBasic => "Glitched (Basic)",
        GlitchAdvanced => "Glitched (Advanced)",
        GlitchHell => "Glitched (Hell) - Did you really mean to choose this?",
        NoLogic => "No Logic",
    }));
    log(&format!("Super Items:                    {}", if settings.logic.super_items {"Included"} else {"Not Included"}));
    log(&format!("Trials:                         {}", if settings.logic.skip_trials {"Skipped"} else {"Normal"}));
    log(&format!("Dark Rooms:                     {}", if settings.logic.lampless {"Lamp Not Required"} else {"Lamp Required"}));
    log(&format!("Swords:                         {}\n", if settings.logic.swordless_mode {"Swordless Mode - NO SWORDS"} else {"Normal"}));


    let mut rng = StdRng::seed_from_u64(seed as u64);

    let mut world_graph = build_world_graph();

    let mut check_map = prefill_check_map(&mut world_graph);

    let reachable_checks = assumed_search(&mut world_graph, &obtained_items, &mut check_map, &settings); //find_reachable_checks(loc_map, &everything, &mut check_map); //

    // create Vec<string> of reachable checks .name field
    let mut reachable_checks: Vec<String> = reachable_checks.iter().map(|check| check.name.to_string()).collect();
    
    serde_wasm_bindgen::to_value(&reachable_checks).unwrap()
}