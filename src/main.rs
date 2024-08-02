use std::collections::VecDeque;

use cards::{Card, StabCard};
use seed::Seed; // 引入种子生成器，用于游戏随机性

pub mod cards;
pub mod game; // 游戏逻辑模块
pub mod seed; // 种子生成器模块
pub mod tag; // 标签结构体模块

/// 英雄枚举，表示不同的可选英雄角色
pub enum Hero {
    InfinityWarrior, // 无尽剑士
    Aquamancer,      // 水魔导士
}

/// AI 执行器结构体，用于处理敌人的 AI 逻辑
pub struct AIExecutor {}

/// Buff 枚举，表示不同的增益效果
pub enum Buff {
    Regeneration((u32, u32)), // 生命回复，括号内为回复量,持续回合
    Anger(u32),               // 愤怒，括号内为愤怒值或持续回合数
    Strength(u32),            // 力量，括号内为力量加成值
    Counterwound(u32),        // 反伤，括号内为反伤数值
    Sharpness(u32),           // 锋利，括号内为锋利度或攻击加成值
}

/// Debuff 枚举，表示不同的减益效果
pub enum Debuff {
    Poison(u32), // 中毒，括号内为中毒层数或持续回合数
}

/// 特殊效果枚举，表示一些特殊效果，例如技能或道具的效果
pub enum SpecialEffect {}

/// 怪物 Trait，定义了怪物的行为
pub trait Monster {
    /// 获取怪物的 AI 执行器
    fn ai(&self) -> AIExecutor;

    fn take_damage(&mut self, damage: u32);
}

/// 敌人上下文结构体，包含敌人的相关信息
pub struct EnemyContext {
    pub entity_id: u64,            // 敌人的唯一 ID
    pub monster: Box<dyn Monster>, // 怪物实例，使用动态分发以支持不同类型的怪物
}

pub struct BaseState {
    pub round_num: u32,      // 当前回合数
    pub layer_num: u32,      // 当前层数
    pub base_step_num: u128, // 当前基础步数
    pub current_seed: Seed,  // 当前游戏种子
}

/// 游戏上下文结构体，包含当前游戏状态和相关信息
pub struct GameContext {
    pub is_game_on: bool, // 游戏是否正在进行

    pub base_state: BaseState, // 游戏状态

    pub save_path: String,                  // 游戏存档路径
    pub gamer: String,                      // 玩家名称
    pub hero: HeroContext,                  // 英雄上下文
    pub current_enemies: Vec<EnemyContext>, // 当前遇到的敌人列表
    pub deck: Vec<DeckContext>,             // 卡组
    pub environment: EnvironmentContext,    // 环境上下文
}

pub struct DeckContext {
    pub draw_pile: VecDeque<Box<dyn Card>>,    // 抽牌堆
    pub discard_pile: VecDeque<Box<dyn Card>>, // 弃牌堆
    pub hand: Vec<Box<dyn Card>>,              // 手牌
}

/// 环境上下文结构体，包含游戏环境的相关信息
pub struct EnvironmentContext {}

/// 游戏状态结构体，包含游戏是否正在进行、存档路径、当前种子和游戏上下文等信息

/// 英雄上下文结构体，包含英雄的相关信息
pub struct HeroContext {
    pub hero: Hero,                         // 英雄类型
    pub entity_id: u64,                     // 英雄的唯一 ID
    pub health: u32,                        // 英雄当前生命值
    pub buff: Vec<Buff>,                    // 英雄身上的增益效果列表
    pub debuff: Vec<Debuff>,                // 英雄身上的减益效果列表
    pub special_effect: Vec<SpecialEffect>, // 英雄身上的特殊效果列表
}

fn main() {
    let card = StabCard::new(1);
    println!("{:?}", card.get_info());
}
