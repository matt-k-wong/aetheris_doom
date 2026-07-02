use super::states::STATES;
use aetheris::simulation::{Thing, WorldState};

#[derive(Clone, Copy)]
pub struct ThingDef {
    pub health: f32,
    pub speed: f32,
    pub radius: f32,
    pub height: f32,
    pub damage: i32,
    pub reaction_time: i32,
    pub pain_chance: u8,
    pub mass: i32,
}

pub const MONSTER_IMP: u16 = 3001;
pub const MONSTER_DEMON: u16 = 3002;
pub const MONSTER_BARON: u16 = 3003;
pub const MONSTER_ZOMBIEMAN: u16 = 3004;
pub const MONSTER_CACODEMON: u16 = 3005;
pub const MONSTER_LOST_SOUL: u16 = 3006;
pub const MONSTER_SERGEANT: u16 = 9;

// Weapon/Item Type Constants
pub const ITEM_SHOTGUN: u16 = 2001;
pub const ITEM_CHAINGUN: u16 = 2002;
pub const ITEM_ROCKET_LAUNCHER: u16 = 2003;
pub const ITEM_PLASMA_RIFLE: u16 = 2004;
pub const ITEM_CHAINSAW: u16 = 2005;
pub const ITEM_BFG9000: u16 = 2006;
pub const ITEM_CLIP: u16 = 2007;
pub const ITEM_SHELLS: u16 = 2008;
pub const ITEM_ROCKETS: u16 = 2010;
pub const ITEM_STIMPACK: u16 = 2011;
pub const ITEM_MEDIKIT: u16 = 2012;
pub const ITEM_SOULSPHERE: u16 = 2013;
pub const ITEM_HEALTH_BONUS: u16 = 2014;
pub const ITEM_ARMOR_BONUS: u16 = 2015;
pub const ITEM_GREEN_ARMOR: u16 = 2018;
pub const ITEM_BLUE_ARMOR: u16 = 2019;
pub const ITEM_INVULN: u16 = 2022;
pub const ITEM_BERSERK: u16 = 2023;
pub const ITEM_INVIS: u16 = 2024;
pub const ITEM_RADSUIT: u16 = 2025;
pub const ITEM_MAP: u16 = 2026;

// Key Type Constants
pub const KEY_BLUE: u16 = 5;
pub const KEY_YELLOW: u16 = 6;
pub const KEY_RED: u16 = 13;
pub const KEY_BLUE_SKULL: u16 = 40;
pub const KEY_YELLOW_SKULL: u16 = 39;
pub const KEY_RED_SKULL: u16 = 38;

// Effect/Projectile Type Constants
pub const EFFECT_BLOOD: u16 = 9999;
pub const EFFECT_BLOOD_GREEN: u16 = 9997;
pub const EFFECT_PUFF: u16 = 9998;

pub trait DoomThingExt {
    fn is_monster(&self) -> bool;
    fn is_flying(&self) -> bool;
    fn is_pickup(&self) -> bool;
    fn is_barrel(&self) -> bool;
    fn is_effect(&self) -> bool;
    fn initial_health(k: u16, world: &WorldState) -> f32;
    fn pain_chance(k: u16, world: &WorldState) -> u8;
    fn sprite_name<'a>(&self, world: &'a WorldState) -> &'a str;
    fn frame_char(&self, world: &WorldState) -> char;
}

impl DoomThingExt for Thing {
    fn is_monster(&self) -> bool {
        matches!(self.kind,
            7 | 9 | 16 |               // Spiderdemon, Shotgun Guy, Cyberdemon
            3001..=3006 |               // Imp, Demon, Baron, Zombieman, Cacodemon, Lost Soul
            64..=69 | 71 | 84           // Archvile, Chaingunner, Revenant, Mancubus, Arachnotron, Hell Knight, Pain Elemental, WolfSS
        )
    }
    fn is_flying(&self) -> bool {
        matches!(self.kind, 3005 | 3006) // Cacodemon, Lost Soul
    }
    fn is_pickup(&self) -> bool {
        matches!(self.kind, 2001..=2008 | 2010..=2015 | 2018..=2019 | 2022..=2026 | 2045..=2049 | 5..=6 | 13 | 17 | 38..=40)
    }
    fn is_barrel(&self) -> bool {
        self.kind == 2035
    }
    fn is_effect(&self) -> bool {
        matches!(self.kind, 9997 | 9998 | 9999)
    }
    fn initial_health(_k: u16, _world: &WorldState) -> f32 {
        DEFAULT_THING_DEFS
            .iter()
            .find(|&&(k, _)| k == _k)
            .map(|&(_, d)| d.health)
            .unwrap_or(100.0)
    }
    fn pain_chance(_k: u16, _world: &WorldState) -> u8 {
        DEFAULT_THING_DEFS
            .iter()
            .find(|&&(k, _)| k == _k)
            .map(|&(_, d)| d.pain_chance)
            .unwrap_or(0)
    }
    fn sprite_name<'a>(&self, _world: &'a WorldState) -> &'a str {
        if self.state_idx < STATES.len() {
            STATES[self.state_idx].sprite
        } else {
            "TROO"
        }
    }
    fn frame_char(&self, _world: &WorldState) -> char {
        if self.state_idx < STATES.len() {
            STATES[self.state_idx].frame
        } else {
            'A'
        }
    }
}

