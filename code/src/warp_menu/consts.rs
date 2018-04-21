pub type Stage = (&'static str, &'static str);

#[derive(Copy, Clone)]
pub enum Category {
    LastWarp,
    Cavern,
    Developer,
    DragonRoostCavern,
    DragonRoostIsland,
    EarthTemple,
    ForbiddenWoods,
    ForestHaven,
    ForsakenFortress,
    GanonsTower,
    GreatFairy,
    Hyrule,
    NintendoGallery,
    Other,
    Outset,
    SavageLabyrinth,
    Sea,
    ToweroftheGods,
    WindTemple,
    Windfall,
}

pub static CATEGORIES: [(&str, Category); 20] = [
    ("Last Warp", Category::LastWarp),
    ("Cavern", Category::Cavern),
    ("Developer", Category::Developer),
    ("Dragon Roost Cavern", Category::DragonRoostCavern),
    ("Dragon Roost Island", Category::DragonRoostIsland),
    ("Earth Temple", Category::EarthTemple),
    ("Forbidden Woods", Category::ForbiddenWoods),
    ("Forest Haven", Category::ForestHaven),
    ("Forsaken Fortress", Category::ForsakenFortress),
    ("Ganon's Tower", Category::GanonsTower),
    ("Great Fairy", Category::GreatFairy),
    ("Hyrule", Category::Hyrule),
    ("Nintendo Gallery", Category::NintendoGallery),
    ("Other", Category::Other),
    ("Outset", Category::Outset),
    ("Savage Labyrinth", Category::SavageLabyrinth),
    ("Sea", Category::Sea),
    ("Tower of the Gods", Category::ToweroftheGods),
    ("Wind Temple", Category::WindTemple),
    ("Windfall", Category::Windfall),
];

pub static STAGES_CAVERN: &[Stage; 19] = &[
    ("Bomb Island", "Cave01"),
    ("Star Island", "Cave02"),
    ("Cliff Plateau Isles", "Cave03"),
    ("Rock Spire Isle", "Cave04"),
    ("Horseshoe Island", "Cave05"),
    ("Pawprint Isle Wizzrobe", "Cave07"),
    ("Shark Island", "ITest63"),
    ("Ice Ring Isle", "MiniHyo"),
    ("Fire Mountain", "MiniKaz"),
    ("Needle Rock Isle", "SubD42"),
    ("Angular Isles", "SubD43"),
    ("Boating Course", "SubD71"),
    ("Stone Watcher Island", "TF_01"),
    ("Overlook Island", "TF_02"),
    ("Birds Peak Rock", "TF_03"),
    ("Cabana", "TF_04"),
    ("Dragon Roost Island", "TF_06"),
    ("Pawprint Isle Chuchu", "TyuTyu"),
    ("Diamond Steppe Island", "WarpD"),
];

pub static STAGES_DEV: &[Stage; 45] = &[
    ("Invisible Island", "A_nami"),
    ("E3 Forest", "A_R00"),
    ("Amos T", "Amos_T"),
    ("Wind Temple", "Cave08"),
    ("Outset Island", "DmSpot0"),
    ("E3 Boating Course", "E3ROOP"),
    ("Island With House", "Ebesso"),
    ("Pig Chamber", "H_test"),
    ("Fire Cavern With Switches", "ITest61"),
    ("Ice Ring Isle Cavern", "ITest62"),
    ("Fire Mountain Cavern", "I_SubAN"),
    ("Basic Actions", "I_TestM"),
    ("Rope Room", "I_TestR"),
    ("Bridge Room", "KATA_HB"),
    ("Large Empty Room", "KATA_RM"),
    ("Fire Mountain", "kazan"),
    ("K Test2", "K_Test2"),
    ("K Test3", "K_Test3"),
    ("K Test4", "K_Test4"),
    ("K Test5", "K_Test5"),
    ("K Test6", "K_Test6"),
    ("K Test8", "K_Test8"),
    ("K Test9", "K_Test9"),
    ("K Testa", "K_Testa"),
    ("K Testb", "K_Testb"),
    ("K Testc", "K_Testc"),
    ("K Testd", "K_Testd"),
    ("K Teste", "K_Teste"),
    ("Camera Test", "morocam"),
    ("Smoke Test Room", "Msmoke"),
    ("Headstone Island", "Mukao"),
    ("Early Pirate Ship", "Obshop"),
    ("Dev Ending", "ENDumi"),
    ("Orcas Room", "Ojhous2"),
    ("Ghost Ship 1", "PShip2"),
    ("Ghost Ship 2", "PShip3"),
    ("Ghost Ship 3", "SubD45"),
    ("Ship Control Test", "sea_E"),
    ("Stone Watcher Island Cavern", "SubD44"),
    ("Bomb Island Cavern", "SubD51"),
    ("Decorative Pedestals", "TEST"),
    ("Dark Cavern With Switches", "TF_05"),
    ("Grotto With Darknuts", "TF_07"),
    ("Tingles Room", "tincle"),
    ("Basic Island", "VrTest"),
];

