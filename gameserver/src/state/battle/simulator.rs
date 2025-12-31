use anyhow::Result;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sonettobuf::{
    ActEffect, BeginRoundOper, CardInfo, Fight, FightEntityInfo, FightRound, FightStep, fight_step,
};
use std::collections::HashMap;

use crate::state::battle::{effects::effect_types::EffectType, skill_executor::SkillExecutor};

pub struct BattleSimulator {
    fight: Fight,
    rng: StdRng,
}

impl BattleSimulator {
    pub fn new(fight: Fight) -> Self {
        let seed = chrono::Utc::now().timestamp_millis() as u64;
        Self {
            fight,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub async fn process_round(
        &mut self,
        operations: Vec<BeginRoundOper>,
        current_deck: Vec<CardInfo>,
    ) -> Result<FightRound> {
        let mut state = RoundState::from_fight(&self.fight)?;
        state.player_deck = current_deck.clone();

        let mut steps = Vec::new();

        // STEP 1: Round begin effect
        steps.push(self.create_round_begin_step());

        // STEP 2: Trigger passive skills at round start
        let passive_steps = self.trigger_round_start_passives(&mut state).await?;
        steps.extend(passive_steps);

        // STEP 3: Deal/refresh cards
        steps.push(self.create_card_refresh_step(&state));

        // STEP 4: Process player operations
        for oper in operations {
            let step = self.execute_operation(&mut state, oper).await?;
            if step.act_type.is_some() && step.act_type.unwrap() != 0 {
                steps.push(step);
            }
        }

        // STEP 5: Execute AI turn
        if !state.is_finish {
            let ai_steps = self.execute_ai_turn(&mut state).await?;
            steps.extend(ai_steps);
        }

        // STEP 6: Check battle end
        state.is_finish = self.check_battle_end(&state);

        // STEP 7: Round end effect
        steps.push(self.create_round_end_step());

        // STEP 8: Generate next round begin steps
        let next_round_steps = self.generate_next_round_steps(&state);

        state.move_num += 1;

        Ok(self.build_round_response(steps, next_round_steps, state, current_deck))
    }

    fn create_round_begin_step(&self) -> FightStep {
        FightStep {
            act_type: Some(fight_step::ActType::Effect.into()),
            from_id: Some(0),
            to_id: Some(0),
            act_id: Some(0),
            act_effect: vec![self.create_simple_effect(0, EffectType::DealCard1 as i32, 0, 0)],
            card_index: Some(0),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        }
    }

    fn create_card_refresh_step(&self, state: &RoundState) -> FightStep {
        FightStep {
            act_type: Some(fight_step::ActType::Effect.into()),
            from_id: Some(0),
            to_id: Some(0),
            act_id: Some(0),
            act_effect: vec![
                // Show current deck
                ActEffect {
                    target_id: Some(0),
                    effect_type: Some(EffectType::CardsPush as i32),
                    effect_num: Some(0),
                    card_info_list: state.player_deck.clone(),
                    team_type: Some(0),
                    buff: None,
                    entity: None,
                    config_effect: Some(0),
                    buff_act_id: Some(0),
                    reserve_id: Some(0),
                    reserve_str: Some(String::new()),
                    summoned: None,
                    magic_circle: None,
                    card_info: None,
                    fight_step: None,
                    assist_boss_info: None,
                    effect_num1: Some(0),
                    emitter_info: None,
                    player_finisher_info: None,
                    power_info: None,
                    card_heat_value: None,
                    fight_tasks: vec![],
                    fight: None,
                    buff_act_info: None,
                    hurt_info: None,
                },
                // Update power display
                ActEffect {
                    target_id: Some(0),
                    effect_type: Some(EffectType::CardDeckNum as i32),
                    effect_num: Some(state.power),
                    card_info_list: vec![],
                    team_type: Some(1),
                    buff: None,
                    entity: None,
                    config_effect: Some(0),
                    buff_act_id: Some(0),
                    reserve_id: Some(0),
                    reserve_str: Some(String::new()),
                    summoned: None,
                    magic_circle: None,
                    card_info: None,
                    fight_step: None,
                    assist_boss_info: None,
                    effect_num1: Some(0),
                    emitter_info: None,
                    player_finisher_info: None,
                    power_info: None,
                    card_heat_value: None,
                    fight_tasks: vec![],
                    fight: None,
                    buff_act_info: None,
                    hurt_info: None,
                },
            ],
            card_index: Some(0),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        }
    }

    async fn trigger_round_start_passives(
        &mut self,
        state: &mut RoundState,
    ) -> Result<Vec<FightStep>> {
        let steps = Vec::new();

        // For now, just create placeholder passive triggers
        // TODO: Actually execute passive skills that trigger on round start
        for (uid, entity) in &state.entities {
            if *uid > 0 && !entity.passive_skill.is_empty() {
                // Check if any passives should trigger
                // Most passives are stat modifiers, not round-start triggers
                // We'll implement actual passive execution later
            }
        }

        Ok(steps)
    }

    fn generate_next_round_steps(&self, state: &RoundState) -> Vec<FightStep> {
        let mut steps = Vec::new();

        // Add buff duration update steps
        // This shows buffs counting down at end of round
        for (_uid, entity) in &state.entities {
            if !entity.buffs.is_empty() {
                // TODO: Create steps for each buff that needs duration update
                // effectType: 234 for buff duration changes
            }
        }

        // Add card refresh for next round
        steps.push(FightStep {
            act_type: Some(fight_step::ActType::Effect.into()),
            from_id: Some(0),
            to_id: Some(0),
            act_id: Some(0),
            act_effect: vec![
                self.create_simple_effect(0, 59, 0, 0), // ROUNDBEGIN for next round
            ],
            card_index: Some(0),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        });

        steps
    }

    async fn execute_operation(
        &mut self,
        state: &mut RoundState,
        oper: BeginRoundOper,
    ) -> Result<FightStep> {
        let oper_type = oper.oper_type.unwrap_or(0);

        match oper_type {
            1 => self.play_card(state, oper).await,
            2 => {
                if oper.to_id.is_some() && oper.to_id.unwrap() != 0 {
                    self.play_card(state, oper).await
                } else {
                    self.select_card(state, oper).await
                }
            }
            3 => self.change_hero(state, oper).await,
            4 => self.end_turn(state).await,
            _ => Ok(FightStep::default()),
        }
    }

    async fn select_card(
        &mut self,
        _state: &mut RoundState,
        oper: BeginRoundOper,
    ) -> Result<FightStep> {
        let card_index = oper.param1.unwrap_or(0);
        tracing::info!("Selecting card at index {}", card_index);

        Ok(FightStep {
            act_type: Some(fight_step::ActType::Effect.into()),
            from_id: Some(0),
            to_id: Some(0),
            act_id: Some(0),
            act_effect: vec![self.create_simple_effect(
                0,
                EffectType::AddHandCard as i32,
                card_index,
                1,
            )],
            card_index: Some(card_index),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        })
    }

    async fn play_card(
        &mut self,
        state: &mut RoundState,
        oper: BeginRoundOper,
    ) -> Result<FightStep> {
        let card_index = oper.param1.unwrap_or(0) as usize;
        let target_uid = oper.to_id.unwrap_or(0);

        let card = state.player_deck.get(card_index).cloned();
        let Some(card) = card else {
            tracing::warn!("Card index {} not found in deck", card_index);
            return Ok(FightStep::default());
        };

        let caster_uid = card.uid.unwrap_or(0);
        let skill_id = card.skill_id.unwrap_or(0);

        tracing::info!(
            "Playing card {}: caster={}, skill={}, target={}",
            card_index,
            caster_uid,
            skill_id,
            target_uid
        );

        let executor = SkillExecutor::new(state.entities.clone());
        let effects = executor.execute_skill(caster_uid, target_uid, skill_id)?;

        for effect in &effects {
            self.apply_effect_to_state(state, effect)?;
        }

        state.used_cards.push(card_index as i32);
        state.act_point = (state.act_point - 1).max(0);

        Ok(FightStep {
            act_type: Some(fight_step::ActType::Skill.into()),
            from_id: Some(caster_uid),
            to_id: Some(target_uid),
            act_id: Some(skill_id),
            act_effect: effects,
            card_index: Some(card_index as i32),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        })
    }

    fn apply_effect_to_state(&mut self, state: &mut RoundState, effect: &ActEffect) -> Result<()> {
        let effect_type = EffectType::from_i32(effect.effect_type.unwrap_or(0));
        let target_id = effect.target_id.unwrap_or(0);
        let value = effect.effect_num.unwrap_or(0);

        match effect_type {
            EffectType::Damage | EffectType::Crit => {
                if let Some(target) = state.get_entity_mut(target_id) {
                    let current_hp = target.current_hp.unwrap_or(0);
                    let new_hp = (current_hp - value).max(0);
                    target.current_hp = Some(new_hp);

                    tracing::info!(
                        "Applied damage: target={}, damage={}, hp: {} -> {}",
                        target_id,
                        value,
                        current_hp,
                        new_hp
                    );
                }
            }
            EffectType::Heal | EffectType::HealCrit => {
                if let Some(target) = state.get_entity_mut(target_id) {
                    let current_hp = target.current_hp.unwrap_or(0);
                    let max_hp = target
                        .attr
                        .as_ref()
                        .and_then(|a| a.hp)
                        .unwrap_or(current_hp);
                    let new_hp = (current_hp + value).min(max_hp);
                    target.current_hp = Some(new_hp);

                    tracing::info!(
                        "Applied heal: target={}, heal={}, hp: {} -> {}",
                        target_id,
                        value,
                        current_hp,
                        new_hp
                    );
                }
            }
            EffectType::BuffAdd => {
                tracing::info!("Applied buff: target={}, buff_id={}", target_id, value);
            }
            _ => {
                tracing::debug!("Unhandled effect type: {:?}", effect_type);
            }
        }

        Ok(())
    }

    async fn execute_ai_turn(&mut self, state: &mut RoundState) -> Result<Vec<FightStep>> {
        let mut steps = Vec::new();

        let enemies: Vec<i64> = state
            .entities
            .iter()
            .filter(|(uid, entity)| **uid < 0 && entity.current_hp.unwrap_or(0) > 0)
            .map(|(uid, _)| *uid)
            .collect();

        if enemies.is_empty() {
            return Ok(steps);
        }

        let players: Vec<i64> = state
            .entities
            .iter()
            .filter(|(uid, entity)| **uid > 0 && entity.current_hp.unwrap_or(0) > 0)
            .map(|(uid, _)| *uid)
            .collect();

        if players.is_empty() {
            return Ok(steps);
        }

        for enemy_uid in enemies {
            let enemy = state.get_entity(enemy_uid);
            let Some(enemy) = enemy else { continue };

            let skill_id = enemy.skill_group1.first().copied().unwrap_or(0);
            if skill_id == 0 {
                continue;
            }

            let target_uid = *players.get(self.rng.gen_range(0..players.len())).unwrap();

            tracing::info!(
                "AI: Enemy {} uses skill {} on target {}",
                enemy_uid,
                skill_id,
                target_uid
            );

            tracing::info!("Looking up skill {} in skill table", skill_id);
            let game_data = data::exceldb::get();
            let skill = game_data.skill.iter().find(|s| s.id == skill_id);

            let skill_effect_id = if let Some(skill) = skill {
                tracing::info!("  Found skill: skillEffect={}", skill.skill_effect);
                skill.skill_effect
            } else {
                tracing::warn!("  Skill {} not found in skill table!", skill_id);
                continue;
            };

            if skill_effect_id == 0 {
                tracing::warn!("  Skill {} has no skillEffect!", skill_id);
                continue;
            }

            let ai_card = CardInfo {
                uid: Some(enemy_uid),
                skill_id: Some(skill_id),
                card_effect: Some(0),
                temp_card: Some(false),
                enchants: vec![],
                card_type: Some(0),
                hero_id: enemy.model_id,
                status: Some(0),
                target_uid: Some(target_uid),
                extra_info: None,
                energy: Some(0),
                extra_infos: vec![],
                area_red_or_blue: Some(0),
                heat_id: Some(0),
            };
            state.ai_cards.push(ai_card);

            let executor = SkillExecutor::new(state.entities.clone());
            let effects = executor.execute_skill(enemy_uid, target_uid, skill_effect_id)?;

            tracing::info!("AI skill {} generated {} effects", skill_id, effects.len());
            if effects.is_empty() {
                tracing::warn!("AI skill {} produced NO effects!", skill_id);
            } else {
                for effect in &effects {
                    tracing::info!(
                        "  AI effect: type={}, target={}, value={}",
                        effect.effect_type.unwrap_or(0),
                        effect.target_id.unwrap_or(0),
                        effect.effect_num.unwrap_or(0)
                    );
                }
            }

            for effect in &effects {
                self.apply_effect_to_state(state, effect)?;
            }

            steps.push(FightStep {
                act_type: Some(fight_step::ActType::Skill.into()),
                from_id: Some(enemy_uid),
                to_id: Some(target_uid),
                act_id: Some(skill_id),
                act_effect: effects,
                card_index: Some(0),
                support_hero_id: Some(0),
                fake_timeline: Some(false),
            });
        }

        Ok(steps)
    }

    fn check_battle_end(&self, state: &RoundState) -> bool {
        let enemies_alive = state
            .entities
            .iter()
            .any(|(uid, entity)| *uid < 0 && entity.current_hp.unwrap_or(0) > 0);

        let heroes_alive = state
            .entities
            .iter()
            .any(|(uid, entity)| *uid > 0 && entity.current_hp.unwrap_or(0) > 0);

        !enemies_alive || !heroes_alive
    }

    async fn change_hero(
        &mut self,
        _state: &mut RoundState,
        _oper: BeginRoundOper,
    ) -> Result<FightStep> {
        tracing::info!("Changing hero");
        Ok(FightStep::default())
    }

    async fn end_turn(&mut self, _state: &mut RoundState) -> Result<FightStep> {
        tracing::info!("Ending turn");
        Ok(FightStep::default())
    }

    fn create_round_end_step(&self) -> FightStep {
        FightStep {
            act_type: Some(fight_step::ActType::Effect.into()),
            from_id: Some(0),
            to_id: Some(0),
            act_id: Some(0),
            act_effect: vec![self.create_simple_effect(0, EffectType::RoundEnd as i32, 0, 0)],
            card_index: Some(0),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        }
    }

    fn create_simple_effect(
        &self,
        target_id: i64,
        effect_type: i32,
        value: i32,
        team_type: i32,
    ) -> ActEffect {
        ActEffect {
            target_id: Some(target_id),
            effect_type: Some(effect_type),
            effect_num: Some(value),
            buff: None,
            entity: None,
            config_effect: Some(0),
            buff_act_id: Some(0),
            reserve_id: Some(0),
            reserve_str: Some(String::new()),
            summoned: None,
            magic_circle: None,
            card_info: None,
            card_info_list: vec![],
            team_type: Some(team_type),
            fight_step: None,
            assist_boss_info: None,
            effect_num1: Some(0),
            emitter_info: None,
            player_finisher_info: None,
            power_info: None,
            card_heat_value: None,
            fight_tasks: vec![],
            fight: None,
            buff_act_info: None,
            hurt_info: None,
        }
    }

    fn build_sp_attributes(&self, state: &RoundState) -> Vec<sonettobuf::FightHeroSpAttributeInfo> {
        state
            .entities
            .values()
            .filter(|e| e.uid.unwrap_or(0) < 0) // Only enemies
            .map(|e| sonettobuf::FightHeroSpAttributeInfo {
                uid: e.uid,
                attribute: Some(sonettobuf::HeroSpAttribute {
                    revive: Some(0),
                    heal: Some(0),
                    absorb: Some(0),
                    defense_ignore: Some(0),
                    clutch: Some(0),
                    final_add_dmg: Some(0),
                    final_drop_dmg: Some(0),
                    normal_skill_rate: Some(0),
                    play_add_rate: Some(0),
                    play_drop_rate: Some(0),
                    dizzy_resistances: Some(0),
                    sleep_resistances: Some(0),
                    petrified_resistances: Some(0),
                    frozen_resistances: Some(0),
                    disarm_resistances: Some(0),
                    forbid_resistances: Some(0),
                    seal_resistances: Some(0),
                    cant_get_exskill_resistances: Some(0),
                    del_ex_point_resistances: Some(0),
                    stress_up_resistances: Some(0),
                    control_resilience: Some(0),
                    del_ex_point_resilience: Some(0),
                    stress_up_resilience: Some(0),
                    charm_resistances: Some(0),
                    rebound_dmg: Some(0),
                    extra_dmg: Some(0),
                    reuse_dmg: Some(0),
                    big_skill_rate: Some(0),
                    clutch_dmg: Some(0),
                }),
            })
            .collect()
    }

    fn build_round_response(
        &self,
        steps: Vec<FightStep>,
        next_round_steps: Vec<FightStep>,
        state: RoundState,
        before_cards: Vec<CardInfo>,
    ) -> FightRound {
        let ex_point_info = state.build_ex_point_info();

        FightRound {
            fight_step: steps,
            act_point: Some(state.act_point),
            is_finish: Some(state.is_finish),
            move_num: Some(state.move_num + 1),
            ex_point_info,
            ai_use_cards: state.ai_cards.clone(),
            power: Some(state.power),
            skill_infos: vec![],
            before_cards1: before_cards, // Cards at start of round
            team_a_cards1: state.player_deck.clone(), // Cards after playing
            before_cards2: vec![],
            team_a_cards2: vec![],
            next_round_begin_step: next_round_steps,
            use_card_list: state.used_cards.clone(),
            cur_round: Some(state.round_num),
            hero_sp_attributes: self.build_sp_attributes(&state),
            last_change_hero_uid: Some(0),
        }
    }
}

struct RoundState {
    act_point: i32,
    power: i32,
    player_deck: Vec<CardInfo>,
    ai_cards: Vec<CardInfo>,
    entities: HashMap<i64, FightEntityInfo>,
    used_cards: Vec<i32>,
    round_num: i32,
    move_num: i32,
    is_finish: bool,
}

impl RoundState {
    fn from_fight(fight: &Fight) -> Result<Self> {
        let mut entities = HashMap::new();

        if let Some(ref attacker) = fight.attacker {
            for entity in &attacker.entitys {
                if let Some(uid) = entity.uid {
                    entities.insert(uid, entity.clone());
                }
            }
        }

        if let Some(ref defender) = fight.defender {
            for entity in &defender.entitys {
                if let Some(uid) = entity.uid {
                    entities.insert(uid, entity.clone());
                }
            }
        }

        Ok(Self {
            act_point: 4,
            power: 15,
            player_deck: vec![],
            ai_cards: vec![],
            entities,
            used_cards: vec![],
            round_num: fight.cur_round.unwrap_or(1),
            move_num: 0,
            is_finish: false,
        })
    }

    fn get_entity(&self, uid: i64) -> Option<&FightEntityInfo> {
        self.entities.get(&uid)
    }

    fn get_entity_mut(&mut self, uid: i64) -> Option<&mut FightEntityInfo> {
        self.entities.get_mut(&uid)
    }

    fn build_ex_point_info(&self) -> Vec<sonettobuf::FightExPointInfo> {
        self.entities
            .values()
            .map(|entity| sonettobuf::FightExPointInfo {
                uid: entity.uid,
                ex_point: entity.ex_point,
                power_infos: entity.power_infos.clone(),
                current_hp: entity.current_hp,
                ex_point_type: entity.ex_point_type,
            })
            .collect()
    }
}
