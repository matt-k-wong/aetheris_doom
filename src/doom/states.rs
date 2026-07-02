#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterAction {
    Look,
    Chase,
    FaceTarget,
    PosAttack,
    SPosAttack,
    TroopAttack,
    SargAttack,
    HeadAttack,
    BruisAttack,
    SkelMissile,
    FatAttack,
    VileChase,
    VileAttack,
    PainAttack,
    Pain,
    Scream,
    Fall,
    Explode,
    Raise,
    SkullAttack,
}

#[derive(Clone, Copy)]
pub struct MobjState {
    pub sprite: &'static str,
    pub frame: char,
    pub duration: i32,
    pub action: Option<MonsterAction>,
    pub next_state: usize,
}

pub const S_NULL: usize = 0;
// Zombieman States
pub const S_POSS_STND: usize = 1;
pub const S_POSS_RUN: usize = 3;
pub const S_POSS_ATK: usize = 11;
pub const S_POSS_PAIN: usize = 15;
pub const S_POSS_DIE: usize = 17;
// Imp States
pub const S_TROO_STND: usize = 22;
pub const S_TROO_RUN: usize = 24;
pub const S_TROO_ATK: usize = 32;
pub const S_TROO_PAIN: usize = 38;
pub const S_TROO_DIE: usize = 40;
// Lost Soul States
pub const S_SKULL_STND: usize = 44;
pub const S_SKULL_RUN: usize = 46;
pub const S_SKULL_ATK: usize = 48;
pub const S_SKULL_PAIN: usize = 49;
pub const S_SKULL_DIE: usize = 50;

// Barrel States
pub const S_BAR1: usize = 56;
pub const S_BEXP: usize = 58;

// Doom 2 Monster States
pub const S_CPOS_STND: usize = 63;
pub const S_CPOS_RUN: usize = 65;
pub const S_CPOS_ATK: usize = 71;
pub const S_CPOS_PAIN: usize = 74;
pub const S_CPOS_DIE: usize = 75;

pub const S_SKEL_STND: usize = 80;
pub const S_SKEL_RUN: usize = 82;
pub const S_SKEL_ATK: usize = 88;
pub const S_SKEL_PAIN: usize = 91;
pub const S_SKEL_DIE: usize = 92;

pub const S_FATT_STND: usize = 98;
pub const S_FATT_RUN: usize = 100;
pub const S_FATT_ATK: usize = 106;
pub const S_FATT_PAIN: usize = 109;
pub const S_FATT_DIE: usize = 110;

pub const S_BSPI_STND: usize = 116;
pub const S_BSPI_RUN: usize = 118;
pub const S_BSPI_ATK: usize = 124;
pub const S_BSPI_PAIN: usize = 127;
pub const S_BSPI_DIE: usize = 128;

pub const S_BOS2_STND: usize = 134;
pub const S_BOS2_RUN: usize = 136;
pub const S_BOS2_ATK: usize = 142;
pub const S_BOS2_PAIN: usize = 145;
pub const S_BOS2_DIE: usize = 146;

pub const S_PAIN_STND: usize = 152;
pub const S_PAIN_RUN: usize = 154;
pub const S_PAIN_ATK: usize = 160;
pub const S_PAIN_PAIN: usize = 163;
pub const S_PAIN_DIE: usize = 164;

pub const S_VILE_STND: usize = 170;
pub const S_VILE_RUN: usize = 172;
pub const S_VILE_ATK: usize = 178;
pub const S_VILE_PAIN: usize = 181;
pub const S_VILE_DIE: usize = 182;

pub const S_SPID_STND: usize = 188;
pub const S_SPID_RUN: usize = 190;
pub const S_SPID_ATK: usize = 196;
pub const S_SPID_PAIN: usize = 199;
pub const S_SPID_DIE: usize = 200;

pub const S_CYBR_STND: usize = 206;
pub const S_CYBR_RUN: usize = 208;
pub const S_CYBR_ATK: usize = 214;
pub const S_CYBR_PAIN: usize = 217;
pub const S_CYBR_DIE: usize = 218;

pub const S_SSWV_STND: usize = 224;
pub const S_SSWV_RUN: usize = 226;
pub const S_SSWV_ATK: usize = 232;
pub const S_SSWV_PAIN: usize = 235;
pub const S_SSWV_DIE: usize = 236;

// Shotgun Guy (SPOS) — separate sprites from Zombieman
pub const S_SPOS_STND: usize = 240;
pub const S_SPOS_RUN: usize = 242;
pub const S_SPOS_ATK: usize = 250;
pub const S_SPOS_PAIN: usize = 254;
pub const S_SPOS_DIE: usize = 256;

// Demon/Pinky (SARG)
pub const S_SARG_STND: usize = 261;
pub const S_SARG_RUN: usize = 263;
pub const S_SARG_ATK: usize = 271;
pub const S_SARG_PAIN: usize = 274;
pub const S_SARG_DIE: usize = 276;

// Cacodemon (HEAD)
pub const S_HEAD_STND: usize = 282;
pub const S_HEAD_RUN: usize = 284;
pub const S_HEAD_ATK: usize = 290;
pub const S_HEAD_PAIN: usize = 294;
pub const S_HEAD_DIE: usize = 296;

// Baron of Hell (BOSS)
pub const S_BOSS_STND: usize = 302;
pub const S_BOSS_RUN: usize = 304;
pub const S_BOSS_ATK: usize = 308;
pub const S_BOSS_PAIN: usize = 312;
pub const S_BOSS_DIE: usize = 314;

pub const STATES: &[MobjState] = DEFAULT_STATES;