pub static STAGES_DRAGON_ROOST_CAVERN: &[Stage; 3] = &[
    ("Dungeon", "M_NewD2"),
    ("Boss", "M_DragB"),
    ("Mini Boss", "M_Dra09"),
];

pub static STAGES_DRAGON_ROOST_ISLAND: &[Stage; 3] = &[
    ("Pond", "Adanmae"),
    ("Komalis Room", "Comori"),
    ("Postal Service", "Atorizk"),
];

pub static STAGES_EARTH_TEMPLE: &[Stage; 4] = &[
    ("Entrance", "Edaichi"),
    ("Temple", "M_Dai"),
    ("Boss", "M_DaiB"),
    ("Mini Boss", "M_DaiMB"),
];

pub static STAGES_FORBIDDEN_WOODS: &[Stage; 3] = &[
    ("Dungeon", "kindan"),
    ("Boss", "kinBOSS"),
    ("Mini Boss", "kinMB"),
];

pub static STAGES_FOREST_HAVEN: &[Stage; 3] = &[
    ("Potion Room", "Ocrogh"),
    ("Forest Haven", "Omori"),
    ("Makars Hiding Place", "Otkura"),
];

pub static STAGES_FORSAKEN_FORTRESS: &[Stage; 7] = &[
    ("Ganondorfs Room", "M2ganon"),
    ("FF1 Tower", "Mjtower"),
    ("FF2 Tower", "M2tower"),
    ("FF1 Interior", "majroom"),
    ("FF2 Interior", "ma2room"),
    ("FF3 Interior", "ma3room"),
    ("FF1", "MajyuE"),
];

pub static STAGES_GANONS_TOWER: &[Stage; 15] = &[
    ("Entrance", "GanonA"),
    ("Room Towards Gohma", "GanonB"),
    ("Room Towards Molgera", "GanonC"),
    ("Room Towards Kalle Demos", "GanonD"),
    ("Room Towards Jalhalla", "GanonE"),
    ("Phantom Ganons Maze", "GanonJ"),
    ("Puppet Ganon", "GanonK"),
    ("Staircase Towards Puppet Ganon", "GanonL"),
    ("Main Room", "GanonM"),
    ("Staircase To Main Room", "GanonN"),
    ("Tower", "GTower"),
    ("Gohma", "Xboss0"),
    ("Kalle Demos", "Xboss1"),
    ("Jalhalla", "Xboss2"),
    ("Molgera", "Xboss3"),
];

pub static STAGES_GREAT_FAIRY: &[Stage; 6] = &[
    ("North", "Fairy01"),
    ("East", "Fairy02"),
    ("West", "Fairy03"),
    ("Forest Of Fairies", "Fairy04"),
    ("Thorned", "Fairy05"),
    ("South", "Fairy06"),
];

pub static STAGES_HYRULE: &[Stage; 3] = &[
    ("Castle", "Hyroom"),
    ("Field", "Hyrule"),
    ("Master Sword Chamber", "kenroom"),
];

