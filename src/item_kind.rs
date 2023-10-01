use bevy::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BlockSoundType {
    Cloth,
    Dirt,
    Gravel,
    Sand,
    Stone,
    Wood,
}

#[derive(Component, Default, PartialEq, Eq, Clone, Copy, Debug, Reflect)]
pub enum ItemKind {
    #[default]
    // row 1
    Dirt,
    Grass,
    Stone,
    Cobblestone,
    Deepslate,
    CobbledDeepslate,
    Bedrock,
    HayBale,
    OakLog,
    LeafedOakLog,
    OakPlank,
    Leaves,
    OakSapling,
    CraftingTable,
    Furnace,
    FurnaceBurning,
    // row 2
    Sand,
    Sandstone,
    RedSand,
    RedSandstone,
    Cactus,
    RedTulip,
    // row 3
    CoalOre,
    CoalOreDeepslate,
    CopperOre,
    CopperOreDeepslate,
    IronOre,
    IronOreDeepslate,
    GoldOre,
    GoldOreDeepslate,
    LapisOre,
    LapisOreDeepslate,
    RedstoneOre,
    RedstoneOreDeepslate,
    EmraldOre,
    EmraldOreDeepslate,
    DiamondOre,
    DiamondOreDeepslate,
}

impl ItemKind {
    pub fn is_block(&self) -> bool {
        match self {
            _ => true,
        }
    }

    pub fn get_sound_type(&self) -> Option<BlockSoundType> {
        if !self.is_block() {
            return None;
        }

        use ItemKind::*;
        match *self {
            OakSapling | RedTulip | Cactus => Some(BlockSoundType::Cloth),
            Dirt | Grass | HayBale | Leaves => Some(BlockSoundType::Dirt),
            Sand | RedSand => Some(BlockSoundType::Sand),
            Stone | Cobblestone | Deepslate | CobbledDeepslate | Bedrock | Furnace
            | FurnaceBurning | Sandstone | RedSandstone | CoalOre | CoalOreDeepslate
            | CopperOre | CopperOreDeepslate | IronOre | IronOreDeepslate | GoldOre
            | GoldOreDeepslate | LapisOre | LapisOreDeepslate | RedstoneOre
            | RedstoneOreDeepslate | EmraldOre | EmraldOreDeepslate | DiamondOre
            | DiamondOreDeepslate => Some(BlockSoundType::Stone),
            OakLog | LeafedOakLog | OakPlank | CraftingTable => Some(BlockSoundType::Wood),
            _ => None,
        }
    }

    pub fn to_index(&self) -> usize {
        use ItemKind::*;
        match *self {
            // row 1
            Dirt => 0,
            Grass => 1,
            Stone => 2,
            Cobblestone => 3,
            Deepslate => 4,
            CobbledDeepslate => 5,
            Bedrock => 6,
            HayBale => 7,
            OakLog => 8,
            LeafedOakLog => 9,
            OakPlank => 10,
            Leaves => 11,
            OakSapling => 12,
            CraftingTable => 13,
            Furnace => 14,
            FurnaceBurning => 15,
            // row 2
            Sand => 16,
            Sandstone => 17,
            RedSand => 18,
            RedSandstone => 19,
            Cactus => 20,
            RedTulip => 21,
            // row 3
            CoalOre => 32,
            CoalOreDeepslate => 33,
            CopperOre => 34,
            CopperOreDeepslate => 35,
            IronOre => 36,
            IronOreDeepslate => 37,
            GoldOre => 38,
            GoldOreDeepslate => 39,
            LapisOre => 40,
            LapisOreDeepslate => 41,
            RedstoneOre => 42,
            RedstoneOreDeepslate => 43,
            EmraldOre => 44,
            EmraldOreDeepslate => 45,
            DiamondOre => 46,
            DiamondOreDeepslate => 47,
        }
    }
}