pub const DEFAULT_THING_DEFS: &[(u16, ThingDef)] = &[
    // Zombieman
    (
        3004,
        ThingDef {
            health: 20.0,
            speed: 8.0,
            radius: 20.0,
            height: 56.0,
            damage: 0,
            reaction_time: 8,
            pain_chance: 200,
            mass: 100,
        },
    ),
    // Imp
    (
        3001,
        ThingDef {
            health: 60.0,
            speed: 8.0,
            radius: 20.0,
            height: 56.0,
            damage: 3,
            reaction_time: 8,
            pain_chance: 200,
            mass: 100,
        },
    ),
    // Demon
    (
        3002,
        ThingDef {
            health: 150.0,
            speed: 10.0,
            radius: 30.0,
            height: 56.0,
            damage: 4,
            reaction_time: 8,
            pain_chance: 180,
            mass: 400,
        },
    ),
    // Baron
    (
        3003,
        ThingDef {
            health: 1000.0,
            speed: 8.0,
            radius: 24.0,
            height: 64.0,
            damage: 10,
            reaction_time: 8,
            pain_chance: 50,
            mass: 1000,
        },
    ),
    // Cacodemon
    (
        3005,
        ThingDef {
            health: 400.0,
            speed: 8.0,
            radius: 31.0,
            height: 56.0,
            damage: 5,
            reaction_time: 8,
            pain_chance: 128,
            mass: 400,
        },
    ),
    // Lost Soul
    (
        3006,
        ThingDef {
            health: 100.0,
            speed: 8.0,
            radius: 16.0,
            height: 56.0,
            damage: 3,
            reaction_time: 8,
            pain_chance: 255,
            mass: 50,
        },
    ),
    // Barrel
    (
        2035,
        ThingDef {
            health: 20.0,
            speed: 0.0,
            radius: 10.0,
            height: 32.0,
            damage: 0,
            reaction_time: 0,
            pain_chance: 0,
            mass: 100,
        },
    ),
    // Doom 2 Monsters
    // Archvile (64)
    (
        64,
        ThingDef {
            health: 700.0,
            speed: 15.0,
            radius: 20.0,
            height: 56.0,
            damage: 20,
            reaction_time: 8,
            pain_chance: 10,
            mass: 500,
        },
    ),
    // Chaingunner (65)
    (
        65,
        ThingDef {
            health: 70.0,
            speed: 8.0,
            radius: 20.0,
            height: 56.0,
            damage: 3,
            reaction_time: 8,
            pain_chance: 170,
            mass: 100,
        },
    ),
    // Revenant (66)
    (
        66,
        ThingDef {
            health: 300.0,
            speed: 10.0,
            radius: 20.0,
            height: 56.0,
            damage: 10,
            reaction_time: 8,
            pain_chance: 100,
            mass: 500,
        },
    ),
    // Mancubus (67)
    (
        67,
        ThingDef {
            health: 600.0,
            speed: 8.0,
            radius: 48.0,
            height: 64.0,
            damage: 20,
            reaction_time: 8,
            pain_chance: 80,
            mass: 1000,
        },
    ),
    // Arachnotron (68)
    (
        68,
        ThingDef {
            health: 500.0,
            speed: 12.0,
            radius: 64.0,
            height: 64.0,
            damage: 5,
            reaction_time: 8,
            pain_chance: 128,
            mass: 600,
        },
    ),
    // Hell Knight (69)
    (
        69,
        ThingDef {
            health: 500.0,
            speed: 8.0,
            radius: 24.0,
            height: 64.0,
            damage: 10,
            reaction_time: 8,
            pain_chance: 50,
            mass: 1000,
        },
    ),
    // Pain Elemental (71)
    (
        71,
        ThingDef {
            health: 400.0,
            speed: 8.0,
            radius: 31.0,
            height: 56.0,
            damage: 0,
            reaction_time: 8,
            pain_chance: 128,
            mass: 400,
        },
    ),
    // Spider Mastermind (7)
    (
        7,
        ThingDef {
            health: 3000.0,
            speed: 12.0,
            radius: 128.0,
            height: 100.0,
            damage: 3,
            reaction_time: 8,
            pain_chance: 40,
            mass: 1000,
        },
    ), // Large radius
    // Cyberdemon (16)
    (
        16,
        ThingDef {
            health: 4000.0,
            speed: 16.0,
            radius: 40.0,
            height: 110.0,
            damage: 20,
            reaction_time: 8,
            pain_chance: 20,
            mass: 1000,
        },
    ),
    // WolfSS (84)
    (
        84,
        ThingDef {
            health: 50.0,
            speed: 8.0,
            radius: 20.0,
            height: 56.0,
            damage: 3,
            reaction_time: 8,
            pain_chance: 170,
            mass: 100,
        },
    ),
];