pub static STAGES_NINTENDO_GALLERY: &[Stage; 8] = &[
    ("Great Sea", "figureA"),
    ("Windfall Island", "figureB"),
    ("Outset Island", "figureC"),
    ("Forsaken Fortress", "figureD"),
    ("Secret Cavern", "figureE"),
    ("Dragon Roost Island", "figureF"),
    ("Forest Haven", "figureG"),
    ("Main Room", "Pfigure"),
];

pub static STAGES_OTHER: &[Stage; 3] = &[
    ("Name Select", "Name"),
    ("Ending", "ENDING"),
    ("Title Screen", "sea_T"),
];

pub static STAGES_OUTSET: &[Stage; 7] = &[
    ("Links House", "LinkRM"),
    ("Under Links House", "LinkUG"),
    ("Forest Of Fairies", "A_mori"),
    ("Orcas Room", "Ojhous"),
    ("Mesas House", "Omasao"),
    ("Abe And Roses House", "Onobuta"),
    ("Jabuns Room", "Pjavdou"),
];

pub static STAGES_SAVAGE_LABYRINTH: &[Stage; 4] = &[
    ("Entrance", "Cave09"),
    ("Room11", "Cave10"),
    ("Room32", "Cave11"),
    ("End", "Cave06"),
];

pub static STAGES_SEA: &[Stage; 8] = &[
    ("Sea", "sea"),
    ("Tetras Ship Inside", "Asoko"),
    ("Tetras Ship Outside", "A umikz"),
    ("Submarine", "Abship"),
    ("Cabana", "Abesso"),
    ("Boating Course", "Ocean"),
    ("Ghost Ship", "PShip"),
    ("Islet Of Steel", "ShipD"),
];

pub static STAGES_TOWER_OF_THE_GODS: &[Stage; 4] = &[
    ("Dungeon", "Siren"),
    ("Boss", "SirenB"),
    ("Mini Boss", "SirenMB"),
    ("Outside", "ADMumi"),
];

pub static STAGES_WIND_TEMPLE: &[Stage; 4] = &[
    ("Entrance", "Ekaze"),
    ("Temple", "kaze"),
    ("Boss", "kazeB"),
    ("Mini Boss", "kazeMB"),
];

pub static STAGES_WINDFALL: &[Stage; 8] = &[
    ("Game Room", "Kaisen"),
    ("School Of Joy", "Nitiyou"),
    ("Bomb Shop", "Obombh"),
    ("Lenzos House", "Ocmera"),
    ("Cafe Bar", "Opub"),
    ("House Of Wealth", "Orichh"),
    ("Chu Jelly Juice Shop", "Pdrgsh"),
    ("Jail", "Pnezumi"),
];

pub static SEA_ROOMS: &[&str; 49] = &[
    "Forsaken Fortress",
    "Star Island",
    "Northern Fairy Island",
    "Gale Isle",
    "Crescent Moon Island",
    "Seven-Star Isles",
    "Overlook Island",
    "Four-Eye Reef",
    "Mother and Child Isles",
    "Spectacle Island",
    "Windfall Island",
    "Pawprint Isle",
    "Dragon Roost Island",
    "Flight Control Platform",
    "Western Fairy Island",
    "Rock Spire Isle",
    "Tingle Island",
    "Northern Triangle Island",
    "Eastern Fairy Island",
    "Fire Mountain",
    "Star Belt Archipelago",
    "Three-Eye Reef",
    "Greatfish Isle",
    "Cyclops Reef",
    "Six-Eye Reef",
    "Tower of the Gods",
    "Eastern Triangle Island",
    "Thorned Fairy Island",
    "Needle Rock Isle",
    "Islet of Steel",
    "Stone Watcher Island",
    "Southern Triangle Island",
    "Private Oasis",
    "Bomb Island",
    "Bird's Peak Rock",
    "Diamond Steppe Island",
    "Five-Eye Reef",
    "Shark Island",
    "Southern Fairy Island",
    "Ice Ring Isle",
    "Forest Haven",
    "Cliff Plateau Isles",
    "Horseshoe Island",
    "Outset Island",
    "Headstone Island",
    "Two-Eye Reef",
    "Angular Isles",
    "Boating Course",
    "Five-Star Isles",
];
