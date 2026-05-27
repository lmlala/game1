// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use godot::builtin::{Array, Dictionary, GString, PackedStringArray, Variant};
use godot::classes::Node;
use godot::prelude::*;

use crate::core::content::player_gang_id;
use crate::core::ids::EntityId;
use crate::core::sim::{PlayerCommand, SimRunner};
use crate::core::world::{Item, Person, Territory};

type VarDict = Dictionary<Variant, Variant>;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct DemoController {
    runner: SimRunner,
    base: Base<Node>,
}

#[godot_api]
impl INode for DemoController {
    fn init(base: Base<Node>) -> Self {
        godot_print!("game1_core: DemoController initialized");
        Self {
            runner: SimRunner::new(42),
            base,
        }
    }
}

#[godot_api]
impl DemoController {
    #[signal]
    fn tick_advanced(tick: i32);

    #[func]
    fn reset_demo(&mut self, seed: i64) {
        let seed = if seed <= 0 { 42 } else { seed as u64 };
        self.runner.reset(seed);
        let tick = self.runner.world.tick as i32;
        self.base_mut()
            .emit_signal("tick_advanced", &[Variant::from(tick)]);
    }

    #[func]
    fn advance_tick(&mut self) -> bool {
        match self.runner.advance_tick() {
            Ok(()) => {
                let tick = self.runner.world.tick as i32;
                self.base_mut()
                    .emit_signal("tick_advanced", &[Variant::from(tick)]);
                true
            }
            Err(err) => {
                godot_error!("advance_tick failed: {err}");
                false
            }
        }
    }

    #[func]
    fn set_paused(&mut self, paused: bool) {
        self.runner.set_paused(paused);
    }

    #[func]
    fn is_paused(&self) -> bool {
        self.runner.is_paused()
    }

    #[func]
    fn get_tick(&self) -> i32 {
        self.runner.world.tick as i32
    }

    #[func]
    fn get_time_label(&self) -> GString {
        let state = if self.runner.is_paused() {
            "暂停"
        } else {
            "运行"
        };
        let text = format!("第 {} 天 | {}", self.runner.world.tick, state);
        GString::from(text.as_str())
    }

    #[func]
    fn get_metrics_summary(&self) -> GString {
        let text = self.runner.metrics().summary_line();
        GString::from(text.as_str())
    }

    #[func]
    fn get_recent_logs(&self) -> PackedStringArray {
        let mut arr = PackedStringArray::new();
        for line in self.runner.world.events.recent_player_logs(40) {
            arr.push(line.as_str());
        }
        if arr.is_empty() {
            arr.push("【等待】江湖尚未开始躁动, 点击下一天或继续。");
        }
        arr
    }

    #[func]
    fn get_entities(&self, kind: GString) -> Array<Variant> {
        let mut arr = Array::<Variant>::new();
        let kind = kind.to_string();
        match kind.as_str() {
            "person" => {
                for person in self.runner.world.persons.values() {
                    arr.push(&Variant::from(person_summary(person)));
                }
            }
            "territory" => {
                for territory in self.runner.world.territories.values() {
                    arr.push(&Variant::from(territory_summary(
                        territory,
                        &self.runner,
                    )));
                }
            }
            "item" => {
                for item in self.runner.world.items.values() {
                    arr.push(&Variant::from(item_summary(item)));
                }
            }
            _ => {}
        }
        arr
    }

