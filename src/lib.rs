use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use randomizer::{filler::{assumed_search, prefill_check_map, get_items}, Settings, Seed, world::build_world_graph, filler_item::FillerItem};
use rand::{rngs::StdRng, SeedableRng};
use lazy_static::lazy_static;

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

// Create static variable of mappings from stringToFillerItem
lazy_static! {
    static ref FILLER_ITEM_MAP: HashMap<std::string::String, FillerItem> = {
        let mut m = HashMap::new();
        m.insert("Bow01".to_string(), FillerItem::Bow01);
        m.insert("Bow02".to_string(), FillerItem::Bow02);
        m.insert("Boomerang01".to_string(), FillerItem::Boomerang01);
        m.insert("Boomerang02".to_string(), FillerItem::Boomerang02);
        m.insert("Hookshot01".to_string(), FillerItem::Hookshot01);
        m.insert("Hookshot02".to_string(), FillerItem::Hookshot02);
        m.insert("Bombs01".to_string(), FillerItem::Bombs01);
        m.insert("Bombs02".to_string(), FillerItem::Bombs02);
        m.insert("FireRod01".to_string(), FillerItem::FireRod01);
        m.insert("FireRod02".to_string(), FillerItem::FireRod02);
        m.insert("IceRod01".to_string(), FillerItem::IceRod01);
        m.insert("IceRod02".to_string(), FillerItem::IceRod02);
        m.insert("Hammer01".to_string(), FillerItem::Hammer01);
        m.insert("Hammer02".to_string(), FillerItem::Hammer02);
        m.insert("SandRod01".to_string(), FillerItem::SandRod01);
        m.insert("SandRod02".to_string(), FillerItem::SandRod02);
        m.insert("TornadoRod01".to_string(), FillerItem::TornadoRod01);
        m.insert("TornadoRod02".to_string(), FillerItem::TornadoRod02);
        m.insert("Bell".to_string(), FillerItem::Bell);
        m.insert("StaminaScroll".to_string(), FillerItem::StaminaScroll);
        m.insert("BowOfLight".to_string(), FillerItem::BowOfLight);
        m.insert("PegasusBoots".to_string(), FillerItem::PegasusBoots);
        m.insert("Flippers".to_string(), FillerItem::Flippers);
        m.insert("RaviosBracelet01".to_string(), FillerItem::RaviosBracelet01);
        m.insert("RaviosBracelet02".to_string(), FillerItem::RaviosBracelet02);
        m.insert("HylianShield".to_string(), FillerItem::HylianShield);
        m.insert("SmoothGem".to_string(), FillerItem::SmoothGem);
        m.insert("LetterInABottle".to_string(), FillerItem::LetterInABottle);
        m.insert("PremiumMilk".to_string(), FillerItem::PremiumMilk);
        m.insert("Pouch".to_string(), FillerItem::Pouch);
        m.insert("BeeBadge".to_string(), FillerItem::BeeBadge);
        m.insert("HintGlasses".to_string(), FillerItem::HintGlasses);
        m.insert("RupeeGreen".to_string(), FillerItem::RupeeGreen);
        m.insert("RupeeBlue".to_string(), FillerItem::RupeeBlue);
        m.insert("RupeeRed".to_string(), FillerItem::RupeeRed);
        m.insert("RupeePurple01".to_string(), FillerItem::RupeePurple01);
        m.insert("RupeePurple02".to_string(), FillerItem::RupeePurple02);
        m.insert("RupeePurple03".to_string(), FillerItem::RupeePurple03);
        m.insert("RupeePurple04".to_string(), FillerItem::RupeePurple04);
        m.insert("RupeePurple05".to_string(), FillerItem::RupeePurple05);
        m.insert("RupeePurple06".to_string(), FillerItem::RupeePurple06);
        m.insert("RupeePurple07".to_string(), FillerItem::RupeePurple07);
        m.insert("RupeePurple08".to_string(), FillerItem::RupeePurple08);
        m.insert("RupeePurple09".to_string(), FillerItem::RupeePurple09);
        m.insert("RupeePurple10".to_string(), FillerItem::RupeePurple10);
        m.insert("RupeePurple11".to_string(), FillerItem::RupeePurple11);
        m.insert("RupeePurple12".to_string(), FillerItem::RupeePurple12);
        m.insert("RupeePurple13".to_string(), FillerItem::RupeePurple13);
        m.insert("RupeePurple14".to_string(), FillerItem::RupeePurple14);
        m.insert("RupeePurple15".to_string(), FillerItem::RupeePurple15);
        m.insert("RupeePurple16".to_string(), FillerItem::RupeePurple16);
        m.insert("RupeePurple17".to_string(), FillerItem::RupeePurple17);
        m.insert("RupeePurple18".to_string(), FillerItem::RupeePurple18);
        m.insert("RupeeSilver01".to_string(), FillerItem::RupeeSilver01);
        m.insert("RupeeSilver02".to_string(), FillerItem::RupeeSilver02);
        m.insert("RupeeSilver03".to_string(), FillerItem::RupeeSilver03);
        m.insert("RupeeSilver04".to_string(), FillerItem::RupeeSilver04);
        m.insert("RupeeSilver05".to_string(), FillerItem::RupeeSilver05);
        m.insert("RupeeSilver06".to_string(), FillerItem::RupeeSilver06);
        m.insert("RupeeSilver07".to_string(), FillerItem::RupeeSilver07);
        m.insert("RupeeSilver08".to_string(), FillerItem::RupeeSilver08);
        m.insert("RupeeSilver09".to_string(), FillerItem::RupeeSilver09);
        m.insert("RupeeSilver10".to_string(), FillerItem::RupeeSilver10);
        m.insert("RupeeSilver11".to_string(), FillerItem::RupeeSilver11);
        m.insert("RupeeSilver12".to_string(), FillerItem::RupeeSilver12);
        m.insert("RupeeSilver13".to_string(), FillerItem::RupeeSilver13);
        m.insert("RupeeSilver14".to_string(), FillerItem::RupeeSilver14);
        m.insert("RupeeSilver15".to_string(), FillerItem::RupeeSilver15);
        m.insert("RupeeSilver16".to_string(), FillerItem::RupeeSilver16);
        m.insert("RupeeSilver17".to_string(), FillerItem::RupeeSilver17);
        m.insert("RupeeSilver18".to_string(), FillerItem::RupeeSilver18);
        m.insert("RupeeSilver19".to_string(), FillerItem::RupeeSilver19);
        m.insert("RupeeSilver20".to_string(), FillerItem::RupeeSilver20);
        m.insert("RupeeSilver21".to_string(), FillerItem::RupeeSilver21);
        m.insert("RupeeSilver22".to_string(), FillerItem::RupeeSilver22);
        m.insert("RupeeSilver23".to_string(), FillerItem::RupeeSilver23);
        m.insert("RupeeSilver24".to_string(), FillerItem::RupeeSilver24);
        m.insert("RupeeSilver25".to_string(), FillerItem::RupeeSilver25);
        m.insert("RupeeSilver26".to_string(), FillerItem::RupeeSilver26);
        m.insert("RupeeSilver27".to_string(), FillerItem::RupeeSilver27);
        m.insert("RupeeSilver28".to_string(), FillerItem::RupeeSilver28);
        m.insert("RupeeSilver29".to_string(), FillerItem::RupeeSilver29);
        m.insert("RupeeSilver30".to_string(), FillerItem::RupeeSilver30);
        m.insert("RupeeSilver31".to_string(), FillerItem::RupeeSilver31);
        m.insert("RupeeSilver32".to_string(), FillerItem::RupeeSilver32);
        m.insert("RupeeSilver33".to_string(), FillerItem::RupeeSilver33);
        m.insert("RupeeSilver34".to_string(), FillerItem::RupeeSilver34);
        m.insert("RupeeSilver35".to_string(), FillerItem::RupeeSilver35);
        m.insert("RupeeSilver36".to_string(), FillerItem::RupeeSilver36);
        m.insert("RupeeSilver37".to_string(), FillerItem::RupeeSilver37);
        m.insert("RupeeSilver38".to_string(), FillerItem::RupeeSilver38);
        m.insert("RupeeGold01".to_string(), FillerItem::RupeeGold01);
        m.insert("RupeeGold02".to_string(), FillerItem::RupeeGold02);
        m.insert("RupeeGold03".to_string(), FillerItem::RupeeGold03);
        m.insert("RupeeGold04".to_string(), FillerItem::RupeeGold04);
        m.insert("RupeeGold05".to_string(), FillerItem::RupeeGold05);
        m.insert("RupeeGold06".to_string(), FillerItem::RupeeGold06);
        m.insert("RupeeGold07".to_string(), FillerItem::RupeeGold07);
        m.insert("RupeeGold08".to_string(), FillerItem::RupeeGold08);
        m.insert("MonsterGuts".to_string(), FillerItem::MonsterGuts);
        m.insert("MonsterHorn".to_string(), FillerItem::MonsterHorn);
        m.insert("MonsterTail".to_string(), FillerItem::MonsterTail);
        m.insert("HeartPiece01".to_string(), FillerItem::HeartPiece01);
        m.insert("HeartPiece02".to_string(), FillerItem::HeartPiece02);
        m.insert("HeartPiece03".to_string(), FillerItem::HeartPiece03);
        m.insert("HeartPiece04".to_string(), FillerItem::HeartPiece04);
        m.insert("HeartPiece05".to_string(), FillerItem::HeartPiece05);
        m.insert("HeartPiece06".to_string(), FillerItem::HeartPiece06);
        m.insert("HeartPiece07".to_string(), FillerItem::HeartPiece07);
        m.insert("HeartPiece08".to_string(), FillerItem::HeartPiece08);
        m.insert("HeartPiece09".to_string(), FillerItem::HeartPiece09);
        m.insert("HeartPiece10".to_string(), FillerItem::HeartPiece10);
        m.insert("HeartPiece11".to_string(), FillerItem::HeartPiece11);
        m.insert("HeartPiece12".to_string(), FillerItem::HeartPiece12);
        m.insert("HeartPiece13".to_string(), FillerItem::HeartPiece13);
        m.insert("HeartPiece14".to_string(), FillerItem::HeartPiece14);
        m.insert("HeartPiece15".to_string(), FillerItem::HeartPiece15);
        m.insert("HeartPiece16".to_string(), FillerItem::HeartPiece16);
        m.insert("HeartPiece17".to_string(), FillerItem::HeartPiece17);
        m.insert("HeartPiece18".to_string(), FillerItem::HeartPiece18);
        m.insert("HeartPiece19".to_string(), FillerItem::HeartPiece19);
        m.insert("HeartPiece20".to_string(), FillerItem::HeartPiece20);
        m.insert("HeartPiece21".to_string(), FillerItem::HeartPiece21);
        m.insert("HeartPiece22".to_string(), FillerItem::HeartPiece22);
        m.insert("HeartPiece23".to_string(), FillerItem::HeartPiece23);
        m.insert("HeartPiece24".to_string(), FillerItem::HeartPiece24);
        m.insert("HeartPiece25".to_string(), FillerItem::HeartPiece25);
        m.insert("HeartPiece26".to_string(), FillerItem::HeartPiece26);
        m.insert("HeartPiece27".to_string(), FillerItem::HeartPiece27);
        m.insert("HeartContainer01".to_string(), FillerItem::HeartContainer01);
        m.insert("HeartContainer02".to_string(), FillerItem::HeartContainer02);
        m.insert("HeartContainer03".to_string(), FillerItem::HeartContainer03);
        m.insert("HeartContainer04".to_string(), FillerItem::HeartContainer04);
        m.insert("HeartContainer05".to_string(), FillerItem::HeartContainer05);
        m.insert("HeartContainer06".to_string(), FillerItem::HeartContainer06);
        m.insert("HeartContainer07".to_string(), FillerItem::HeartContainer07);
        m.insert("HeartContainer08".to_string(), FillerItem::HeartContainer08);
        m.insert("HeartContainer09".to_string(), FillerItem::HeartContainer09);
        m.insert("HeartContainer10".to_string(), FillerItem::HeartContainer10);
        m.insert("Bottle01".to_string(), FillerItem::Bottle01);
        m.insert("Bottle02".to_string(), FillerItem::Bottle02);
        m.insert("Bottle03".to_string(), FillerItem::Bottle03);
        m.insert("Bottle04".to_string(), FillerItem::Bottle04);
        m.insert("Bottle05".to_string(), FillerItem::Bottle05);
        m.insert("Lamp01".to_string(), FillerItem::Lamp01);
        m.insert("Lamp02".to_string(), FillerItem::Lamp02);
        m.insert("Sword01".to_string(), FillerItem::Sword01);
        m.insert("Sword02".to_string(), FillerItem::Sword02);
        m.insert("Sword03".to_string(), FillerItem::Sword03);
        m.insert("Sword04".to_string(), FillerItem::Sword04);
        m.insert("Glove01".to_string(), FillerItem::Glove01);
        m.insert("Glove02".to_string(), FillerItem::Glove02);
        m.insert("Net01".to_string(), FillerItem::Net01);
        m.insert("Net02".to_string(), FillerItem::Net02);
        m.insert("Mail01".to_string(), FillerItem::Mail01);
        m.insert("Mail02".to_string(), FillerItem::Mail02);
        m.insert("OreYellow".to_string(), FillerItem::OreYellow);
        m.insert("OreGreen".to_string(), FillerItem::OreGreen);
        m.insert("OreBlue".to_string(), FillerItem::OreBlue);
        m.insert("OreRed".to_string(), FillerItem::OreRed);
        m.insert("HyruleSanctuaryKey".to_string(), FillerItem::HyruleSanctuaryKey);
        m.insert("LoruleSanctuaryKey".to_string(), FillerItem::LoruleSanctuaryKey);
        m.insert("EasternCompass".to_string(), FillerItem::EasternCompass);
        m.insert("EasternKeyBig".to_string(), FillerItem::EasternKeyBig);
        m.insert("EasternKeySmall01".to_string(), FillerItem::EasternKeySmall01);
        m.insert("EasternKeySmall02".to_string(), FillerItem::EasternKeySmall02);
        m.insert("GalesCompass".to_string(), FillerItem::GalesCompass);
        m.insert("GalesKeyBig".to_string(), FillerItem::GalesKeyBig);
        m.insert("GalesKeySmall01".to_string(), FillerItem::GalesKeySmall01);
        m.insert("GalesKeySmall02".to_string(), FillerItem::GalesKeySmall02);
        m.insert("GalesKeySmall03".to_string(), FillerItem::GalesKeySmall03);
        m.insert("GalesKeySmall04".to_string(), FillerItem::GalesKeySmall04);
        m.insert("HeraCompass".to_string(), FillerItem::HeraCompass);
        m.insert("HeraKeyBig".to_string(), FillerItem::HeraKeyBig);
        m.insert("HeraKeySmall01".to_string(), FillerItem::HeraKeySmall01);
        m.insert("HeraKeySmall02".to_string(), FillerItem::HeraKeySmall02);
        m.insert("DarkCompass".to_string(), FillerItem::DarkCompass);
        m.insert("DarkKeyBig".to_string(), FillerItem::DarkKeyBig);
        m.insert("DarkKeySmall01".to_string(), FillerItem::DarkKeySmall01);
        m.insert("DarkKeySmall02".to_string(), FillerItem::DarkKeySmall02);
        m.insert("DarkKeySmall03".to_string(), FillerItem::DarkKeySmall03);
        m.insert("DarkKeySmall04".to_string(), FillerItem::DarkKeySmall04);
        m.insert("SwampCompass".to_string(), FillerItem::SwampCompass);
        m.insert("SwampKeyBig".to_string(), FillerItem::SwampKeyBig);
        m.insert("SwampKeySmall01".to_string(), FillerItem::SwampKeySmall01);
        m.insert("SwampKeySmall02".to_string(), FillerItem::SwampKeySmall02);
        m.insert("SwampKeySmall03".to_string(), FillerItem::SwampKeySmall03);
        m.insert("SwampKeySmall04".to_string(), FillerItem::SwampKeySmall04);
        m.insert("SkullCompass".to_string(), FillerItem::SkullCompass);
        m.insert("SkullKeyBig".to_string(), FillerItem::SkullKeyBig);
        m.insert("SkullKeySmall01".to_string(), FillerItem::SkullKeySmall01);
        m.insert("SkullKeySmall02".to_string(), FillerItem::SkullKeySmall02);
        m.insert("SkullKeySmall03".to_string(), FillerItem::SkullKeySmall03);
        m.insert("ThievesCompass".to_string(), FillerItem::ThievesCompass);
        m.insert("ThievesKeyBig".to_string(), FillerItem::ThievesKeyBig);
        m.insert("ThievesKeySmall".to_string(), FillerItem::ThievesKeySmall);
        m.insert("IceCompass".to_string(), FillerItem::IceCompass);
        m.insert("IceKeyBig".to_string(), FillerItem::IceKeyBig);
        m.insert("IceKeySmall01".to_string(), FillerItem::IceKeySmall01);
        m.insert("IceKeySmall02".to_string(), FillerItem::IceKeySmall02);
        m.insert("IceKeySmall03".to_string(), FillerItem::IceKeySmall03);
        m.insert("DesertCompass".to_string(), FillerItem::DesertCompass);
        m.insert("DesertKeyBig".to_string(), FillerItem::DesertKeyBig);
        m.insert("DesertKeySmall01".to_string(), FillerItem::DesertKeySmall01);
        m.insert("DesertKeySmall02".to_string(), FillerItem::DesertKeySmall02);
        m.insert("DesertKeySmall03".to_string(), FillerItem::DesertKeySmall03);
        m.insert("DesertKeySmall04".to_string(), FillerItem::DesertKeySmall04);
        m.insert("DesertKeySmall05".to_string(), FillerItem::DesertKeySmall05);
        m.insert("TurtleCompass".to_string(), FillerItem::TurtleCompass);
        m.insert("TurtleKeyBig".to_string(), FillerItem::TurtleKeyBig);
        m.insert("TurtleKeySmall01".to_string(), FillerItem::TurtleKeySmall01);
        m.insert("TurtleKeySmall02".to_string(), FillerItem::TurtleKeySmall02);
        m.insert("TurtleKeySmall03".to_string(), FillerItem::TurtleKeySmall03);
        m.insert("LoruleCastleCompass".to_string(), FillerItem::LoruleCastleCompass);
        m.insert("LoruleCastleKeySmall01".to_string(), FillerItem::LoruleCastleKeySmall01);
        m.insert("LoruleCastleKeySmall02".to_string(), FillerItem::LoruleCastleKeySmall02);
        m.insert("LoruleCastleKeySmall03".to_string(), FillerItem::LoruleCastleKeySmall03);
        m.insert("LoruleCastleKeySmall04".to_string(), FillerItem::LoruleCastleKeySmall04);
        m.insert("LoruleCastleKeySmall05".to_string(), FillerItem::LoruleCastleKeySmall05);
        m.insert("PendantOfCourage".to_string(), FillerItem::PendantOfCourage);
        m.insert("PendantOfWisdom".to_string(), FillerItem::PendantOfWisdom);
        m.insert("PendantOfPower".to_string(), FillerItem::PendantOfPower);
        m.insert("SageGulley".to_string(), FillerItem::SageGulley);
        m.insert("SageOren".to_string(), FillerItem::SageOren);
        m.insert("SageSeres".to_string(), FillerItem::SageSeres);
        m.insert("SageOsfala".to_string(), FillerItem::SageOsfala);
        m.insert("SageRosso".to_string(), FillerItem::SageRosso);
        m.insert("SageIrene".to_string(), FillerItem::SageIrene);
        m.insert("SageImpa".to_string(), FillerItem::SageImpa);
        m.insert("ScootFruit".to_string(), FillerItem::ScootFruit);
        m.insert("FoulFruit".to_string(), FillerItem::FoulFruit);
        m.insert("Shield".to_string(), FillerItem::Shield);
        m.insert("GoldBee".to_string(), FillerItem::GoldBee);
        m.insert("OpenSanctuaryDoors".to_string(), FillerItem::OpenSanctuaryDoors);
        m.insert("BigBombFlower".to_string(), FillerItem::BigBombFlower);
        m.insert("StylishWomansHouseOpen".to_string(), FillerItem::StylishWomansHouseOpen);
        m.insert("SkullEyeRight".to_string(), FillerItem::SkullEyeRight);
        m.insert("SkullEyeLeft".to_string(), FillerItem::SkullEyeLeft);
        m.insert("AccessPotionShop".to_string(), FillerItem::AccessPotionShop);
        m.insert("AccessMilkBar".to_string(), FillerItem::AccessMilkBar);
        m.insert("AccessHyruleBlacksmith".to_string(), FillerItem::AccessHyruleBlacksmith);
        m.insert("AccessLoruleCastleField".to_string(), FillerItem::AccessLoruleCastleField);
        m.insert("Triforce".to_string(), FillerItem::Triforce);
        m
    };
}

