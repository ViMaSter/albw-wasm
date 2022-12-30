use std::collections::{HashMap, HashSet};

use wasm_bindgen::prelude::*;
use randomizer::{filler::{assumed_search, get_items, prefill_check_map}, Settings, LocationInfo, Seed, world::build_world_graph};
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

#[wasm_bindgen]
pub fn fill_stuff(jsettings: JsValue, seed: Seed) -> JsValue {
    console_error_panic_hook::set_once();
    let asettings: ASettings = serde_wasm_bindgen::from_value(jsettings).unwrap();

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
    let (mut progression_pool, mut trash_pool) = get_items(&settings, &mut rng);

    let mut check_map = prefill_check_map(&mut world_graph);

    let reachable_checks = assumed_search(&mut world_graph, &progression_pool, &mut check_map, &settings); //find_reachable_checks(loc_map, &everything, &mut check_map); //

    // create Vec<string> of reachable checks .name field
    let mut reachable_checks: Vec<String> = reachable_checks.iter().map(|check| check.name.to_string()).collect();
    
    serde_wasm_bindgen::to_value(&reachable_checks).unwrap()
}