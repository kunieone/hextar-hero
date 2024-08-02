use crate::{tag::Tag, GameContext, Hero};
// cards.rs
#[derive(Debug)]
pub enum TargetType {
    SelfTarget,
    OneMonsterTarget,
    MonstersTarget,
}

pub trait SelfTargetable {
}
pub trait OneMonsterTargetable {}
pub trait MonstersTargetable {}

pub trait Card {
    fn card_id(&self) -> u64;
    fn card_tag(&self) -> Tag;
    fn target_type(&self) -> TargetType;
    fn card_users(&self) -> Vec<Hero>;
    fn get_info(&self) -> String {
        // 完成输出 id, tag, 限制，类型等
        format!(
            "ID: {}, TAG: {}, TYPE: {:?}",
            self.card_id(),
            self.card_tag(),
            self.target_type()
        )
    }
}

pub trait CardCommand {
    fn execute(&self, ctx: &mut GameContext);
}

pub struct StabCommand {
    pub monster_id: u64,
    pub damage: u32,
}

impl CardCommand for StabCommand {
    fn execute(&self, ctx: &mut GameContext) {
        // 根据 monster_id 查找目标敌人
        let target_enemy = ctx
            .current_enemies
            .iter_mut()
            .find(|e| e.entity_id == self.monster_id);

        if let Some(target_enemy) = target_enemy {
            target_enemy.monster.take_damage(self.damage);
        }
    }
}

// 刺击卡牌
#[derive(Debug)]
pub struct StabCard {
    pub id: u64,
    pub tag: Tag,
    pub damage: u32,
}

impl StabCard {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            tag: Tag::zh("刺击").en("Stab"),
            damage: 5,
        }
    }
}

impl Card for StabCard {
    fn card_id(&self) -> u64 {
        self.id
    }
    fn target_type(&self) -> TargetType {
        TargetType::OneMonsterTarget
    }

    fn card_tag(&self) -> Tag {
        self.tag.clone()
    }

    fn card_users(&self) -> Vec<Hero> {
        vec![Hero::InfinityWarrior, Hero::Aquamancer]
    }
}
