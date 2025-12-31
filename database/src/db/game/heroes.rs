use crate::models::game::hero_group_snapshots::{
    HeroGroupSnapshot, HeroGroupSnapshotGroup, HeroGroupSnapshotInfo,
};
use crate::models::game::hero_groups::{
    HeroGroupCommon, HeroGroupEquip, HeroGroupInfo, HeroGroupType, HeroGroupTypeInfo,
};
pub use crate::models::game::heros::*;
use anyhow::Result;
use data::exceldb;
use sqlx::SqlitePool;
use std::collections::HashMap;

/// Get all heroes for a user
pub async fn get_user_heroes(pool: &SqlitePool, user_id: i64) -> Result<Vec<HeroData>> {
    let heroes = sqlx::query_as::<_, Hero>("SELECT * FROM heroes WHERE user_id = ?1 ORDER BY uid")
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    let mut result = Vec::new();

    for hero_record in heroes {
        let hero_uid = hero_record.uid;

        // Get passive skill levels
        let passive_skill_levels: Vec<i32> = sqlx::query_scalar(
            "SELECT level FROM hero_passive_skill_levels WHERE hero_uid = ?1 ORDER BY skill_index",
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        let voices: Vec<i32> =
            sqlx::query_scalar("SELECT voice_id FROM hero_voices WHERE hero_uid = ?1")
                .bind(hero_uid)
                .fetch_all(pool)
                .await?;

        let voices_heard: Vec<i32> =
            sqlx::query_scalar("SELECT voice_id FROM hero_voices_heard WHERE hero_uid = ?1")
                .bind(hero_uid)
                .fetch_all(pool)
                .await?;

        let skin_list = sqlx::query_as::<_, HeroSkin>(
            "SELECT hero_uid, skin, expire_sec FROM hero_skins WHERE hero_uid = ?1",
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        let sp_attr =
            sqlx::query_as::<_, HeroSpAttribute>("SELECT * FROM hero_sp_attrs WHERE hero_uid = ?1")
                .bind(hero_uid)
                .fetch_optional(pool)
                .await?;

        let equip_attrs = sqlx::query_as::<_, HeroEquipAttribute>(
            "SELECT * FROM hero_equip_attributes WHERE hero_uid = ?1",
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        let item_unlocks: Vec<i32> =
            sqlx::query_scalar("SELECT item_id FROM hero_item_unlocks WHERE hero_uid = ?1")
                .bind(hero_uid)
                .fetch_all(pool)
                .await?;

        let talent_cubes = sqlx::query_as::<_, HeroTalentCube>(
            "SELECT hero_uid, cube_id, direction, pos_x, pos_y FROM hero_talent_cubes WHERE hero_uid = ?1"
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        let templates = sqlx::query_as::<_, HeroTalentTemplate>(
            "SELECT id, hero_uid, template_id, name, style FROM hero_talent_templates WHERE hero_uid = ?1"
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        let mut talent_templates = Vec::new();
        for template in templates {
            let template_cubes = sqlx::query_as::<_, HeroTalentCube>(
                "SELECT 0 as hero_uid, cube_id, direction, pos_x, pos_y
                 FROM hero_talent_template_cubes WHERE template_row_id = ?1",
            )
            .bind(template.id)
            .fetch_all(pool)
            .await?;

            talent_templates.push((template, template_cubes));
        }

        let destiny_stone_unlocks: Vec<i32> = sqlx::query_scalar(
            "SELECT stone_id FROM hero_destiny_stone_unlocks WHERE hero_uid = ?1",
        )
        .bind(hero_uid)
        .fetch_all(pool)
        .await?;

        result.push(HeroData {
            record: hero_record,
            passive_skill_levels,
            voices,
            voices_heard,
            skin_list,
            sp_attr,
            equip_attrs,
            item_unlocks,
            talent_cubes,
            talent_templates,
            destiny_stone_unlocks,
        });
    }

    Ok(result)
}

pub async fn get_hero_by_hero_id(
    pool: &SqlitePool,
    user_id: i64,
    hero_id: i32,
) -> Result<HeroData> {
    let hero_record =
        sqlx::query_as::<_, Hero>("SELECT * FROM heroes WHERE user_id = ? AND hero_id = ?")
            .bind(user_id)
            .bind(hero_id)
            .fetch_one(pool)
            .await?;

    let hero_uid = hero_record.uid;

    let passive_skill_levels: Vec<i32> = sqlx::query_scalar(
        "SELECT level FROM hero_passive_skill_levels WHERE hero_uid = ? ORDER BY skill_index",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let voices: Vec<i32> =
        sqlx::query_scalar("SELECT voice_id FROM hero_voices WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    let voices_heard: Vec<i32> =
        sqlx::query_scalar("SELECT voice_id FROM hero_voices_heard WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    let skin_list = sqlx::query_as::<_, HeroSkin>(
        "SELECT hero_uid, skin, expire_sec FROM hero_skins WHERE hero_uid = ?",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let sp_attr =
        sqlx::query_as::<_, HeroSpAttribute>("SELECT * FROM hero_sp_attrs WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_optional(pool)
            .await?;

    let equip_attrs = sqlx::query_as::<_, HeroEquipAttribute>(
        "SELECT * FROM hero_equip_attributes WHERE hero_uid = ?",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get item unlocks
    let item_unlocks: Vec<i32> =
        sqlx::query_scalar("SELECT item_id FROM hero_item_unlocks WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    let talent_cubes = sqlx::query_as::<_, HeroTalentCube>(
        "SELECT hero_uid, cube_id, direction, pos_x, pos_y FROM hero_talent_cubes WHERE hero_uid = ?"
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let templates = sqlx::query_as::<_, HeroTalentTemplate>(
        "SELECT id, hero_uid, template_id, name, style FROM hero_talent_templates WHERE hero_uid = ?"
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let mut talent_templates = Vec::new();
    for template in templates {
        let template_cubes = sqlx::query_as::<_, HeroTalentCube>(
            "SELECT 0 as hero_uid, cube_id, direction, pos_x, pos_y
             FROM hero_talent_template_cubes WHERE template_row_id = ?",
        )
        .bind(template.id)
        .fetch_all(pool)
        .await?;

        talent_templates.push((template, template_cubes));
    }

    let destiny_stone_unlocks: Vec<i32> =
        sqlx::query_scalar("SELECT stone_id FROM hero_destiny_stone_unlocks WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    Ok(HeroData {
        record: hero_record,
        passive_skill_levels,
        voices,
        voices_heard,
        skin_list,
        sp_attr,
        equip_attrs,
        item_unlocks,
        talent_cubes,
        talent_templates,
        destiny_stone_unlocks,
    })
}

pub async fn get_hero_by_hero_uid(
    pool: &SqlitePool,
    user_id: i64,
    hero_uid: i32,
) -> Result<HeroData> {
    let hero_record =
        sqlx::query_as::<_, Hero>("SELECT * FROM heroes WHERE user_id = ? AND uid = ?")
            .bind(user_id)
            .bind(hero_uid)
            .fetch_one(pool)
            .await?;

    let hero_uid = hero_record.uid;
    let passive_skill_levels: Vec<i32> = sqlx::query_scalar(
        "SELECT level FROM hero_passive_skill_levels WHERE hero_uid = ? ORDER BY skill_index",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let voices: Vec<i32> =
        sqlx::query_scalar("SELECT voice_id FROM hero_voices WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    let voices_heard: Vec<i32> =
        sqlx::query_scalar("SELECT voice_id FROM hero_voices_heard WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    let skin_list = sqlx::query_as::<_, HeroSkin>(
        "SELECT hero_uid, skin, expire_sec FROM hero_skins WHERE hero_uid = ?",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let sp_attr =
        sqlx::query_as::<_, HeroSpAttribute>("SELECT * FROM hero_sp_attrs WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_optional(pool)
            .await?;

    let equip_attrs = sqlx::query_as::<_, HeroEquipAttribute>(
        "SELECT * FROM hero_equip_attributes WHERE hero_uid = ?",
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let item_unlocks: Vec<i32> =
        sqlx::query_scalar("SELECT item_id FROM hero_item_unlocks WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    let talent_cubes = sqlx::query_as::<_, HeroTalentCube>(
        "SELECT hero_uid, cube_id, direction, pos_x, pos_y FROM hero_talent_cubes WHERE hero_uid = ?"
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    // Get talent templates with their cubes
    let templates = sqlx::query_as::<_, HeroTalentTemplate>(
        "SELECT id, hero_uid, template_id, name, style FROM hero_talent_templates WHERE hero_uid = ?"
    )
    .bind(hero_uid)
    .fetch_all(pool)
    .await?;

    let mut talent_templates = Vec::new();
    for template in templates {
        let template_cubes = sqlx::query_as::<_, HeroTalentCube>(
            "SELECT 0 as hero_uid, cube_id, direction, pos_x, pos_y
             FROM hero_talent_template_cubes WHERE template_row_id = ?",
        )
        .bind(template.id)
        .fetch_all(pool)
        .await?;

        talent_templates.push((template, template_cubes));
    }

    let destiny_stone_unlocks: Vec<i32> =
        sqlx::query_scalar("SELECT stone_id FROM hero_destiny_stone_unlocks WHERE hero_uid = ?")
            .bind(hero_uid)
            .fetch_all(pool)
            .await?;

    Ok(HeroData {
        record: hero_record,
        passive_skill_levels,
        voices,
        voices_heard,
        skin_list,
        sp_attr,
        equip_attrs,
        item_unlocks,
        talent_cubes,
        talent_templates,
        destiny_stone_unlocks,
    })
}

pub async fn get_touch_count(pool: &SqlitePool, user_id: i64) -> Result<Option<i32>> {
    let count: Option<i32> =
        sqlx::query_scalar("SELECT touch_count_left FROM hero_touch_count WHERE user_id = ?1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

    Ok(count)
}

pub async fn use_touch(pool: &SqlitePool, user_id: i64) -> Result<Option<i32>> {
    let current = get_touch_count(pool, user_id).await?;
    let current = current.unwrap_or(5);

    if current <= 0 {
        return Ok(None);
    }

    let new_count = current - 1;
    sqlx::query("UPDATE hero_touch_count SET touch_count_left = ? WHERE user_id = ?")
        .bind(new_count)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(Some(new_count))
}

pub async fn reset_touch_count(pool: &SqlitePool, user_id: i64) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO hero_touch_count (user_id, touch_count_left)
        VALUES (?, 5)
        ON CONFLICT(user_id) DO UPDATE SET touch_count_left = 5
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_all_hero_skins(pool: &SqlitePool, user_id: i64) -> Result<Vec<i32>> {
    let skins: Vec<i32> =
        sqlx::query_scalar("SELECT skin_id FROM hero_all_skins WHERE user_id = ?1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;

    Ok(skins)
}

pub async fn get_birthday_info(pool: &SqlitePool, user_id: i64) -> Result<Vec<(i32, i32)>> {
    let info: Vec<(i32, i32)> =
        sqlx::query_as("SELECT hero_id, birthday_count FROM hero_birthday_info WHERE user_id = ?1")
            .bind(user_id)
            .fetch_all(pool)
            .await?;

    Ok(info)
}

pub async fn has_hero(pool: &sqlx::SqlitePool, user_id: i64, hero_id: i32) -> sqlx::Result<bool> {
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM heroes WHERE user_id = ? AND hero_id = ?",
    )
    .bind(user_id)
    .bind(hero_id)
    .fetch_one(pool)
    .await?;

    Ok(exists > 0)
}

pub async fn add_hero_duplicate(
    pool: &sqlx::SqlitePool,
    user_id: i64,
    hero_id: i32,
) -> sqlx::Result<i32> {
    sqlx::query(
        r#"
        UPDATE heroes
        SET duplicate_count = duplicate_count + 1
        WHERE user_id = ? AND hero_id = ?
        "#,
    )
    .bind(user_id)
    .bind(hero_id)
    .execute(pool)
    .await?;

    let new_count = sqlx::query_scalar::<_, i32>(
        "SELECT duplicate_count FROM heroes WHERE user_id = ? AND hero_id = ?",
    )
    .bind(user_id)
    .bind(hero_id)
    .fetch_one(pool)
    .await?;

    Ok(new_count)
}

// Create a single hero with specified parameters
pub async fn create_hero(pool: &SqlitePool, user_id: i64, hero_id: i32) -> sqlx::Result<i64> {
    let game_data = exceldb::get();
    let now = common::time::ServerTime::now_ms();

    // Get the last hero UID from database and increment
    let last_hero_uid: Option<i64> =
        sqlx::query_scalar("SELECT uid FROM heroes ORDER BY uid DESC LIMIT 1")
            .fetch_optional(pool)
            .await?;

    let hero_uid = match last_hero_uid {
        Some(uid) => uid + 1,
        None => 20000001, // Starting UID if no heroes exist
    };

    // Find the character from game data
    let character = game_data
        .character
        .iter()
        .find(|c| c.id == hero_id && c.id != 3029 && c.id != 9998)
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let hero_skin = character.skin_id;
    let rare = character.rare as usize;

    let level1_stats = game_data
        .character_level
        .iter()
        .filter(|s| s.hero_id == hero_id)
        .min_by_key(|s| s.level); // Get the lowest level entry

    let (level, hp, atk, def, mdef, technic, cri, recri, cri_dmg, cri_def, add_dmg, drop_dmg) =
        if let Some(stats) = level1_stats {
            (
                stats.level,
                stats.hp,
                stats.atk,
                stats.def,
                stats.mdef,
                stats.technic,
                stats.cri,
                stats.recri,
                stats.cri_dmg,
                stats.cri_def,
                stats.add_dmg,
                stats.drop_dmg,
            )
        } else {
            // Fallback to minimal values
            (1, 1000, 100, 100, 100, 100, 0, 0, 1300, 0, 0, 0)
        };

    let min_ranks = game_data
        .character_rank
        .iter()
        .filter(|s| s.hero_id == hero_id)
        .min_by_key(|s| s.rank);

    // Determine min rank
    let min_rank = if let Some(min) = min_ranks {
        min.rank
    } else {
        1
    };

    // Get default skin
    let default_skin = game_data
        .skin
        .iter()
        .filter(|s| s.character_id != 0)
        .filter(|s| s.character_id == hero_id)
        .max_by_key(|s| s.id)
        .map(|s| s.id)
        .unwrap_or(hero_skin);

    let destiny_data = game_data
        .character_destiny
        .iter()
        .find(|d| d.hero_id == hero_id);

    let (destiny_rank, destiny_level, destiny_stone, red_dot_type) = if let Some(d) = destiny_data {
        let rank = min_rank;
        let level = 1;
        let stone = d
            .facets_id
            .split('#')
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);
        let red_dot_type = 6;
        (rank, level, stone, red_dot_type)
    } else {
        (0, 0, 0, 0)
    };

    // Get default equipment ID (but we won't assign it)
    let equip_id = character
        .equip_rec
        .split('#')
        .next()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1501);

    // Get strengthen stats for equipment (for base stats calculation)
    let strengthen_stats = game_data
        .equip_strengthen
        .iter()
        .find(|s| s.strength_type == equip_id);

    // Calculate base stats (without equipment bonus for new heroes)
    let (
        final_hp,
        final_atk,
        final_def,
        final_mdef,
        final_technic,
        final_cri,
        final_recri,
        final_cri_dmg,
        final_cri_def,
        final_add_dmg,
        final_drop_dmg,
    ) = if let Some(_) = strengthen_stats {
        // For new heroes, don't add equipment bonuses
        (
            hp,   // No + s.hp
            atk,  // No + s.atk
            def,  // No + s.def
            mdef, // No + s.mdef
            technic, cri, recri, cri_dmg, cri_def, add_dmg, drop_dmg,
        )
    } else {
        (
            hp, atk, def, mdef, technic, cri, recri, cri_dmg, cri_def, add_dmg, drop_dmg,
        )
    };

    let extra_str = if hero_id == 3123 {
        "1003#2003"
    } else if hero_id == 3124 {
        "2#21,22|3#32,33,31"
    } else {
        ""
    };

    let min_talent_id = game_data
        .character_talent
        .iter()
        .filter(|t| t.hero_id == hero_id)
        .map(|t| t.talent_id)
        .min()
        .unwrap_or(0);

    sqlx::query(
        r#"
        INSERT INTO heroes (
            uid, user_id, hero_id, create_time,
            level, exp, rank, breakthrough, skin, faith,
            active_skill_level, ex_skill_level, is_new, talent,
            default_equip_uid, duplicate_count, use_talent_template_id,
            talent_style_unlock, talent_style_red, is_favor,
            destiny_rank, destiny_level, destiny_stone, red_dot, extra_str,
            base_hp, base_attack, base_defense, base_mdefense, base_technic,
            base_multi_hp_idx, base_multi_hp_num,
            ex_cri, ex_recri, ex_cri_dmg, ex_cri_def, ex_add_dmg, ex_drop_dmg
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
            ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
            ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30,
            ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38
        )
        "#,
    )
    .bind(hero_uid)
    .bind(user_id)
    .bind(hero_id)
    .bind(now)
    .bind(level)
    .bind(0)
    .bind(min_rank)
    .bind(0)
    .bind(default_skin)
    .bind(100)
    .bind(1)
    .bind(1)
    .bind(true)
    .bind(min_talent_id)
    .bind(0)
    .bind(0)
    .bind(1)
    .bind(0)
    .bind(0)
    .bind(false)
    .bind(destiny_rank)
    .bind(destiny_level)
    .bind(destiny_stone)
    .bind(red_dot_type)
    .bind(extra_str)
    .bind(final_hp)
    .bind(final_atk)
    .bind(final_def)
    .bind(final_mdef)
    .bind(final_technic)
    .bind(0)
    .bind(0)
    .bind(final_cri)
    .bind(final_recri)
    .bind(final_cri_dmg)
    .bind(final_cri_def)
    .bind(final_add_dmg)
    .bind(final_drop_dmg)
    .execute(pool)
    .await?;

    // Insert passive skill levels
    let max_skill_group = game_data
        .skill_passive_level
        .iter()
        .filter(|s| s.hero_id == hero_id)
        .map(|s| s.skill_group)
        .max()
        .unwrap_or(0);

    for skill_group in 1..=max_skill_group {
        let min_level = game_data
            .skill_passive_level
            .iter()
            .filter(|s| s.hero_id == hero_id && s.skill_group == skill_group)
            .map(|s| s.skill_level)
            .min()
            .unwrap_or(1);

        sqlx::query(
            "INSERT INTO hero_passive_skill_levels (hero_uid, skill_index, level) VALUES (?, ?, ?)",
        )
        .bind(hero_uid)
        .bind(skill_group - 1)
        .bind(min_level)
        .execute(pool)
        .await?;
    }

    // Insert default voices
    let character_voices: Vec<&data::exceldb::character_voice::CharacterVoice> = game_data
        .character_voice
        .iter()
        .filter(|v| v.hero_id == hero_id)
        .filter(|t| t.r#type == 9 || t.r#type == 11)
        .collect();

    for voice in &character_voices {
        sqlx::query("INSERT INTO hero_voices (hero_uid, voice_id) VALUES (?, ?)")
            .bind(hero_uid)
            .bind(voice.audio)
            .execute(pool)
            .await?;
    }

    // Add item unlocks
    for item_id in [6, 3, 7, 4] {
        sqlx::query("INSERT INTO hero_item_unlocks (hero_uid, item_id) VALUES (?, ?)")
            .bind(hero_uid)
            .bind(item_id)
            .execute(pool)
            .await?;
    }

    sqlx::query(
        r#"
        INSERT INTO hero_sp_attrs (
            hero_uid, revive, heal, absorb, defense_ignore, clutch,
            final_add_dmg, final_drop_dmg, normal_skill_rate, play_add_rate, play_drop_rate,
            dizzy_resistances, sleep_resistances, petrified_resistances, frozen_resistances,
            disarm_resistances, forbid_resistances, seal_resistances, cant_get_exskill_resistances,
            del_ex_point_resistances, stress_up_resistances, control_resilience,
            del_ex_point_resilience, stress_up_resilience, charm_resistances,
            rebound_dmg, extra_dmg, reuse_dmg, big_skill_rate, clutch_dmg
        ) VALUES (
            ?1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        )
        "#,
    )
    .bind(hero_uid)
    .execute(pool)
    .await?;

    // Birthday info
    sqlx::query(
        "INSERT INTO hero_birthday_info (user_id, hero_id, birthday_count) VALUES (?, ?, ?)",
    )
    .bind(user_id)
    .bind(hero_id)
    .bind(0)
    .execute(pool)
    .await?;

    if let Some(destiny_data) = destiny_data {
        for stone_str in destiny_data.facets_id.split('#') {
            if let Ok(stone_id) = stone_str.parse::<i32>() {
                sqlx::query(
                    "INSERT INTO hero_destiny_stone_unlocks (hero_uid, stone_id) VALUES (?, ?)",
                )
                .bind(hero_uid)
                .bind(stone_id)
                .execute(pool)
                .await?;
            }
        }
    }

    for template_id in 1..=4 {
        sqlx::query(
            "INSERT INTO hero_talent_templates (hero_uid, template_id, name, style) VALUES (?, ?, ?, ?)"
        )
        .bind(hero_uid)
        .bind(template_id)
        .bind("")
        .bind(0)
        .execute(pool)
        .await?;
    }

    update_player_hero_count(pool, user_id, rare, now).await?;

    tracing::info!(
        "Created hero {} (uid {}) for user {}",
        hero_id,
        hero_uid,
        user_id
    );

    Ok(hero_uid)
}

async fn update_player_hero_count(
    pool: &SqlitePool,
    user_id: i64,
    rarity: usize,
    now: i64,
) -> sqlx::Result<()> {
    let rarity_column = match rarity {
        1 => "hero_rare_nn_count",
        2 => "hero_rare_n_count",
        3 => "hero_rare_r_count",
        4 => "hero_rare_sr_count",
        5 => "hero_rare_ssr_count",
        _ => return Ok(()),
    };

    sqlx::query(&format!(
        r#"
        UPDATE player_info
        SET {} = {} + 1,
            updated_at = ?
        WHERE player_id = ?
        "#,
        rarity_column, rarity_column
    ))
    .bind(now)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

async fn build_hero_group_info(
    pool: &SqlitePool,
    _user_id: i64,
    db_group_id: i64,
    group_id: i32,
) -> Result<HeroGroupInfo> {
    // Get hero members
    let hero_list: Vec<i64> = sqlx::query_scalar(
        "SELECT hero_uid FROM hero_group_members WHERE hero_group_id = ? ORDER BY position",
    )
    .bind(db_group_id)
    .fetch_all(pool)
    .await?;

    // Get group details
    let group =
        sqlx::query_as::<_, HeroGroupCommon>("SELECT * FROM hero_groups_common WHERE id = ?")
            .bind(db_group_id)
            .fetch_one(pool)
            .await?;

    let equip_rows: Vec<(i32, i64)> = sqlx::query_as(
        "SELECT index_slot, equip_uid FROM hero_group_equips WHERE hero_group_id = ? ORDER BY index_slot"
    )
    .bind(db_group_id)
    .fetch_all(pool)
    .await?;

    let mut equips_map: HashMap<i32, Vec<i64>> = HashMap::new();
    for (index, equip_uid) in equip_rows {
        equips_map.entry(index).or_default().push(equip_uid);
    }

    let equips = equips_map
        .into_iter()
        .map(|(index, equip_uids)| HeroGroupEquip { index, equip_uids })
        .collect();

    let activity104_rows: Vec<(i32, i64)> = sqlx::query_as(
        "SELECT index_slot, equip_uid FROM hero_group_activity104_equips WHERE hero_group_id = ? ORDER BY index_slot"
    )
    .bind(db_group_id)
    .fetch_all(pool)
    .await?;

    let mut activity104_map: HashMap<i32, Vec<i64>> = HashMap::new();
    for (index, equip_uid) in activity104_rows {
        activity104_map.entry(index).or_default().push(equip_uid);
    }

    let activity104_equips = activity104_map
        .into_iter()
        .map(|(index, equip_uids)| HeroGroupEquip { index, equip_uids })
        .collect();

    Ok(HeroGroupInfo {
        group_id,
        hero_list,
        name: group.name,
        cloth_id: group.cloth_id,
        equips,
        activity104_equips,
        assist_boss_id: group.assist_boss_id,
    })
}

// Get ONE specific hero group (returns current active group)
pub async fn get_hero_group(
    pool: &SqlitePool,
    user_id: i64,
    group_id: i32,
) -> Result<Option<HeroGroupInfo>> {
    // Find the DB id for this group_id
    let db_group_id: Option<i64> =
        sqlx::query_scalar("SELECT id FROM hero_groups_common WHERE user_id = ? AND group_id = ?")
            .bind(user_id)
            .bind(group_id)
            .fetch_optional(pool)
            .await?;

    if let Some(db_id) = db_group_id {
        Ok(Some(
            build_hero_group_info(pool, user_id, db_id, group_id).await?,
        ))
    } else {
        Ok(None)
    }
}

// Get current active group (probably type 1's current selection)
pub async fn get_current_hero_group(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Option<HeroGroupInfo>> {
    // Get the current selected group from type 1 (main battle group)
    let selected_group: Option<i32> = sqlx::query_scalar(
        "SELECT group_id FROM hero_group_types WHERE user_id = ? AND type_id = 1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    if let Some(group_id) = selected_group {
        get_hero_group(pool, user_id, group_id).await
    } else {
        Ok(None)
    }
}

// Get ALL common hero groups
pub async fn get_hero_groups_common(pool: &SqlitePool, user_id: i64) -> Result<Vec<HeroGroupInfo>> {
    let groups = sqlx::query_as::<_, HeroGroupCommon>(
        "SELECT * FROM hero_groups_common WHERE user_id = ? ORDER BY group_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for group in groups {
        let info = build_hero_group_info(pool, user_id, group.id, group.group_id).await?;
        result.push(info);
    }

    Ok(result)
}

// Get all hero group types
pub async fn get_hero_group_types(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<HeroGroupTypeInfo>> {
    let types = sqlx::query_as::<_, HeroGroupType>(
        "SELECT * FROM hero_group_types WHERE user_id = ? ORDER BY type_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for type_info in types {
        let group_info = if let Some(group_id) = type_info.group_id {
            get_hero_group(pool, user_id, group_id).await?
        } else {
            None
        };

        result.push(HeroGroupTypeInfo {
            type_id: type_info.type_id,
            current_select: type_info.current_select,
            group_info,
        });
    }

    Ok(result)
}

pub async fn set_hero_group_equip(
    pool: &SqlitePool,
    user_id: i64,
    group_id: i32,
    index: i32,
    equip_uids: Vec<i64>,
) -> Result<()> {
    let db_group_id: Option<i64> =
        sqlx::query_scalar("SELECT id FROM hero_groups_common WHERE user_id = ? AND group_id = ?")
            .bind(user_id)
            .bind(group_id)
            .fetch_optional(pool)
            .await?;

    let db_group_id = db_group_id.ok_or_else(|| anyhow::anyhow!("Hero group not found"))?;

    sqlx::query("DELETE FROM hero_group_equips WHERE hero_group_id = ? AND index_slot = ?")
        .bind(db_group_id)
        .bind(index)
        .execute(pool)
        .await?;

    // Insert new equips
    for equip_uid in equip_uids {
        sqlx::query(
            "INSERT INTO hero_group_equips (hero_group_id, index_slot, equip_uid) VALUES (?, ?, ?)",
        )
        .bind(db_group_id)
        .bind(index)
        .bind(equip_uid)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn build_snapshot_group_info(
    pool: &SqlitePool,
    snapshot_group_id: i64,
    group_id: i32,
) -> Result<HeroGroupInfo> {
    // Get group details
    let group = sqlx::query_as::<_, HeroGroupSnapshotGroup>(
        "SELECT * FROM hero_group_snapshot_groups WHERE id = ?",
    )
    .bind(snapshot_group_id)
    .fetch_one(pool)
    .await?;

    // Get hero members
    let hero_list: Vec<i64> = sqlx::query_scalar(
        "SELECT hero_uid FROM hero_group_snapshot_members WHERE snapshot_group_id = ? ORDER BY position"
    )
    .bind(snapshot_group_id)
    .fetch_all(pool)
    .await?;

    // Get equips
    let equip_rows: Vec<(i32, i64)> = sqlx::query_as(
        "SELECT index_slot, equip_uid FROM hero_group_snapshot_equips WHERE snapshot_group_id = ? ORDER BY index_slot"
    )
    .bind(snapshot_group_id)
    .fetch_all(pool)
    .await?;

    let mut equips_map: HashMap<i32, Vec<i64>> = HashMap::new();
    for (index, equip_uid) in equip_rows {
        equips_map.entry(index).or_default().push(equip_uid);
    }

    let equips = equips_map
        .into_iter()
        .map(|(index, equip_uids)| HeroGroupEquip { index, equip_uids })
        .collect();

    // Get activity104 equips
    let activity104_rows: Vec<(i32, i64)> = sqlx::query_as(
        "SELECT index_slot, equip_uid FROM hero_group_snapshot_activity104_equips WHERE snapshot_group_id = ? ORDER BY index_slot"
    )
    .bind(snapshot_group_id)
    .fetch_all(pool)
    .await?;

    let mut activity104_map: HashMap<i32, Vec<i64>> = HashMap::new();
    for (index, equip_uid) in activity104_rows {
        activity104_map.entry(index).or_default().push(equip_uid);
    }

    let activity104_equips = activity104_map
        .into_iter()
        .map(|(index, equip_uids)| HeroGroupEquip { index, equip_uids })
        .collect();

    tracing::info!(
        "Loaded group {}: sub {} {} heroes: {:?}",
        group_id,
        snapshot_group_id,
        hero_list.len(),
        hero_list
    );

    Ok(HeroGroupInfo {
        group_id,
        hero_list,
        name: group.name,
        cloth_id: group.cloth_id,
        equips,
        activity104_equips,
        assist_boss_id: group.assist_boss_id,
    })
}

// Get all snapshots for a user
pub async fn get_hero_group_snapshots(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<HeroGroupSnapshotInfo>> {
    let snapshots = sqlx::query_as::<_, HeroGroupSnapshot>(
        "SELECT * FROM hero_group_snapshots WHERE user_id = ? ORDER BY snapshot_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();

    for snapshot in snapshots {
        // Get all groups in this snapshot
        let snapshot_groups = sqlx::query_as::<_, HeroGroupSnapshotGroup>(
            "SELECT * FROM hero_group_snapshot_groups WHERE snapshot_id = ? ORDER BY group_id",
        )
        .bind(snapshot.id)
        .fetch_all(pool)
        .await?;

        let mut hero_group_snapshots = Vec::new();
        for group in snapshot_groups {
            let info = build_snapshot_group_info(pool, group.id, group.group_id).await?;
            hero_group_snapshots.push(info);
        }

        // Get sort sub IDs
        let sort_sub_ids: Vec<i32> = sqlx::query_scalar(
            "SELECT sub_id FROM hero_group_snapshot_sort_ids WHERE snapshot_id = ? ORDER BY sort_order"
        )
        .bind(snapshot.id)
        .fetch_all(pool)
        .await?;

        result.push(HeroGroupSnapshotInfo {
            snapshot_id: snapshot.snapshot_id,
            hero_group_snapshots,
            sort_sub_ids,
        });
    }

    Ok(result)
}

// Get a specific snapshot by ID
pub async fn get_hero_group_snapshot(
    pool: &SqlitePool,
    user_id: i64,
    snapshot_id: i32,
) -> Result<Option<HeroGroupSnapshotInfo>> {
    let snapshot = sqlx::query_as::<_, HeroGroupSnapshot>(
        "SELECT * FROM hero_group_snapshots WHERE user_id = ? AND snapshot_id = ?",
    )
    .bind(user_id)
    .bind(snapshot_id)
    .fetch_optional(pool)
    .await?;

    let Some(snapshot) = snapshot else {
        return Ok(None);
    };

    // Get all groups in this snapshot
    let snapshot_groups = sqlx::query_as::<_, HeroGroupSnapshotGroup>(
        "SELECT * FROM hero_group_snapshot_groups WHERE snapshot_id = ? ORDER BY group_id",
    )
    .bind(snapshot.id)
    .fetch_all(pool)
    .await?;

    let mut hero_group_snapshots = Vec::new();
    for group in snapshot_groups {
        let info = build_snapshot_group_info(pool, group.id, group.group_id).await?;
        hero_group_snapshots.push(info);
    }

    // Get sort sub IDs
    let sort_sub_ids: Vec<i32> = sqlx::query_scalar(
        "SELECT sub_id FROM hero_group_snapshot_sort_ids WHERE snapshot_id = ? ORDER BY sort_order",
    )
    .bind(snapshot.id)
    .fetch_all(pool)
    .await?;

    Ok(Some(HeroGroupSnapshotInfo {
        snapshot_id: snapshot.snapshot_id,
        hero_group_snapshots,
        sort_sub_ids,
    }))
}

// Save a snapshot from current hero groups
pub async fn save_hero_group_snapshot(
    pool: &SqlitePool,
    user_id: i64,
    snapshot_id: i32,
    groups: Vec<HeroGroupInfo>,
    sort_sub_ids: Vec<i32>,
) -> Result<()> {
    let now = common::time::ServerTime::now_ms();

    // Create or update snapshot
    sqlx::query(
        "INSERT INTO hero_group_snapshots (user_id, snapshot_id, created_at, updated_at)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(user_id, snapshot_id) DO UPDATE SET updated_at = excluded.updated_at",
    )
    .bind(user_id)
    .bind(snapshot_id)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    // Get the snapshot DB ID
    let db_snapshot_id: i64 = sqlx::query_scalar(
        "SELECT id FROM hero_group_snapshots WHERE user_id = ? AND snapshot_id = ?",
    )
    .bind(user_id)
    .bind(snapshot_id)
    .fetch_one(pool)
    .await?;

    // Save each group (only delete the specific group being updated)
    for group in groups {
        // Delete only THIS specific group and its related data
        let existing_group: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM hero_group_snapshot_groups
             WHERE snapshot_id = ? AND group_id = ?",
        )
        .bind(db_snapshot_id)
        .bind(group.group_id)
        .fetch_optional(pool)
        .await?;

        if let Some(old_group_id) = existing_group {
            // Delete old members
            sqlx::query("DELETE FROM hero_group_snapshot_members WHERE snapshot_group_id = ?")
                .bind(old_group_id)
                .execute(pool)
                .await?;

            // Delete old equips
            sqlx::query("DELETE FROM hero_group_snapshot_equips WHERE snapshot_group_id = ?")
                .bind(old_group_id)
                .execute(pool)
                .await?;

            // Delete old activity104 equips
            sqlx::query(
                "DELETE FROM hero_group_snapshot_activity104_equips WHERE snapshot_group_id = ?",
            )
            .bind(old_group_id)
            .execute(pool)
            .await?;

            // Delete the group itself
            sqlx::query("DELETE FROM hero_group_snapshot_groups WHERE id = ?")
                .bind(old_group_id)
                .execute(pool)
                .await?;
        }

        // Insert new group
        let group_result = sqlx::query(
            "INSERT INTO hero_group_snapshot_groups (snapshot_id, group_id, name, cloth_id, assist_boss_id)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(db_snapshot_id)
        .bind(group.group_id)
        .bind(&group.name)
        .bind(group.cloth_id)
        .bind(group.assist_boss_id)
        .execute(pool)
        .await?;

        let snapshot_group_id = group_result.last_insert_rowid();

        // Save heroes
        for (position, hero_uid) in group.hero_list.iter().enumerate() {
            sqlx::query(
                "INSERT INTO hero_group_snapshot_members (snapshot_group_id, hero_uid, position)
                 VALUES (?, ?, ?)",
            )
            .bind(snapshot_group_id)
            .bind(hero_uid)
            .bind(position as i32)
            .execute(pool)
            .await?;
        }

        // Save equips
        for equip in &group.equips {
            for equip_uid in &equip.equip_uids {
                sqlx::query(
                    "INSERT INTO hero_group_snapshot_equips (snapshot_group_id, index_slot, equip_uid)
                     VALUES (?, ?, ?)"
                )
                .bind(snapshot_group_id)
                .bind(equip.index)
                .bind(equip_uid)
                .execute(pool)
                .await?;
            }
        }

        // Save activity104 equips
        for equip in &group.activity104_equips {
            for equip_uid in &equip.equip_uids {
                sqlx::query(
                    "INSERT INTO hero_group_snapshot_activity104_equips (snapshot_group_id, index_slot, equip_uid)
                     VALUES (?, ?, ?)"
                )
                .bind(snapshot_group_id)
                .bind(equip.index)
                .bind(equip_uid)
                .execute(pool)
                .await?;
            }
        }
    }

    // Update sort IDs, merge with existing ones
    let existing_sort_ids: Vec<i32> = sqlx::query_scalar(
        "SELECT sub_id FROM hero_group_snapshot_sort_ids
         WHERE snapshot_id = ? ORDER BY sort_order",
    )
    .bind(db_snapshot_id)
    .fetch_all(pool)
    .await?;

    // Merge: add new sort_sub_ids if not already present
    let mut merged_sort_ids = existing_sort_ids;
    for sub_id in &sort_sub_ids {
        if !merged_sort_ids.contains(sub_id) {
            merged_sort_ids.push(*sub_id);
        }
    }

    // Replace all sort IDs
    sqlx::query("DELETE FROM hero_group_snapshot_sort_ids WHERE snapshot_id = ?")
        .bind(db_snapshot_id)
        .execute(pool)
        .await?;

    for (order, sub_id) in merged_sort_ids.iter().enumerate() {
        sqlx::query(
            "INSERT INTO hero_group_snapshot_sort_ids (snapshot_id, sub_id, sort_order)
             VALUES (?, ?, ?)",
        )
        .bind(db_snapshot_id)
        .bind(sub_id)
        .bind(order as i32)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn sync_snapshot_to_common(
    pool: &SqlitePool,
    user_id: i64,
    group: &HeroGroupInfo,
) -> Result<()> {
    let db_group_id: i64 =
        sqlx::query_scalar("SELECT id FROM hero_groups_common WHERE user_id = ? AND group_id = ?")
            .bind(user_id)
            .bind(group.group_id)
            .fetch_one(pool)
            .await?;

    // Replace heroes
    sqlx::query("DELETE FROM hero_group_members WHERE hero_group_id = ?")
        .bind(db_group_id)
        .execute(pool)
        .await?;

    for (pos, hero_uid) in group.hero_list.iter().enumerate() {
        sqlx::query(
            "INSERT INTO hero_group_members (hero_group_id, hero_uid, position)
             VALUES (?, ?, ?)",
        )
        .bind(db_group_id)
        .bind(hero_uid)
        .bind(pos as i32)
        .execute(pool)
        .await?;
    }

    // Replace equips
    sqlx::query("DELETE FROM hero_group_equips WHERE hero_group_id = ?")
        .bind(db_group_id)
        .execute(pool)
        .await?;

    for equip in &group.equips {
        for uid in &equip.equip_uids {
            sqlx::query(
                "INSERT INTO hero_group_equips (hero_group_id, index_slot, equip_uid)
                 VALUES (?, ?, ?)",
            )
            .bind(db_group_id)
            .bind(equip.index)
            .bind(uid)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