pub const DEFAULT_STATES: &[MobjState] = &[
    /* 0 S_NULL */
    MobjState {
        sprite: "TNT1",
        frame: 'A',
        duration: -1,
        action: None,
        next_state: 0,
    },
    /* 1 S_POSS_STND */
    MobjState {
        sprite: "POSS",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 2,
    },
    /* 2 */
    MobjState {
        sprite: "POSS",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 1,
    },
    /* 3 S_POSS_RUN */
    MobjState {
        sprite: "POSS",
        frame: 'A',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 4,
    },
    /* 4 */
    MobjState {
        sprite: "POSS",
        frame: 'A',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 5,
    },
    /* 5 */
    MobjState {
        sprite: "POSS",
        frame: 'B',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 6,
    },
    /* 6 */
    MobjState {
        sprite: "POSS",
        frame: 'B',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 7,
    },
    /* 7 */
    MobjState {
        sprite: "POSS",
        frame: 'C',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 8,
    },
    /* 8 */
    MobjState {
        sprite: "POSS",
        frame: 'C',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 9,
    },
    /* 9 */
    MobjState {
        sprite: "POSS",
        frame: 'D',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 10,
    },
    /* 10 */
    MobjState {
        sprite: "POSS",
        frame: 'D',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 3,
    },
    /* 11 S_POSS_ATK */
    MobjState {
        sprite: "POSS",
        frame: 'E',
        duration: 10,
        action: Some(MonsterAction::FaceTarget),
        next_state: 12,
    },
    /* 12 */
    MobjState {
        sprite: "POSS",
        frame: 'F',
        duration: 8,
        action: Some(MonsterAction::PosAttack),
        next_state: 13,
    },
    /* 13 */
    MobjState {
        sprite: "POSS",
        frame: 'E',
        duration: 8,
        action: None,
        next_state: 3,
    }, // Back to chase
    /* 14 */
    MobjState {
        sprite: "POSS",
        frame: 'G',
        duration: 3,
        action: None,
        next_state: 15,
    },
    /* 15 S_POSS_PAIN */
    MobjState {
        sprite: "POSS",
        frame: 'G',
        duration: 3,
        action: Some(MonsterAction::Pain),
        next_state: 3,
    },
    /* 16 */
    MobjState {
        sprite: "POSS",
        frame: 'H',
        duration: 5,
        action: None,
        next_state: 17,
    },
    /* 17 S_POSS_DIE */
    MobjState {
        sprite: "POSS",
        frame: 'I',
        duration: 5,
        action: Some(MonsterAction::Scream),
        next_state: 18,
    },
    /* 18 */
    MobjState {
        sprite: "POSS",
        frame: 'J',
        duration: 5,
        action: Some(MonsterAction::Fall),
        next_state: 19,
    },
    /* 19 */
    MobjState {
        sprite: "POSS",
        frame: 'K',
        duration: 5,
        action: None,
        next_state: 20,
    },
    /* 20 */
    MobjState {
        sprite: "POSS",
        frame: 'L',
        duration: -1,
        action: None,
        next_state: 20,
    },
    /* 21 */
    MobjState {
        sprite: "POSS",
        frame: 'M',
        duration: 5,
        action: None,
        next_state: 22,
    }, // Extra check?
    /* 22 S_TROO_STND */
    MobjState {
        sprite: "TROO",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 23,
    },
    /* 23 */
    MobjState {
        sprite: "TROO",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 22,
    },
    /* 24 S_TROO_RUN */
    MobjState {
        sprite: "TROO",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 25,
    },
    /* 25 */
    MobjState {
        sprite: "TROO",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 26,
    },
    /* 26 */
    MobjState {
        sprite: "TROO",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 27,
    },
    /* 27 */
    MobjState {
        sprite: "TROO",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 28,
    },
    /* 28 */
    MobjState {
        sprite: "TROO",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 29,
    },
    /* 29 */
    MobjState {
        sprite: "TROO",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 30,
    },
    /* 30 */
    MobjState {
        sprite: "TROO",
        frame: 'D',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 31,
    },
    /* 31 */
    MobjState {
        sprite: "TROO",
        frame: 'D',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 24,
    },
    /* 32 S_TROO_ATK */
    MobjState {
        sprite: "TROO",
        frame: 'E',
        duration: 8,
        action: Some(MonsterAction::FaceTarget),
        next_state: 33,
    },
    /* 33 */
    MobjState {
        sprite: "TROO",
        frame: 'F',
        duration: 8,
        action: Some(MonsterAction::FaceTarget),
        next_state: 34,
    },
    /* 34 */
    MobjState {
        sprite: "TROO",
        frame: 'G',
        duration: 6,
        action: Some(MonsterAction::TroopAttack),
        next_state: 35,
    },
    /* 35 */
    MobjState {
        sprite: "TROO",
        frame: 'H',
        duration: 2,
        action: None,
        next_state: 36,
    }, // Extra frame?
    /* 36 */
    MobjState {
        sprite: "TROO",
        frame: 'H',
        duration: 2,
        action: None,
        next_state: 24,
    }, // Back to chase
    /* 37 */
    MobjState {
        sprite: "TROO",
        frame: 'H',
        duration: 2,
        action: None,
        next_state: 38,
    },
    /* 38 S_TROO_PAIN */
    MobjState {
        sprite: "TROO",
        frame: 'H',
        duration: 2,
        action: Some(MonsterAction::Pain),
        next_state: 24,
    },
    /* 39 */
    MobjState {
        sprite: "TROO",
        frame: 'I',
        duration: 8,
        action: None,
        next_state: 40,
    },
    /* 40 S_TROO_DIE */
    MobjState {
        sprite: "TROO",
        frame: 'J',
        duration: 8,
        action: Some(MonsterAction::Scream),
        next_state: 41,
    },
    /* 41 */
    MobjState {
        sprite: "TROO",
        frame: 'K',
        duration: 6,
        action: Some(MonsterAction::Fall),
        next_state: 42,
    },
    /* 42 */
    MobjState {
        sprite: "TROO",
        frame: 'L',
        duration: 6,
        action: None,
        next_state: 43,
    },
    /* 43 */
    MobjState {
        sprite: "TROO",
        frame: 'M',
        duration: -1,
        action: None,
        next_state: 43,
    },
    /* 44 S_SKULL_STND */
    MobjState {
        sprite: "SKUL",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 45,
    },
    /* 45 */
    MobjState {
        sprite: "SKUL",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 44,
    },
    /* 46 S_SKULL_RUN */
    MobjState {
        sprite: "SKUL",
        frame: 'A',
        duration: 6,
        action: Some(MonsterAction::Chase),
        next_state: 47,
    },
    /* 47 */
    MobjState {
        sprite: "SKUL",
        frame: 'B',
        duration: 6,
        action: Some(MonsterAction::Chase),
        next_state: 46,
    },
    /* 48 S_SKULL_ATK */
    MobjState {
        sprite: "SKUL",
        frame: 'C',
        duration: 20,
        action: Some(MonsterAction::SkullAttack),
        next_state: 46,
    },
    /* 49 S_SKULL_PAIN */
    MobjState {
        sprite: "SKUL",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Pain),
        next_state: 46,
    },
    /* 50 S_SKULL_DIE */
    MobjState {
        sprite: "SKUL",
        frame: 'F',
        duration: 6,
        action: Some(MonsterAction::Scream),
        next_state: 51,
    },
    /* 51 */
    MobjState {
        sprite: "SKUL",
        frame: 'G',
        duration: 6,
        action: None,
        next_state: 52,
    },
    /* 52 */
    MobjState {
        sprite: "SKUL",
        frame: 'H',
        duration: 6,
        action: Some(MonsterAction::Fall),
        next_state: 53,
    },
    /* 53 */
    MobjState {
        sprite: "SKUL",
        frame: 'I',
        duration: 6,
        action: None,
        next_state: 54,
    },
    /* 54 */
    MobjState {
        sprite: "SKUL",
        frame: 'J',
        duration: 6,
        action: None,
        next_state: 55,
    },
    /* 55 */
    MobjState {
        sprite: "SKUL",
        frame: 'K',
        duration: -1,
        action: None,
        next_state: 55,
    },
    /* 56 S_BAR1 */
    MobjState {
        sprite: "BAR1",
        frame: 'A',
        duration: 10,
        action: None,
        next_state: 57,
    },
    /* 57 */
    MobjState {
        sprite: "BAR1",
        frame: 'B',
        duration: 10,
        action: None,
        next_state: 56,
    },
    /* 58 S_BEXP */
    MobjState {
        sprite: "BEXP",
        frame: 'A',
        duration: 5,
        action: Some(MonsterAction::Explode),
        next_state: 59,
    },
    /* 59 */
    MobjState {
        sprite: "BEXP",
        frame: 'B',
        duration: 5,
        action: None,
        next_state: 60,
    },
    /* 60 */
    MobjState {
        sprite: "BEXP",
        frame: 'C',
        duration: 5,
        action: None,
        next_state: 61,
    },
    /* 61 */
    MobjState {
        sprite: "BEXP",
        frame: 'D',
        duration: 10,
        action: None,
        next_state: 62,
    },
    /* 62 */
    MobjState {
        sprite: "TNT1",
        frame: 'A',
        duration: -1,
        action: None,
        next_state: 62,
    },
    // Chaingunner (CPOS)
    /* 63 */
    MobjState {
        sprite: "CPOS",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 64,
    },
    /* 64 */
    MobjState {
        sprite: "CPOS",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 63,
    },
    /* 65 */
    MobjState {
        sprite: "CPOS",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 66,
    },
    /* 66 */
    MobjState {
        sprite: "CPOS",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 67,
    },
    /* 67 */
    MobjState {
        sprite: "CPOS",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 68,
    },
    /* 68 */
    MobjState {
        sprite: "CPOS",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 69,
    },
    /* 69 */
    MobjState {
        sprite: "CPOS",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 70,
    },
    /* 70 */
    MobjState {
        sprite: "CPOS",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 65,
    },
    /* 71 */
    MobjState {
        sprite: "CPOS",
        frame: 'E',
        duration: 10,
        action: Some(MonsterAction::FaceTarget),
        next_state: 72,
    },
    /* 72 */
    MobjState {
        sprite: "CPOS",
        frame: 'F',
        duration: 4,
        action: Some(MonsterAction::PosAttack),
        next_state: 73,
    },
    /* 73 */
    MobjState {
        sprite: "CPOS",
        frame: 'F',
        duration: 4,
        action: Some(MonsterAction::PosAttack),
        next_state: 65,
    },
    /* 74 */
    MobjState {
        sprite: "CPOS",
        frame: 'G',
        duration: 3,
        action: Some(MonsterAction::Pain),
        next_state: 65,
    },
    /* 75 */
    MobjState {
        sprite: "CPOS",
        frame: 'H',
        duration: 5,
        action: None,
        next_state: 76,
    },
    /* 76 */
    MobjState {
        sprite: "CPOS",
        frame: 'I',
        duration: 5,
        action: Some(MonsterAction::Scream),
        next_state: 77,
    },
    /* 77 */
    MobjState {
        sprite: "CPOS",
        frame: 'J',
        duration: 5,
        action: Some(MonsterAction::Fall),
        next_state: 78,
    },
    /* 78 */
    MobjState {
        sprite: "CPOS",
        frame: 'K',
        duration: 5,
        action: None,
        next_state: 79,
    },
    /* 79 */
    MobjState {
        sprite: "CPOS",
        frame: 'L',
        duration: -1,
        action: None,
        next_state: 79,
    },
    // Revenant (SKEL)
    /* 80 */
    MobjState {
        sprite: "SKEL",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 81,
    },
    /* 81 */
    MobjState {
        sprite: "SKEL",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 80,
    },
    /* 82 */
    MobjState {
        sprite: "SKEL",
        frame: 'A',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 83,
    },
    /* 83 */
    MobjState {
        sprite: "SKEL",
        frame: 'B',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 84,
    },
    /* 84 */
    MobjState {
        sprite: "SKEL",
        frame: 'C',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 85,
    },
    /* 85 */
    MobjState {
        sprite: "SKEL",
        frame: 'D',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 86,
    },
    /* 86 */
    MobjState {
        sprite: "SKEL",
        frame: 'E',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 87,
    },
    /* 87 */
    MobjState {
        sprite: "SKEL",
        frame: 'F',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 82,
    },
    /* 88 */
    MobjState {
        sprite: "SKEL",
        frame: 'J',
        duration: 10,
        action: Some(MonsterAction::FaceTarget),
        next_state: 89,
    },
    /* 89 */
    MobjState {
        sprite: "SKEL",
        frame: 'K',
        duration: 10,
        action: Some(MonsterAction::TroopAttack),
        next_state: 90,
    },
    /* 90 */
    MobjState {
        sprite: "SKEL",
        frame: 'K',
        duration: 10,
        action: None,
        next_state: 82,
    },
    /* 91 */
    MobjState {
        sprite: "SKEL",
        frame: 'L',
        duration: 5,
        action: Some(MonsterAction::Pain),
        next_state: 82,
    },
    /* 92 */
    MobjState {
        sprite: "SKEL",
        frame: 'L',
        duration: 5,
        action: None,
        next_state: 93,
    },
    /* 93 */
    MobjState {
        sprite: "SKEL",
        frame: 'M',
        duration: 5,
        action: Some(MonsterAction::Scream),
        next_state: 94,
    },
    /* 94 */
    MobjState {
        sprite: "SKEL",
        frame: 'N',
        duration: 5,
        action: Some(MonsterAction::Fall),
        next_state: 95,
    },
    /* 95 */
    MobjState {
        sprite: "SKEL",
        frame: 'O',
        duration: 5,
        action: None,
        next_state: 96,
    },
    /* 96 */
    MobjState {
        sprite: "SKEL",
        frame: 'P',
        duration: -1,
        action: None,
        next_state: 96,
    },
    /* 97 */
    MobjState {
        sprite: "SKEL",
        frame: 'Q',
        duration: -1,
        action: None,
        next_state: 97,
    },
    // Mancubus (FATT)
    /* 98 */
    MobjState {
        sprite: "FATT",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 99,
    },
    /* 99 */
    MobjState {
        sprite: "FATT",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 98,
    },
    /* 100 */
    MobjState {
        sprite: "FATT",
        frame: 'A',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 101,
    },
    /* 101 */
    MobjState {
        sprite: "FATT",
        frame: 'B',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 102,
    },
    /* 102 */
    MobjState {
        sprite: "FATT",
        frame: 'C',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 103,
    },
    /* 103 */
    MobjState {
        sprite: "FATT",
        frame: 'D',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 104,
    },
    /* 104 */
    MobjState {
        sprite: "FATT",
        frame: 'E',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 105,
    },
    /* 105 */
    MobjState {
        sprite: "FATT",
        frame: 'F',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 100,
    },
    /* 106 */
    MobjState {
        sprite: "FATT",
        frame: 'G',
        duration: 20,
        action: Some(MonsterAction::FaceTarget),
        next_state: 107,
    },
    /* 107 */
    MobjState {
        sprite: "FATT",
        frame: 'H',
        duration: 10,
        action: Some(MonsterAction::TroopAttack),
        next_state: 108,
    },
    /* 108 */
    MobjState {
        sprite: "FATT",
        frame: 'I',
        duration: 5,
        action: None,
        next_state: 100,
    },
    /* 109 */
    MobjState {
        sprite: "FATT",
        frame: 'J',
        duration: 3,
        action: Some(MonsterAction::Pain),
        next_state: 100,
    },
    /* 110 */
    MobjState {
        sprite: "FATT",
        frame: 'K',
        duration: 5,
        action: None,
        next_state: 111,
    },
    /* 111 */
    MobjState {
        sprite: "FATT",
        frame: 'L',
        duration: 5,
        action: Some(MonsterAction::Scream),
        next_state: 112,
    },
    /* 112 */
    MobjState {
        sprite: "FATT",
        frame: 'M',
        duration: 5,
        action: Some(MonsterAction::Fall),
        next_state: 113,
    },
    /* 113 */
    MobjState {
        sprite: "FATT",
        frame: 'N',
        duration: 5,
        action: None,
        next_state: 114,
    },
    /* 114 */
    MobjState {
        sprite: "FATT",
        frame: 'O',
        duration: -1,
        action: None,
        next_state: 114,
    },
    /* 115 */
    MobjState {
        sprite: "FATT",
        frame: 'P',
        duration: -1,
        action: None,
        next_state: 115,
    },
    // Arachnotron (BSPI)
    /* 116 */
    MobjState {
        sprite: "BSPI",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 117,
    },
    /* 117 */
    MobjState {
        sprite: "BSPI",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 116,
    },
    /* 118 */
    MobjState {
        sprite: "BSPI",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 119,
    },
    /* 119 */
    MobjState {
        sprite: "BSPI",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 120,
    },
    /* 120 */
    MobjState {
        sprite: "BSPI",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 121,
    },
    /* 121 */
    MobjState {
        sprite: "BSPI",
        frame: 'D',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 122,
    },
    /* 122 */
    MobjState {
        sprite: "BSPI",
        frame: 'E',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 123,
    },
    /* 123 */
    MobjState {
        sprite: "BSPI",
        frame: 'F',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 118,
    },
    /* 124 */
    MobjState {
        sprite: "BSPI",
        frame: 'A',
        duration: 20,
        action: Some(MonsterAction::FaceTarget),
        next_state: 125,
    },
    /* 125 */
    MobjState {
        sprite: "BSPI",
        frame: 'G',
        duration: 4,
        action: Some(MonsterAction::TroopAttack),
        next_state: 126,
    },
    /* 126 */
    MobjState {
        sprite: "BSPI",
        frame: 'H',
        duration: 4,
        action: None,
        next_state: 118,
    },
    /* 127 */
    MobjState {
        sprite: "BSPI",
        frame: 'I',
        duration: 3,
        action: Some(MonsterAction::Pain),
        next_state: 118,
    },
    /* 128 */
    MobjState {
        sprite: "BSPI",
        frame: 'J',
        duration: 5,
        action: None,
        next_state: 129,
    },
    /* 129 */
    MobjState {
        sprite: "BSPI",
        frame: 'K',
        duration: 5,
        action: Some(MonsterAction::Scream),
        next_state: 130,
    },
    /* 130 */
    MobjState {
        sprite: "BSPI",
        frame: 'L',
        duration: 5,
        action: Some(MonsterAction::Fall),
        next_state: 131,
    },
    /* 131 */
    MobjState {
        sprite: "BSPI",
        frame: 'M',
        duration: 5,
        action: None,
        next_state: 132,
    },
    /* 132 */
    MobjState {
        sprite: "BSPI",
        frame: 'N',
        duration: -1,
        action: None,
        next_state: 132,
    },
    /* 133 */
    MobjState {
        sprite: "BSPI",
        frame: 'O',
        duration: -1,
        action: None,
        next_state: 133,
    },
    // Hell Knight (BOS2)
    /* 134 */
    MobjState {
        sprite: "BOS2",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 135,
    },
    /* 135 */
    MobjState {
        sprite: "BOS2",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 134,
    },
    /* 136 */
    MobjState {
        sprite: "BOS2",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 137,
    },
    /* 137 */
    MobjState {
        sprite: "BOS2",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 138,
    },
    /* 138 */
    MobjState {
        sprite: "BOS2",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 139,
    },
    /* 139 */
    MobjState {
        sprite: "BOS2",
        frame: 'D',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 136,
    },
    /* 140 */
    MobjState {
        sprite: "BOS2",
        frame: 'A',
        duration: 0,
        action: None,
        next_state: 136,
    },
    /* 141 */
    MobjState {
        sprite: "BOS2",
        frame: 'A',
        duration: 0,
        action: None,
        next_state: 136,
    },
    /* 142 */
    MobjState {
        sprite: "BOS2",
        frame: 'E',
        duration: 8,
        action: Some(MonsterAction::FaceTarget),
        next_state: 143,
    },
    /* 143 */
    MobjState {
        sprite: "BOS2",
        frame: 'F',
        duration: 8,
        action: Some(MonsterAction::FaceTarget),
        next_state: 144,
    },
    /* 144 */
    MobjState {
        sprite: "BOS2",
        frame: 'G',
        duration: 8,
        action: Some(MonsterAction::TroopAttack),
        next_state: 136,
    },
    /* 145 */
    MobjState {
        sprite: "BOS2",
        frame: 'H',
        duration: 2,
        action: Some(MonsterAction::Pain),
        next_state: 136,
    },
    /* 146 */
    MobjState {
        sprite: "BOS2",
        frame: 'I',
        duration: 8,
        action: None,
        next_state: 147,
    },
    /* 147 */
    MobjState {
        sprite: "BOS2",
        frame: 'J',
        duration: 8,
        action: Some(MonsterAction::Scream),
        next_state: 148,
    },
    /* 148 */
    MobjState {
        sprite: "BOS2",
        frame: 'K',
        duration: 8,
        action: Some(MonsterAction::Fall),
        next_state: 149,
    },
    /* 149 */
    MobjState {
        sprite: "BOS2",
        frame: 'L',
        duration: 8,
        action: None,
        next_state: 150,
    },
    /* 150 */
    MobjState {
        sprite: "BOS2",
        frame: 'M',
        duration: -1,
        action: None,
        next_state: 150,
    },
    /* 151 */
    MobjState {
        sprite: "BOS2",
        frame: 'N',
        duration: -1,
        action: None,
        next_state: 151,
    },
    // Pain Elemental (PAIN)
    /* 152 */
    MobjState {
        sprite: "PAIN",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 153,
    },
    /* 153 */
    MobjState {
        sprite: "PAIN",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 152,
    },
    /* 154 */
    MobjState {
        sprite: "PAIN",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 155,
    },
    /* 155 */
    MobjState {
        sprite: "PAIN",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 156,
    },
    /* 156 */
    MobjState {
        sprite: "PAIN",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 154,
    },
    /* 157 */
    MobjState {
        sprite: "PAIN",
        frame: 'A',
        duration: 0,
        action: None,
        next_state: 154,
    },
    /* 158 */
    MobjState {
        sprite: "PAIN",
        frame: 'A',
        duration: 0,
        action: None,
        next_state: 154,
    },
    /* 159 */
    MobjState {
        sprite: "PAIN",
        frame: 'A',
        duration: 0,
        action: None,
        next_state: 154,
    },
    /* 160 */
    MobjState {
        sprite: "PAIN",
        frame: 'D',
        duration: 20,
        action: Some(MonsterAction::FaceTarget),
        next_state: 161,
    },
    /* 161 */
    MobjState {
        sprite: "PAIN",
        frame: 'E',
        duration: 10,
        action: Some(MonsterAction::SkullAttack),
        next_state: 162,
    },
    /* 162 */
    MobjState {
        sprite: "PAIN",
        frame: 'F',
        duration: 10,
        action: None,
        next_state: 154,
    },
    /* 163 */
    MobjState {
        sprite: "PAIN",
        frame: 'G',
        duration: 6,
        action: Some(MonsterAction::Pain),
        next_state: 154,
    },
    /* 164 */
    MobjState {
        sprite: "PAIN",
        frame: 'H',
        duration: 8,
        action: Some(MonsterAction::Scream),
        next_state: 165,
    },
    /* 165 */
    MobjState {
        sprite: "PAIN",
        frame: 'I',
        duration: 8,
        action: Some(MonsterAction::Fall),
        next_state: 166,
    },
    /* 166 */
    MobjState {
        sprite: "PAIN",
        frame: 'J',
        duration: 8,
        action: None,
        next_state: 167,
    },
    /* 167 */
    MobjState {
        sprite: "PAIN",
        frame: 'K',
        duration: -1,
        action: None,
        next_state: 167,
    },
    /* 168 */
    MobjState {
        sprite: "PAIN",
        frame: 'L',
        duration: -1,
        action: None,
        next_state: 168,
    },
    /* 169 */
    MobjState {
        sprite: "PAIN",
        frame: 'M',
        duration: -1,
        action: None,
        next_state: 169,
    },
    // Archvile (VILE)
    /* 170 */
    MobjState {
        sprite: "VILE",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 171,
    },
    /* 171 */
    MobjState {
        sprite: "VILE",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 170,
    },
    /* 172 */
    MobjState {
        sprite: "VILE",
        frame: 'A',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 173,
    },
    /* 173 */
    MobjState {
        sprite: "VILE",
        frame: 'B',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 174,
    },
    /* 174 */
    MobjState {
        sprite: "VILE",
        frame: 'C',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 175,
    },
    /* 175 */
    MobjState {
        sprite: "VILE",
        frame: 'D',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 176,
    },
    /* 176 */
    MobjState {
        sprite: "VILE",
        frame: 'E',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 177,
    },
    /* 177 */
    MobjState {
        sprite: "VILE",
        frame: 'F',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 172,
    },
    /* 178 */
    MobjState {
        sprite: "VILE",
        frame: 'G',
        duration: 20,
        action: Some(MonsterAction::FaceTarget),
        next_state: 179,
    },
    /* 179 */
    MobjState {
        sprite: "VILE",
        frame: 'H',
        duration: 10,
        action: Some(MonsterAction::TroopAttack),
        next_state: 180,
    },
    /* 180 */
    MobjState {
        sprite: "VILE",
        frame: 'I',
        duration: 10,
        action: None,
        next_state: 172,
    },
    /* 181 */
    MobjState {
        sprite: "VILE",
        frame: 'Q',
        duration: 5,
        action: Some(MonsterAction::Pain),
        next_state: 172,
    },
    /* 182 */
    MobjState {
        sprite: "VILE",
        frame: 'Q',
        duration: 7,
        action: Some(MonsterAction::Scream),
        next_state: 183,
    },
    /* 183 */
    MobjState {
        sprite: "VILE",
        frame: 'R',
        duration: 7,
        action: None,
        next_state: 184,
    },
    /* 184 */
    MobjState {
        sprite: "VILE",
        frame: 'S',
        duration: 7,
        action: Some(MonsterAction::Fall),
        next_state: 185,
    },
    /* 185 */
    MobjState {
        sprite: "VILE",
        frame: 'T',
        duration: 7,
        action: None,
        next_state: 186,
    },
    /* 186 */
    MobjState {
        sprite: "VILE",
        frame: 'U',
        duration: 7,
        action: None,
        next_state: 187,
    },
    /* 187 */
    MobjState {
        sprite: "VILE",
        frame: 'V',
        duration: -1,
        action: None,
        next_state: 187,
    },
    // Spider Mastermind (SPID)
    /* 188 */
    MobjState {
        sprite: "SPID",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 189,
    },
    /* 189 */
    MobjState {
        sprite: "SPID",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 188,
    },
    /* 190 */
    MobjState {
        sprite: "SPID",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 191,
    },
    /* 191 */
    MobjState {
        sprite: "SPID",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 192,
    },
    /* 192 */
    MobjState {
        sprite: "SPID",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 193,
    },
    /* 193 */
    MobjState {
        sprite: "SPID",
        frame: 'D',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 194,
    },
    /* 194 */
    MobjState {
        sprite: "SPID",
        frame: 'E',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 195,
    },
    /* 195 */
    MobjState {
        sprite: "SPID",
        frame: 'F',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 190,
    },
    /* 196 */
    MobjState {
        sprite: "SPID",
        frame: 'G',
        duration: 20,
        action: Some(MonsterAction::FaceTarget),
        next_state: 197,
    },
    /* 197 */
    MobjState {
        sprite: "SPID",
        frame: 'H',
        duration: 4,
        action: Some(MonsterAction::PosAttack),
        next_state: 198,
    },
    /* 198 */
    MobjState {
        sprite: "SPID",
        frame: 'H',
        duration: 4,
        action: Some(MonsterAction::PosAttack),
        next_state: 190,
    },
    /* 199 */
    MobjState {
        sprite: "SPID",
        frame: 'I',
        duration: 3,
        action: Some(MonsterAction::Pain),
        next_state: 190,
    },
    /* 200 */
    MobjState {
        sprite: "SPID",
        frame: 'J',
        duration: 20,
        action: Some(MonsterAction::Scream),
        next_state: 201,
    },
    /* 201 */
    MobjState {
        sprite: "SPID",
        frame: 'K',
        duration: 10,
        action: Some(MonsterAction::Fall),
        next_state: 202,
    },
    /* 202 */
    MobjState {
        sprite: "SPID",
        frame: 'L',
        duration: 10,
        action: None,
        next_state: 203,
    },
    /* 203 */
    MobjState {
        sprite: "SPID",
        frame: 'M',
        duration: 10,
        action: None,
        next_state: 204,
    },
    /* 204 */
    MobjState {
        sprite: "SPID",
        frame: 'N',
        duration: 10,
        action: None,
        next_state: 205,
    },
    /* 205 */
    MobjState {
        sprite: "SPID",
        frame: 'O',
        duration: -1,
        action: None,
        next_state: 205,
    },
    // Cyberdemon (CYBR)
    /* 206 */
    MobjState {
        sprite: "CYBR",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 207,
    },
    /* 207 */
    MobjState {
        sprite: "CYBR",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 206,
    },
    /* 208 */
    MobjState {
        sprite: "CYBR",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 209,
    },
    /* 209 */
    MobjState {
        sprite: "CYBR",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 210,
    },
    /* 210 */
    MobjState {
        sprite: "CYBR",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 211,
    },
    /* 211 */
    MobjState {
        sprite: "CYBR",
        frame: 'D',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 208,
    },
    /* 212 */
    MobjState {
        sprite: "CYBR",
        frame: 'A',
        duration: 0,
        action: None,
        next_state: 208,
    },
    /* 213 */
    MobjState {
        sprite: "CYBR",
        frame: 'A',
        duration: 0,
        action: None,
        next_state: 208,
    },
    /* 214 */
    MobjState {
        sprite: "CYBR",
        frame: 'E',
        duration: 6,
        action: Some(MonsterAction::FaceTarget),
        next_state: 215,
    },
    /* 215 */
    MobjState {
        sprite: "CYBR",
        frame: 'F',
        duration: 12,
        action: Some(MonsterAction::TroopAttack),
        next_state: 216,
    },
    /* 216 */
    MobjState {
        sprite: "CYBR",
        frame: 'E',
        duration: 12,
        action: None,
        next_state: 208,
    },
    /* 217 */
    MobjState {
        sprite: "CYBR",
        frame: 'G',
        duration: 10,
        action: Some(MonsterAction::Pain),
        next_state: 208,
    },
    /* 218 */
    MobjState {
        sprite: "CYBR",
        frame: 'H',
        duration: 10,
        action: Some(MonsterAction::Scream),
        next_state: 219,
    },
    /* 219 */
    MobjState {
        sprite: "CYBR",
        frame: 'I',
        duration: 10,
        action: None,
        next_state: 220,
    },
    /* 220 */
    MobjState {
        sprite: "CYBR",
        frame: 'J',
        duration: 10,
        action: None,
        next_state: 221,
    },
    /* 221 */
    MobjState {
        sprite: "CYBR",
        frame: 'K',
        duration: 10,
        action: None,
        next_state: 222,
    },
    /* 222 */
    MobjState {
        sprite: "CYBR",
        frame: 'L',
        duration: 10,
        action: Some(MonsterAction::Fall),
        next_state: 223,
    },
    /* 223 */
    MobjState {
        sprite: "CYBR",
        frame: 'M',
        duration: -1,
        action: None,
        next_state: 223,
    },
    // WolfSS (SSWV)
    /* 224 */
    MobjState {
        sprite: "SSWV",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 225,
    },
    /* 225 */
    MobjState {
        sprite: "SSWV",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 224,
    },
    /* 226 */
    MobjState {
        sprite: "SSWV",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 227,
    },
    /* 227 */
    MobjState {
        sprite: "SSWV",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 228,
    },
    /* 228 */
    MobjState {
        sprite: "SSWV",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 229,
    },
    /* 229 */
    MobjState {
        sprite: "SSWV",
        frame: 'D',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 226,
    },
    /* 230 */
    MobjState {
        sprite: "SSWV",
        frame: 'A',
        duration: 0,
        action: None,
        next_state: 226,
    },
    /* 231 */
    MobjState {
        sprite: "SSWV",
        frame: 'A',
        duration: 0,
        action: None,
        next_state: 226,
    },
    /* 232 */
    MobjState {
        sprite: "SSWV",
        frame: 'E',
        duration: 10,
        action: Some(MonsterAction::FaceTarget),
        next_state: 233,
    },
    /* 233 */
    MobjState {
        sprite: "SSWV",
        frame: 'F',
        duration: 10,
        action: Some(MonsterAction::PosAttack),
        next_state: 234,
    },
    /* 234 */
    MobjState {
        sprite: "SSWV",
        frame: 'G',
        duration: 10,
        action: Some(MonsterAction::PosAttack),
        next_state: 226,
    },
    /* 235 */
    MobjState {
        sprite: "SSWV",
        frame: 'H',
        duration: 3,
        action: Some(MonsterAction::Pain),
        next_state: 226,
    },
    /* 236 */
    MobjState {
        sprite: "SSWV",
        frame: 'I',
        duration: 5,
        action: Some(MonsterAction::Scream),
        next_state: 237,
    },
    /* 237 */
    MobjState {
        sprite: "SSWV",
        frame: 'J',
        duration: 5,
        action: Some(MonsterAction::Fall),
        next_state: 238,
    },
    /* 238 */
    MobjState {
        sprite: "SSWV",
        frame: 'K',
        duration: 5,
        action: None,
        next_state: 239,
    },
    /* 239 */
    MobjState {
        sprite: "SSWV",
        frame: 'L',
        duration: -1,
        action: None,
        next_state: 239,
    },
    // Shotgun Guy (SPOS) — uses separate sprites from Zombieman
    /* 240 S_SPOS_STND */
    MobjState {
        sprite: "SPOS",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 241,
    },
    /* 241 */
    MobjState {
        sprite: "SPOS",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 240,
    },
    /* 242 S_SPOS_RUN */
    MobjState {
        sprite: "SPOS",
        frame: 'A',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 243,
    },
    /* 243 */
    MobjState {
        sprite: "SPOS",
        frame: 'A',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 244,
    },
    /* 244 */
    MobjState {
        sprite: "SPOS",
        frame: 'B',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 245,
    },
    /* 245 */
    MobjState {
        sprite: "SPOS",
        frame: 'B',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 246,
    },
    /* 246 */
    MobjState {
        sprite: "SPOS",
        frame: 'C',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 247,
    },
    /* 247 */
    MobjState {
        sprite: "SPOS",
        frame: 'C',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 248,
    },
    /* 248 */
    MobjState {
        sprite: "SPOS",
        frame: 'D',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 249,
    },
    /* 249 */
    MobjState {
        sprite: "SPOS",
        frame: 'D',
        duration: 4,
        action: Some(MonsterAction::Chase),
        next_state: 242,
    },
    /* 250 S_SPOS_ATK */
    MobjState {
        sprite: "SPOS",
        frame: 'E',
        duration: 10,
        action: Some(MonsterAction::FaceTarget),
        next_state: 251,
    },
    /* 251 */
    MobjState {
        sprite: "SPOS",
        frame: 'F',
        duration: 10,
        action: Some(MonsterAction::SPosAttack),
        next_state: 252,
    },
    /* 252 */
    MobjState {
        sprite: "SPOS",
        frame: 'E',
        duration: 10,
        action: None,
        next_state: 253,
    },
    /* 253 */
    MobjState {
        sprite: "SPOS",
        frame: 'E',
        duration: 0,
        action: None,
        next_state: 242,
    },
    /* 254 S_SPOS_PAIN */
    MobjState {
        sprite: "SPOS",
        frame: 'G',
        duration: 3,
        action: None,
        next_state: 255,
    },
    /* 255 */
    MobjState {
        sprite: "SPOS",
        frame: 'G',
        duration: 3,
        action: Some(MonsterAction::Pain),
        next_state: 242,
    },
    /* 256 S_SPOS_DIE */
    MobjState {
        sprite: "SPOS",
        frame: 'H',
        duration: 5,
        action: None,
        next_state: 257,
    },
    /* 257 */
    MobjState {
        sprite: "SPOS",
        frame: 'I',
        duration: 5,
        action: Some(MonsterAction::Scream),
        next_state: 258,
    },
    /* 258 */
    MobjState {
        sprite: "SPOS",
        frame: 'J',
        duration: 5,
        action: Some(MonsterAction::Fall),
        next_state: 259,
    },
    /* 259 */
    MobjState {
        sprite: "SPOS",
        frame: 'K',
        duration: 5,
        action: None,
        next_state: 260,
    },
    /* 260 */
    MobjState {
        sprite: "SPOS",
        frame: 'L',
        duration: -1,
        action: None,
        next_state: 260,
    },
    // Demon / Pinky (SARG)
    /* 261 S_SARG_STND */
    MobjState {
        sprite: "SARG",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 262,
    },
    /* 262 */
    MobjState {
        sprite: "SARG",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 261,
    },
    /* 263 S_SARG_RUN */
    MobjState {
        sprite: "SARG",
        frame: 'A',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 264,
    },
    /* 264 */
    MobjState {
        sprite: "SARG",
        frame: 'A',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 265,
    },
    /* 265 */
    MobjState {
        sprite: "SARG",
        frame: 'B',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 266,
    },
    /* 266 */
    MobjState {
        sprite: "SARG",
        frame: 'B',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 267,
    },
    /* 267 */
    MobjState {
        sprite: "SARG",
        frame: 'C',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 268,
    },
    /* 268 */
    MobjState {
        sprite: "SARG",
        frame: 'C',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 269,
    },
    /* 269 */
    MobjState {
        sprite: "SARG",
        frame: 'D',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 270,
    },
    /* 270 */
    MobjState {
        sprite: "SARG",
        frame: 'D',
        duration: 2,
        action: Some(MonsterAction::Chase),
        next_state: 263,
    },
    /* 271 S_SARG_ATK */
    MobjState {
        sprite: "SARG",
        frame: 'E',
        duration: 8,
        action: Some(MonsterAction::FaceTarget),
        next_state: 272,
    },
    /* 272 */
    MobjState {
        sprite: "SARG",
        frame: 'F',
        duration: 8,
        action: Some(MonsterAction::FaceTarget),
        next_state: 273,
    },
    /* 273 */
    MobjState {
        sprite: "SARG",
        frame: 'G',
        duration: 8,
        action: Some(MonsterAction::SargAttack),
        next_state: 263,
    },
    /* 274 S_SARG_PAIN */
    MobjState {
        sprite: "SARG",
        frame: 'H',
        duration: 2,
        action: None,
        next_state: 275,
    },
    /* 275 */
    MobjState {
        sprite: "SARG",
        frame: 'H',
        duration: 2,
        action: Some(MonsterAction::Pain),
        next_state: 263,
    },
    /* 276 S_SARG_DIE */
    MobjState {
        sprite: "SARG",
        frame: 'I',
        duration: 8,
        action: None,
        next_state: 277,
    },
    /* 277 */
    MobjState {
        sprite: "SARG",
        frame: 'J',
        duration: 8,
        action: Some(MonsterAction::Scream),
        next_state: 278,
    },
    /* 278 */
    MobjState {
        sprite: "SARG",
        frame: 'K',
        duration: 4,
        action: None,
        next_state: 279,
    },
    /* 279 */
    MobjState {
        sprite: "SARG",
        frame: 'L',
        duration: 4,
        action: Some(MonsterAction::Fall),
        next_state: 280,
    },
    /* 280 */
    MobjState {
        sprite: "SARG",
        frame: 'M',
        duration: 4,
        action: None,
        next_state: 281,
    },
    /* 281 */
    MobjState {
        sprite: "SARG",
        frame: 'N',
        duration: -1,
        action: None,
        next_state: 281,
    },
    // Cacodemon (HEAD)
    /* 282 S_HEAD_STND */
    MobjState {
        sprite: "HEAD",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 283,
    },
    /* 283 */
    MobjState {
        sprite: "HEAD",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 282,
    },
    /* 284 S_HEAD_RUN */
    MobjState {
        sprite: "HEAD",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 285,
    },
    /* 285 */
    MobjState {
        sprite: "HEAD",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 286,
    },
    /* 286 */
    MobjState {
        sprite: "HEAD",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 287,
    },
    /* 287 */
    MobjState {
        sprite: "HEAD",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 288,
    },
    /* 288 */
    MobjState {
        sprite: "HEAD",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 289,
    },
    /* 289 */
    MobjState {
        sprite: "HEAD",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 284,
    },
    /* 290 S_HEAD_ATK */
    MobjState {
        sprite: "HEAD",
        frame: 'D',
        duration: 5,
        action: Some(MonsterAction::FaceTarget),
        next_state: 291,
    },
    /* 291 */
    MobjState {
        sprite: "HEAD",
        frame: 'E',
        duration: 5,
        action: Some(MonsterAction::FaceTarget),
        next_state: 292,
    },
    /* 292 */
    MobjState {
        sprite: "HEAD",
        frame: 'F',
        duration: 5,
        action: Some(MonsterAction::TroopAttack),
        next_state: 293,
    },
    /* 293 */
    MobjState {
        sprite: "HEAD",
        frame: 'F',
        duration: 0,
        action: None,
        next_state: 284,
    },
    /* 294 S_HEAD_PAIN */
    MobjState {
        sprite: "HEAD",
        frame: 'G',
        duration: 3,
        action: None,
        next_state: 295,
    },
    /* 295 */
    MobjState {
        sprite: "HEAD",
        frame: 'G',
        duration: 3,
        action: Some(MonsterAction::Pain),
        next_state: 284,
    },
    /* 296 S_HEAD_DIE */
    MobjState {
        sprite: "HEAD",
        frame: 'H',
        duration: 8,
        action: None,
        next_state: 297,
    },
    /* 297 */
    MobjState {
        sprite: "HEAD",
        frame: 'I',
        duration: 8,
        action: Some(MonsterAction::Scream),
        next_state: 298,
    },
    /* 298 */
    MobjState {
        sprite: "HEAD",
        frame: 'J',
        duration: 8,
        action: None,
        next_state: 299,
    },
    /* 299 */
    MobjState {
        sprite: "HEAD",
        frame: 'K',
        duration: 8,
        action: Some(MonsterAction::Fall),
        next_state: 300,
    },
    /* 300 */
    MobjState {
        sprite: "HEAD",
        frame: 'L',
        duration: 8,
        action: None,
        next_state: 301,
    },
    /* 301 */
    MobjState {
        sprite: "HEAD",
        frame: 'M',
        duration: -1,
        action: None,
        next_state: 301,
    },
    // Baron of Hell (BOSS)
    /* 302 S_BOSS_STND */
    MobjState {
        sprite: "BOSS",
        frame: 'A',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 303,
    },
    /* 303 */
    MobjState {
        sprite: "BOSS",
        frame: 'B',
        duration: 10,
        action: Some(MonsterAction::Look),
        next_state: 302,
    },
    /* 304 S_BOSS_RUN */
    MobjState {
        sprite: "BOSS",
        frame: 'A',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 305,
    },
    /* 305 */
    MobjState {
        sprite: "BOSS",
        frame: 'B',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 306,
    },
    /* 306 */
    MobjState {
        sprite: "BOSS",
        frame: 'C',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 307,
    },
    /* 307 */
    MobjState {
        sprite: "BOSS",
        frame: 'D',
        duration: 3,
        action: Some(MonsterAction::Chase),
        next_state: 304,
    },
    /* 308 S_BOSS_ATK */
    MobjState {
        sprite: "BOSS",
        frame: 'E',
        duration: 8,
        action: Some(MonsterAction::FaceTarget),
        next_state: 309,
    },
    /* 309 */
    MobjState {
        sprite: "BOSS",
        frame: 'F',
        duration: 8,
        action: Some(MonsterAction::FaceTarget),
        next_state: 310,
    },
    /* 310 */
    MobjState {
        sprite: "BOSS",
        frame: 'G',
        duration: 8,
        action: Some(MonsterAction::TroopAttack),
        next_state: 311,
    },
    /* 311 */
    MobjState {
        sprite: "BOSS",
        frame: 'G',
        duration: 0,
        action: None,
        next_state: 304,
    },
    /* 312 S_BOSS_PAIN */
    MobjState {
        sprite: "BOSS",
        frame: 'H',
        duration: 2,
        action: None,
        next_state: 313,
    },
    /* 313 */
    MobjState {
        sprite: "BOSS",
        frame: 'H',
        duration: 2,
        action: Some(MonsterAction::Pain),
        next_state: 304,
    },
    /* 314 S_BOSS_DIE */
    MobjState {
        sprite: "BOSS",
        frame: 'I',
        duration: 8,
        action: None,
        next_state: 315,
    },
    /* 315 */
    MobjState {
        sprite: "BOSS",
        frame: 'J',
        duration: 8,
        action: Some(MonsterAction::Scream),
        next_state: 316,
    },
    /* 316 */
    MobjState {
        sprite: "BOSS",
        frame: 'K',
        duration: 8,
        action: None,
        next_state: 317,
    },
    /* 317 */
    MobjState {
        sprite: "BOSS",
        frame: 'L',
        duration: 8,
        action: Some(MonsterAction::Fall),
        next_state: 318,
    },
    /* 318 */
    MobjState {
        sprite: "BOSS",
        frame: 'M',
        duration: 8,
        action: None,
        next_state: 319,
    },
    /* 319 */
    MobjState {
        sprite: "BOSS",
        frame: 'N',
        duration: -1,
        action: None,
        next_state: 319,
    },
];

pub fn get_start_state(kind: u16) -> usize {
    match kind {
        3004 => S_POSS_STND,  // Zombieman
        9 => S_SPOS_STND,     // Shotgun Guy (own SPOS sprites)
        3001 => S_TROO_STND,  // Imp
        3002 => S_SARG_STND,  // Demon (Pinky)
        3003 => S_BOSS_STND,  // Baron of Hell
        3005 => S_HEAD_STND,  // Cacodemon
        3006 => S_SKULL_STND, // Lost Soul
        2035 => S_BAR1,       // Barrel
        64 => S_VILE_STND,    // Archvile
        65 => S_CPOS_STND,    // Chaingunner
        66 => S_SKEL_STND,    // Revenant
        67 => S_FATT_STND,    // Mancubus
        68 => S_BSPI_STND,    // Arachnotron
        69 => S_BOS2_STND,    // Hell Knight
        71 => S_PAIN_STND,    // Pain Elemental
        7 => S_SPID_STND,     // Spider Mastermind
        16 => S_CYBR_STND,    // Cyberdemon
        84 => S_SSWV_STND,    // WolfSS
        _ => S_POSS_STND,     // Default to Zombieman (POSS)
    }
}