// get key based on value from FILLER_ITEM_MAP in oneliner
fn filler_item_to_string(filler_item: FillerItem) -> String {
    FILLER_ITEM_MAP
        .iter()
        .find(|&(_, &v)| v == filler_item)
        .map(|(k, _)| k.clone())
        .unwrap()
}

// method that returns the FillerItem value of a key inside lazy static ref FILLER_ITEM_MAP
pub fn string_to_filler_item(s: &String) -> FillerItem {
    match FILLER_ITEM_MAP.get(s) {
        Some(x) => *x,
        None => panic!("No FillerItem found for {}", s),
    }
}

#[wasm_bindgen]
pub struct Cartridge {
    settings : Settings,
    seed : Seed,
}

#[wasm_bindgen]
impl Cartridge {
    #[wasm_bindgen(constructor)]
    pub fn new(jsettings: JsValue, seed: Seed) -> Cartridge {
        console_error_panic_hook::set_once();
        let settings: Settings = serde_wasm_bindgen::from_value(jsettings).unwrap();

        log("Generating cartridge...");
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
    
        Cartridge {
            settings,
            seed,
        }
    }

    #[wasm_bindgen]
    pub fn get_trash_item_names(&self) -> JsValue {
        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        let (_progression_pool, trash_pool) = get_items(&self.settings, &mut rng);
        // convert trash_pool to string using filler_item_to_string
        let mut trash_pool : Vec<String> = trash_pool.iter().map(|x| filler_item_to_string(*x)).collect();
        trash_pool.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        
        serde_wasm_bindgen::to_value(&trash_pool).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_progression_item_names(&self) -> JsValue {
        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        let (progression_pool, _trash_pool) = get_items(&self.settings, &mut rng);
        // convert trash_pool to string using filler_item_to_string
        let mut progression_pool : Vec<String> = progression_pool.iter().map(|x| filler_item_to_string(*x)).collect();
        progression_pool.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        
        serde_wasm_bindgen::to_value(&progression_pool).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_available_checks(&self, obtained_items_js: JsValue) -> JsValue {
        let obtained_items_as_string : Vec<String> = serde_wasm_bindgen::from_value(obtained_items_js).unwrap();
        let obtained_items : Vec<FillerItem> = obtained_items_as_string.iter().map(|x| string_to_filler_item(x)).collect();

        let mut world_graph = build_world_graph();
        let mut check_map = prefill_check_map(&mut world_graph);
        let reachable_checks = assumed_search(&mut world_graph, &obtained_items, &mut check_map, &self.settings); //find_reachable_checks(loc_map, &everything, &mut check_map); //
        let reachable_check_names: Vec<String> = reachable_checks.iter().map(|check| check.name.to_string()).collect();
        serde_wasm_bindgen::to_value(&reachable_check_names).unwrap()
    }
}