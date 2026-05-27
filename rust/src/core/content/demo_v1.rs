// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use std::collections::HashMap;

use crate::core::ids::EntityId;
use crate::core::world::{
    Gang, GangStrategy, Item, Person, Territory, WorldState,
};

pub fn build_demo_v1(seed: u64) -> WorldState {
    let mut world = WorldState::new(seed);
    init_territories(&mut world);
    init_gangs(&mut world);
    init_persons(&mut world);
    init_items(&mut world);
    world
}

fn init_territories(world: &mut WorldState) {
    let defs = [
        ("west_market", "城西商贸区", 50, 25, "gang:black_tiger", true, vec!["trade"]),
        ("east_dock", "东码头", 55, 30, "gang:axe_gang", true, vec!["dock"]),
        ("gambling_house", "赌坊", 35, 18, "gang:black_tiger", true, vec!["gamble"]),
        ("brothel", "青楼", 30, 15, "gang:green_dragon", true, vec!["romance"]),
        ("black_market", "黑市", 45, 20, "gang:green_dragon", true, vec!["contraband"]),
        ("yamen", "官府衙门", 0, 40, "", false, vec!["official"]),
    ];
    for (key, name, revenue, defense, controller, controllable, tags) in defs {
        let id = EntityId::territory(key);
        let controller_gang_id = if controller.is_empty() {
            None
        } else {
            Some(EntityId::new(controller))
        };
        world.territories.insert(
            id.as_str().to_string(),
            Territory {
                id: id.clone(),
                name: name.to_string(),
                revenue,
                defense,
                controller_gang_id,
                controllable,
                risk_tags: tags.into_iter().map(String::from).collect(),
            },
        );
    }
}

fn init_gangs(world: &mut WorldState) {
    let defs = [
        (
            "black_tiger",
            "黑虎帮",
            "person:leader_black",
            180i64,
            0.35f32,
            GangStrategy::Balanced,
        ),
        (
            "axe_gang",
            "斧头帮",
            "person:leader_axe",
            220,
            0.45,
            GangStrategy::Expand,
        ),
        (
            "green_dragon",
            "青龙会",
            "person:leader_dragon",
            260,
            0.40,
            GangStrategy::Balanced,
        ),
    ];
    for (key, name, leader, money, influence, strategy) in defs {
        let id = EntityId::gang(key);
        world.gangs.insert(
            id.as_str().to_string(),
            Gang {
                id: id.clone(),
                name: name.to_string(),
                leader_id: EntityId::new(leader),
                money,
                influence,
                morale: 0.6,
                member_ids: Vec::new(),
                strategy,
                focus_territory: None,
                deficit_ticks: 0,
                defeated: false,
            },
        );
    }
}

