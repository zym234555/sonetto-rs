use super::effects::effect_types::EffectType;
use sonettobuf::{ActEffect, BuffInfo, FightStep, fight_step};

pub struct FightStepBuilder {
    act_type: fight_step::ActType,
    from_id: i64,
    to_id: i64,
    act_id: i32,
    effects: Vec<ActEffect>,
}

#[allow(dead_code)]
impl FightStepBuilder {
    pub fn new_effect() -> Self {
        Self {
            act_type: fight_step::ActType::Effect,
            from_id: 0,
            to_id: 0,
            act_id: 0,
            effects: Vec::new(),
        }
    }

    pub fn new_skill(from_id: i64, to_id: i64, skill_id: i32) -> Self {
        Self {
            act_type: fight_step::ActType::Skill,
            from_id,
            to_id,
            act_id: skill_id,
            effects: Vec::new(),
        }
    }

    pub fn add_nested_step(mut self, step: FightStep) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(0),
            effect_type: Some(EffectType::FightStep.to_i32()),
            fight_step: Some(step),
            ..Default::default()
        });
        self
    }

    pub fn add_buff(mut self, target_id: i64, buff_id: i32, buff_uid: i64, from_uid: i64) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(target_id),
            effect_type: Some(EffectType::BuffAdd.to_i32()),
            effect_num: Some(buff_id),
            buff: Some(BuffInfo {
                buff_id: Some(buff_id),
                duration: Some(0),
                uid: Some(buff_uid),
                ex_info: Some(0),
                from_uid: Some(from_uid),
                count: Some(0),
                layer: Some(1),
                r#type: Some(0),
                act_common_params: Some(String::new()),
                act_info: vec![],
            }),
            ..Default::default()
        });
        self
    }

    pub fn add_indicator_change(mut self, target_id: i64) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(target_id),
            effect_type: Some(EffectType::IndicatorChange.to_i32()),
            effect_num: Some(0),
            ..Default::default()
        });
        self
    }

    pub fn add_card_distribution(
        mut self,
        cards: Vec<sonettobuf::CardInfo>,
        effect_type: EffectType,
    ) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(0),
            effect_type: Some(effect_type.to_i32()),
            card_info_list: cards,
            ..Default::default()
        });
        self
    }

    pub fn add_power_generation(mut self, power: i32, team_type: i32) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(0),
            effect_type: Some(EffectType::CardDeckNum.to_i32()),
            effect_num: Some(power),
            team_type: Some(team_type),
            ..Default::default()
        });
        self
    }

    pub fn add_typed_effect(mut self, effect_type: EffectType) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(0),
            effect_type: Some(effect_type.to_i32()),
            effect_num: Some(0),
            ..Default::default()
        });
        self
    }

    pub fn add_effect_with_value(
        mut self,
        effect_type: EffectType,
        target_id: i64,
        value: i32,
    ) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(target_id),
            effect_type: Some(effect_type.to_i32()),
            effect_num: Some(value),
            ..Default::default()
        });
        self
    }

    pub fn add_damage(mut self, target_id: i64, damage: i32, is_crit: bool) -> Self {
        let effect_type = if is_crit {
            EffectType::Crit
        } else {
            EffectType::Damage
        };

        self.effects.push(ActEffect {
            target_id: Some(target_id),
            effect_type: Some(effect_type.to_i32()),
            effect_num: Some(damage),
            ..Default::default()
        });
        self
    }

    pub fn add_heal(mut self, target_id: i64, heal: i32, is_crit: bool) -> Self {
        let effect_type = if is_crit {
            EffectType::HealCrit
        } else {
            EffectType::Heal
        };

        self.effects.push(ActEffect {
            target_id: Some(target_id),
            effect_type: Some(effect_type.to_i32()),
            effect_num: Some(heal),
            ..Default::default()
        });
        self
    }

    pub fn add_shield(mut self, target_id: i64, shield_value: i32) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(target_id),
            effect_type: Some(EffectType::Shield.to_i32()),
            effect_num: Some(shield_value),
            ..Default::default()
        });
        self
    }

    pub fn add_control_effect(mut self, target_id: i64, effect_type: EffectType) -> Self {
        debug_assert!(
            effect_type.is_control_effect(),
            "Effect type must be a control effect"
        );

        self.effects.push(ActEffect {
            target_id: Some(target_id),
            effect_type: Some(effect_type.to_i32()),
            effect_num: Some(0),
            ..Default::default()
        });
        self
    }

    pub fn add_effect_type(mut self, effect_type: i32) -> Self {
        self.effects.push(ActEffect {
            target_id: Some(0),
            effect_type: Some(effect_type),
            effect_num: Some(0),
            ..Default::default()
        });
        self
    }

    pub fn add_effect(mut self, effect: ActEffect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn build(self) -> FightStep {
        FightStep {
            act_type: Some(self.act_type.into()),
            from_id: Some(self.from_id),
            to_id: Some(self.to_id),
            act_id: Some(self.act_id),
            act_effect: self.effects,
            card_index: Some(0),
            support_hero_id: Some(0),
            fake_timeline: Some(false),
        }
    }
}
