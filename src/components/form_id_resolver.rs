use std::collections::HashMap;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use yew::Html;

pub type FormIdResolver = HashMap<u32, FormIdEntry>;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct FormIdEntry {
    pub name: String,
    pub category: FormIdCategory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub enum FormIdCategory {
    CameraPath,
    Explosions,
    Weapon,
    Eyes,
    Activator,
    WaterForm,
    Static,
    MiscObjects,
    Doors,
    Containers,
    TESForm,
    NonPlayerCharacters,
    Weather,
    ListForm,
    Message,
    ImageSpace,
    ImpactDatasets,
    BodyPartData,
    TextureSet,
    EncounterZone,
    StoryManagerBranchNode,
    Class,
    Factions,
    AcousticSpace,
    EffectSetting,
    LandTexture,
    Enchantment,
    SpellItem,
    ScrollItem,
    TalkingActivator,
    Ammunition,
    Book,
    Ingredient,
    ObjectLight,
    Apparatus,
    Undefined,
    Grass,
    Tree,
    Flora,
    Furniture,
    Ammo,
    LeveledNPC,
    Key,
    AlchemyItem,
    IdleMarker,
    ConstructibleObject,
    Projectile,
    Hazard,
    SoulGem,
    LeveledItem,
    ShaderParticleGeometryData,
    ReferenceEffect,
    Region,
    ObjectREFR,
    Character,
    TopicInfo,
    Packages,
    CombatStyle,
    Loadscreen,
    LeveledSpell,
    EffectShader,
    Debris,
    Perk,
    AddonNode,
    ActorValueInfo,
    CameraShot,
    Material,
    ImpactData,
    ObjectARMA,
    Location,
    LightingTemplate,
    Footstep,
    FootstepSet,
    StoryManagerQuestNode,
    StoryManagerEventNode,
    DialogueBranch,
    MusicTrackFormWrapper,
    WordOfPower,
    Shout,
    EquipSlot,
    Relationship,
    Scene,
    AssociationType,
    Outfit,
    ArtObject,
    MaterialObject,
    MovementType,
    DualCastData,
    SoundCategory,
    SoundOutput,
    CollisionLayer,
    ColorForm,
    ReverbParameters,
    Global,
    KeyWord,
    LocationRefType,
    Action,
}

pub fn render_hex(val: u32) -> Html {
    let mut res: String = String::new();
    let _ = val
        .to_be_bytes()
        .iter()
        .map(|x| {
            if *x != 0 {
                res.push_str(&format!("{:02x?}", x).to_uppercase())
            } else {
                res.push_str("00")
            }
        })
        .collect::<()>();
    //res.push_str(&format!(" as decimal: {}", val));
    Html::from(res)
}