fn init_persons(world: &mut WorldState) {
    let roster = [
        ("black_tiger", "leader_black", "铁帮主", "帮主", 90, 18, 12),
        ("black_tiger", "strategist_black", "白军师", "军师", 70, 10, 10),
        ("black_tiger", "fighter_black_1", "阿狗", "打手", 85, 16, 8),
        ("black_tiger", "fighter_black_2", "张彪", "打手", 80, 15, 9),
        ("black_tiger", "spy_black", "小六", "探子", 65, 9, 7),
        ("black_tiger", "accountant_black", "铁算盘", "账房", 60, 6, 6),
        ("black_tiger", "accountant_black_2", "二麻子", "账房", 58, 5, 5),
        ("black_tiger", "cook_black", "厨子老周", "杂役", 55, 4, 4),
        ("black_tiger", "maid_black", "小翠", "杂役", 50, 3, 4),
        ("axe_gang", "leader_axe", "斧王", "帮主", 95, 20, 11),
        ("axe_gang", "strategist_axe", "瞎眼军师", "军师", 72, 8, 9),
        ("axe_gang", "fighter_axe_1", "铁牛", "打手", 88, 17, 8),
        ("axe_gang", "fighter_axe_2", "屠夫", "打手", 86, 18, 7),
        ("axe_gang", "spy_axe", "夜猫", "探子", 68, 8, 6),
        ("axe_gang", "accountant_axe", "算盘李", "账房", 62, 5, 5),
        ("axe_gang", "runner_axe_1", "快腿", "杂役", 58, 6, 5),
        ("axe_gang", "runner_axe_2", "闷棍", "杂役", 57, 7, 5),
        ("axe_gang", "runner_axe_3", "疤脸", "杂役", 56, 6, 4),
        ("green_dragon", "leader_dragon", "青龙头", "帮主", 82, 14, 11),
        ("green_dragon", "strategist_dragon", "毒师", "军师", 74, 9, 8),
        ("green_dragon", "fighter_dragon_1", "蛇牙", "打手", 78, 13, 8),
        ("green_dragon", "fighter_dragon_2", "影刃", "打手", 76, 12, 7),
        ("green_dragon", "spy_dragon", "眼线", "探子", 70, 7, 7),
        ("green_dragon", "accountant_dragon", "账房吴", "账房", 60, 5, 5),
        ("green_dragon", "maid_dragon", "红袖", "杂役", 52, 4, 4),
        ("green_dragon", "runner_dragon_1", "瘦猴", "杂役", 54, 5, 4),
        ("green_dragon", "runner_dragon_2", "胖虎", "杂役", 55, 5, 4),
    ];
    for (gang_key, person_key, name, role, hp, atk, def) in roster {
        let id = EntityId::person(person_key);
        let gang_id = EntityId::gang(gang_key);
        let mut desires = HashMap::new();
        desires.insert("wealth".into(), 0.5);
        desires.insert("power".into(), 0.4);
        desires.insert("revenge".into(), 0.2);
        let mut personality = HashMap::new();
        personality.insert("impulsive".into(), if role == "打手" { 0.7 } else { 0.3 });
        personality.insert("loyal".into(), if role == "帮主" { 0.8 } else { 0.5 });
        let mut status = HashMap::new();
        if person_key == "accountant_black_2" {
            status.insert("indebted".into(), true);
            desires.insert("wealth".into(), 0.85);
        }
        if person_key == "strategist_axe" {
            status.insert("humiliated".into(), true);
        }
        let person = Person {
            id: id.clone(),
            name: name.to_string(),
            gang_id: gang_id.clone(),
            role: role.to_string(),
            alive: true,
            hp,
            attack: atk,
            defense: def,
            desires,
            personality,
            status,
            relationships: HashMap::new(),
            memories: Vec::new(),
            loyalty: 0.55,
        };
        world.persons.insert(id.as_str().to_string(), person);
        if let Ok(gang) = world.gang_mut(&gang_id) {
            gang.member_ids.push(id);
        }
    }
    seed_rivalries(world);
}

fn seed_rivalries(world: &mut WorldState) {
    let pairs = [
        ("person:fighter_black_1", "person:fighter_axe_1", -0.4),
        ("person:accountant_black_2", "person:accountant_black", -0.2),
        ("person:strategist_axe", "person:leader_axe", -0.3),
    ];
    for (a, b, v) in pairs {
        if let Some(p) = world.persons.get_mut(a) {
            p.relationships.insert(b.to_string(), v);
        }
        if let Some(p) = world.persons.get_mut(b) {
            p.relationships.insert(a.to_string(), -v * 0.5);
        }
    }
}

fn init_items(world: &mut WorldState) {
    let defs = [
        (
            "ledger",
            "黑虎帮账本",
            "document",
            "person:accountant_black",
            "记录着保护费与欠条",
            80,
        ),
        (
            "poison_vial",
            "毒瓶",
            "contraband",
            "person:strategist_dragon",
            "黑市常见货色",
            120,
        ),
        (
            "lime_powder",
            "石灰粉",
            "weapon",
            "person:fighter_black_2",
            "打架常用, 容易误伤",
            15,
        ),
    ];
    for (key, name, kind, owner, desc, value) in defs {
        let id = EntityId::item(key);
        world.items.insert(
            id.as_str().to_string(),
            Item {
                id: id.clone(),
                name: name.to_string(),
                kind: kind.to_string(),
                owner_id: Some(EntityId::new(owner)),
                description: desc.to_string(),
                value,
            },
        );
    }
}

pub fn player_gang_id() -> EntityId {
    EntityId::new("gang:black_tiger")
}