    #[func]
    fn get_entity_panel(&self, entity_id: GString) -> VarDict {
        let id = entity_id.to_string();
        let mut panel = VarDict::new();
        panel.set("id", id.as_str());
        if let Some(person) = self.runner.world.persons.get(&id) {
            panel.set("kind", "person");
            panel.set("title", format!("{} ({})", person.name, person.role).as_str());
            let mut attrs = VarDict::new();
            attrs.set("帮派", gang_name(&self.runner, &person.gang_id).as_str());
            attrs.set("生命", person.hp);
            attrs.set("攻击", person.attack);
            attrs.set("防御", person.defense);
            attrs.set("忠诚", format!("{:.2}", person.loyalty).as_str());
            attrs.set("状态", if person.alive { "存活" } else { "身亡" });
            panel.set("attributes", &attrs);
        } else if let Some(territory) = self.runner.world.territories.get(&id) {
            panel.set("kind", "territory");
            panel.set("title", territory.name.as_str());
            let mut attrs = VarDict::new();
            attrs.set("收益", territory.revenue);
            attrs.set("守备", territory.defense);
            let controller = territory
                .controller_gang_id
                .as_ref()
                .map(|g| gang_name(&self.runner, g))
                .unwrap_or_else(|| "无".into());
            attrs.set("控制帮派", controller.as_str());
            attrs.set("可占领", territory.controllable);
            panel.set("attributes", &attrs);
        } else if let Some(item) = self.runner.world.items.get(&id) {
            panel.set("kind", "item");
            panel.set("title", item.name.as_str());
            let mut attrs = VarDict::new();
            attrs.set("类型", item.kind.as_str());
            attrs.set("价值", item.value);
            attrs.set("描述", item.description.as_str());
            let owner = item
                .owner_id
                .as_ref()
                .and_then(|o| self.runner.world.persons.get(o.as_str()))
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "无主".into());
            attrs.set("持有者", owner.as_str());
            panel.set("attributes", &attrs);
        } else {
            panel.set("kind", "unknown");
            panel.set("title", "未知目标");
            panel.set("attributes", &VarDict::new());
        }
        let mut logs = PackedStringArray::new();
        for line in self.runner.world.events.logs_for_entity(&id, 30) {
            logs.push(line.as_str());
        }
        if logs.is_empty() {
            logs.push("暂无相关日志。");
        }
        panel.set("logs", &logs);
        panel
    }

    #[func]
    fn queue_expand(&mut self, territory_key: GString) -> bool {
        self.queue(PlayerCommand::Expand {
            gang_id: player_gang_id(),
            territory_id: parse_territory_key(&territory_key.to_string()),
        })
    }

    #[func]
    fn queue_defend(&mut self, territory_key: GString) -> bool {
        self.queue(PlayerCommand::Defend {
            gang_id: player_gang_id(),
            territory_id: parse_territory_key(&territory_key.to_string()),
        })
    }

    #[func]
    fn queue_reward(&mut self, person_key: GString, money: i64) -> bool {
        self.queue(PlayerCommand::RewardPerson {
            person_id: parse_person_key(&person_key.to_string()),
            money,
        })
    }

    #[func]
    fn queue_punish(&mut self, person_key: GString) -> bool {
        self.queue(PlayerCommand::PunishPerson {
            person_id: parse_person_key(&person_key.to_string()),
        })
    }

    #[func]
    fn queue_recruit(&mut self, budget: i64) -> bool {
        self.queue(PlayerCommand::Recruit {
            gang_id: player_gang_id(),
            budget,
        })
    }

    #[func]
    fn get_default_selection(&self) -> VarDict {
        let mut d = VarDict::new();
        d.set("person", "person:accountant_black");
        d.set("territory", "territory:east_dock");
        d.set("item", "item:ledger");
        d
    }
}

impl DemoController {
    fn queue(&mut self, command: PlayerCommand) -> bool {
        match self.runner.queue_command(command) {
            Ok(()) => true,
            Err(err) => {
                godot_error!("queue command failed: {err}");
                false
            }
        }
    }
}


fn parse_person_key(key: &str) -> EntityId {
    if key.starts_with("person:") {
        EntityId::new(key)
    } else {
        EntityId::person(key)
    }
}

fn parse_territory_key(key: &str) -> EntityId {
    if key.starts_with("territory:") {
        EntityId::new(key)
    } else {
        EntityId::territory(key)
    }
}

fn gang_name(runner: &SimRunner, gang_id: &EntityId) -> String {
    runner
        .world
        .gang(gang_id)
        .map(|g| g.name.clone())
        .unwrap_or_else(|_| gang_id.as_str().to_string())
}

fn person_summary(person: &Person) -> VarDict {
    let mut d = VarDict::new();
    d.set("id", person.id.as_str());
    d.set(
        "label",
        format!("{} [{}]", person.name, person.role).as_str(),
    );
    d.set("gang_id", person.gang_id.as_str());
    d.set("x", person_pos_x(person.id.as_str()));
    d.set("y", person_pos_y(person.id.as_str()));
    d
}

fn territory_summary(territory: &Territory, runner: &SimRunner) -> VarDict {
    let mut d = VarDict::new();
    d.set("id", territory.id.as_str());
    let controller = territory
        .controller_gang_id
        .as_ref()
        .map(|g| gang_name(runner, g))
        .unwrap_or_else(|| "中立".into());
    d.set(
        "label",
        format!("{} ({controller})", territory.name).as_str(),
    );
    d.set("x", territory_pos_x(territory.id.as_str()));
    d.set("y", territory_pos_y(territory.id.as_str()));
    d
}

fn item_summary(item: &Item) -> VarDict {
    let mut d = VarDict::new();
    d.set("id", item.id.as_str());
    d.set("label", item.name.as_str());
    if let Some(owner) = &item.owner_id {
        d.set("x", person_pos_x(owner.as_str()) + 20.0);
        d.set("y", person_pos_y(owner.as_str()) - 20.0);
    } else {
        d.set("x", 640.0);
        d.set("y", 360.0);
    }
    d
}

fn person_pos_x(id: &str) -> f32 {
    let h = hash_id(id);
    180.0 + (h % 900) as f32
}

fn person_pos_y(id: &str) -> f32 {
    let h = hash_id(id);
    120.0 + ((h / 900) % 420) as f32
}

fn territory_pos_x(id: &str) -> f32 {
    match id {
        "territory:west_market" => 200.0,
        "territory:east_dock" => 520.0,
        "territory:gambling_house" => 760.0,
        "territory:brothel" => 320.0,
        "territory:black_market" => 900.0,
        "territory:yamen" => 640.0,
        _ => 400.0,
    }
}

fn territory_pos_y(id: &str) -> f32 {
    match id {
        "territory:west_market" => 180.0,
        "territory:east_dock" => 220.0,
        "territory:gambling_house" => 260.0,
        "territory:brothel" => 420.0,
        "territory:black_market" => 380.0,
        "territory:yamen" => 520.0,
        _ => 300.0,
    }
}

fn hash_id(id: &str) -> u32 {
    id.bytes().map(u32::from).sum()
}
