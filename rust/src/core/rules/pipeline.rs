// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use rand::Rng;

use crate::core::events::{EventSeverity, EventType, GameEvent};
use crate::core::ids::EntityId;
use crate::core::world::{GangStrategy, WorldState};

pub fn run_tick_systems(world: &mut WorldState, rng: &mut impl Rng) {
    run_economy(world);
    run_person_actions(world, rng);
    run_gang_conflicts(world, rng);
    decay_memories(world);
    check_gang_defeat(world);
}

fn run_economy(world: &mut WorldState) {
    let territories = world.territory_list();
    let mut gang_ids: Vec<EntityId> = world.gangs.values().map(|g| g.id.clone()).collect();
        gang_ids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    for gid in gang_ids {
        let (name, income, expense, mut money, mut morale, mut deficit) = {
            let gang = world.gang(&gid).expect("gang");
            (
                gang.name.clone(),
                gang.daily_income(&territories),
                gang.daily_expense(),
                gang.money,
                gang.morale,
                gang.deficit_ticks,
            )
        };
        if world.gang(&gid).map(|g| g.defeated).unwrap_or(true) {
            continue;
        }
        let delta = income - expense;
        money += delta;
        if delta < 0 {
            deficit += 1;
            morale = (morale - 0.05).max(0.1);
        } else {
            deficit = 0;
            morale = (morale + 0.02).min(1.0);
        }
        {
            let gang = world.gang_mut(&gid).expect("gang");
            gang.money = money;
            gang.morale = morale;
            gang.deficit_ticks = deficit;
        }
        let mut event = GameEvent::new(
            EventType::EconomySettled,
            EventSeverity::Minor,
            vec![gid.as_str().to_string()],
            vec![],
        );
        event.payload.insert("gang_name".into(), name.clone());
        event.payload.insert("delta".into(), delta.to_string());
        world.push_event(event);
        if deficit >= 2 && money < 80 {
            let mut crisis = GameEvent::new(
                EventType::FinancialCrisis,
                EventSeverity::Major,
                vec![gid.as_str().to_string()],
                vec![],
            );
            crisis.payload.insert("gang_name".into(), name);
            world.push_event(crisis);
            try_layoff(world, &gid);
        }
    }
}

fn try_layoff(world: &mut WorldState, gang_id: &EntityId) {
    let member = world
        .gang(gang_id)
        .ok()
        .and_then(|g| {
            g.member_ids
                .iter()
                .find(|m| {
                    world
                        .person(m)
                        .map(|p| p.role == "打手")
                        .unwrap_or(false)
                })
                .cloned()
        });
    let Some(person_id) = member else { return };
    let person_name = world.person(&person_id).ok().map(|p| p.name.clone()).unwrap_or_default();
    {
        let gang = world.gang_mut(gang_id).expect("gang");
        gang.member_ids.retain(|m| m != &person_id);
        gang.morale = (gang.morale - 0.08).max(0.05);
    }
    let other_gangs: Vec<EntityId> = world
        .gangs
        .values()
        .filter(|g| &g.id != gang_id && !g.defeated)
        .map(|g| g.id.clone())
        .collect();
    if let Some(target_gang) = other_gangs.first() {
        if let Ok(person) = world.person_mut(&person_id) {
            person.gang_id = target_gang.clone();
            person.loyalty = 0.35;
        }
        if let Ok(gang) = world.gang_mut(target_gang) {
            gang.member_ids.push(person_id.clone());
        }
        let to_name = world.gang(target_gang).map(|g| g.name.clone()).unwrap_or_default();
        let mut event = GameEvent::new(
            EventType::PersonDefected,
            EventSeverity::Critical,
            vec![person_id.as_str().to_string()],
            vec![target_gang.as_str().to_string()],
        );
        event.payload.insert("person_name".into(), person_name);
        event.payload.insert("to_gang".into(), to_name);
        world.push_event(event);
    } else {
        let mut event = GameEvent::new(
            EventType::PersonLeftGang,
            EventSeverity::Major,
            vec![person_id.as_str().to_string()],
            vec![gang_id.as_str().to_string()],
        );
        event.payload.insert("person_name".into(), person_name);
        world.push_event(event);
    }
}

fn run_person_actions(world: &mut WorldState, rng: &mut impl Rng) {
    let person_ids = world.alive_person_ids();
    for pid in person_ids {
        let (name, gang_id, role, wealth_desire, impulsive, indebted, loyalty, hate_leader) = {
            let p = match world.person(&pid) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let hate = p
                .relationships
                .get("gang:black_tiger")
                .copied()
                .unwrap_or(0.0)
                .min(0.0)
                .abs();
            (
                p.name.clone(),
                p.gang_id.clone(),
                p.role.clone(),
                p.desires.get("wealth").copied().unwrap_or(0.3),
                p.personality.get("impulsive").copied().unwrap_or(0.3),
                p.status.get("indebted").copied().unwrap_or(false),
                p.loyalty,
                hate,
            )
        };
        if rng.gen::<f32>() < 0.08 && indebted && wealth_desire > 0.6 && role == "账房" {
            let amount = rng.gen_range(30..90);
            if let Ok(gang) = world.gang_mut(&gang_id) {
                gang.money = (gang.money - amount).max(0);
            }
            let mut event = GameEvent::new(
                EventType::MoneyStolen,
                EventSeverity::Major,
                vec![pid.as_str().to_string()],
                vec![gang_id.as_str().to_string()],
            );
            event.payload.insert("person_name".into(), name.clone());
            event.payload.insert("amount".into(), amount.to_string());
            world.push_event(event);
            continue;
        }
        if loyalty < 0.35 && rng.gen::<f32>() < 0.06 {
            try_defect(world, &pid, &gang_id, &name, rng);
            continue;
        }
        if impulsive > 0.6 && hate_leader > 0.2 && rng.gen::<f32>() < 0.05 {
            let target = world
                .persons
                .values()
                .find(|p| p.alive && p.gang_id != gang_id)
                .map(|p| p.id.clone());
            if let Some(defender) = target {
                let defender_name = world.person(&defender).map(|p| p.name.clone()).unwrap_or_default();
                let mut event = GameEvent::new(
                    EventType::DuelStarted,
                    EventSeverity::Major,
                    vec![pid.as_str().to_string()],
                    vec![defender.as_str().to_string()],
                );
                event.payload.insert("attacker".into(), name.clone());
                event.payload.insert("defender".into(), defender_name);
                world.push_event(event);
                resolve_duel(world, &pid, &defender, rng);
            }
        }
        if role == "探子" && rng.gen::<f32>() < 0.03 {
            let mut event = GameEvent::new(
                EventType::OfficialReported,
                EventSeverity::Major,
                vec![pid.as_str().to_string()],
                vec!["territory:yamen".into()],
            );
            event.payload.insert("person_name".into(), name);
            world.push_event(event);
            if let Ok(gang) = world.gang_mut(&gang_id) {
                gang.influence = (gang.influence - 0.1).max(0.05);
            }
        }
    }
}

fn try_defect(
    world: &mut WorldState,
    person_id: &EntityId,
    from_gang: &EntityId,
    person_name: &str,
    rng: &mut impl Rng,
) {
    let targets: Vec<EntityId> = world
        .gangs
        .values()
        .filter(|g| &g.id != from_gang && !g.defeated)
        .map(|g| g.id.clone())
        .collect();
    if targets.is_empty() {
        return;
    }
    let idx = rng.gen_range(0..targets.len());
    let to = targets[idx].clone();
    {
        let from = world.gang_mut(from_gang).expect("gang");
        from.member_ids.retain(|m| m != person_id);
    }
    {
        let person = world.person_mut(person_id).expect("person");
        person.gang_id = to.clone();
        person.loyalty = 0.45;
    }
    {
        let gang = world.gang_mut(&to).expect("gang");
        gang.member_ids.push(person_id.clone());
    }
    let to_name = world.gang(&to).map(|g| g.name.clone()).unwrap_or_default();
    let mut event = GameEvent::new(
        EventType::PersonDefected,
        EventSeverity::Critical,
        vec![person_id.as_str().to_string()],
        vec![to.as_str().to_string()],
    );
    event.payload.insert("person_name".into(), person_name.to_string());
    event.payload.insert("to_gang".into(), to_name);
    world.push_event(event);
}

fn resolve_duel(world: &mut WorldState, attacker: &EntityId, defender: &EntityId, rng: &mut impl Rng) {
    let (atk, def, atk_name, def_name) = {
        let a = world.person(attacker).expect("a");
        let d = world.person(defender).expect("d");
        (a.attack, d.defense, a.name.clone(), d.name.clone())
    };
    let damage = (atk - def / 2).max(5) + rng.gen_range(0..8);
    let mut defender_dead = false;
    if let Ok(d) = world.person_mut(defender) {
        d.hp -= damage;
        if d.hp <= 0 {
            d.alive = false;
            d.hp = 0;
            defender_dead = true;
        }
    }
    if defender_dead {
        let mut event = GameEvent::new(
            EventType::PersonKilled,
            EventSeverity::Critical,
            vec![defender.as_str().to_string()],
            vec![attacker.as_str().to_string()],
        );
        event.payload.insert("person_name".into(), def_name);
        world.push_event(event);
    } else {
        let mut event = GameEvent::new(
            EventType::PersonInjured,
            EventSeverity::Normal,
            vec![defender.as_str().to_string()],
            vec![],
        );
        event.payload.insert("person_name".into(), def_name);
        world.push_event(event);
    }
    if rng.gen::<f32>() < 0.15 {
        let detail = format!(
            "{} 挥招时误伤同伴, 场面一度像排练失败的戏班",
            atk_name
        );
        let mut event = GameEvent::new(
            EventType::AbsurdAccident,
            EventSeverity::Major,
            vec![attacker.as_str().to_string()],
            vec![],
        );
        event.payload.insert("detail".into(), detail);
        world.push_event(event);
    }
}

fn run_gang_conflicts(world: &mut WorldState, rng: &mut impl Rng) {
    let mut gangs: Vec<(EntityId, GangStrategy, Option<EntityId>)> = world
        .gangs
        .values()
        .filter(|g| !g.defeated)
        .map(|g| (g.id.clone(), g.strategy, g.focus_territory.clone()))
        .collect();
    gangs.sort_by(|a, b| a.0.as_str().cmp(b.0.as_str()));
    for (gang_id, strategy, focus) in gangs {
        if !matches!(strategy, GangStrategy::Expand) {
            continue;
        }
        let target_territory = if let Some(t) = focus {
            t
        } else {
            continue;
        };
        let (t_name, defender) = {
            let t = match world.territory(&target_territory) {
                Ok(t) => t,
                Err(_) => continue,
            };
            if !t.controllable {
                continue;
            }
            (t.name.clone(), t.controller_gang_id.clone())
        };
        let Some(defender_id) = defender else { continue };
        if defender_id == gang_id {
            continue;
        }
        let attacker_name = world.gang(&gang_id).map(|g| g.name.clone()).unwrap_or_default();
        let defender_name = world.gang(&defender_id).map(|g| g.name.clone()).unwrap_or_default();
        let mut start = GameEvent::new(
            EventType::BattleStarted,
            EventSeverity::Major,
            vec![gang_id.as_str().to_string()],
            vec![target_territory.as_str().to_string()],
        );
        start.payload.insert("attacker_gang".into(), attacker_name.clone());
        start.payload.insert("territory".into(), t_name.clone());
        world.push_event(start);
        let atk_power: i32 = world
            .gang(&gang_id)
            .map(|g| g.member_ids.len() as i32 * 10)
            .unwrap_or(0)
            + rng.gen_range(0..15);
        let def_power: i32 = world
            .gang(&defender_id)
            .map(|g| g.member_ids.len() as i32 * 10)
            .unwrap_or(0)
            + rng.gen_range(0..15);
        let attacker_wins = atk_power >= def_power;
        let winner_id = if attacker_wins {
            gang_id.clone()
        } else {
            defender_id.clone()
        };
        let winner_name = world.gang(&winner_id).map(|g| g.name.clone()).unwrap_or_default();
        let mut resolved = GameEvent::new(
            EventType::BattleResolved,
            EventSeverity::Critical,
            vec![winner_id.as_str().to_string()],
            vec![target_territory.as_str().to_string()],
        );
        resolved.payload.insert("winner".into(), winner_name.clone());
        resolved.payload.insert("territory".into(), t_name.clone());
        world.push_event(resolved);
        if attacker_wins {
            if let Ok(t) = world.territory_mut(&target_territory) {
                t.controller_gang_id = Some(gang_id.clone());
            }
            let mut conquered = GameEvent::new(
                EventType::TerritoryConquered,
                EventSeverity::Critical,
                vec![gang_id.as_str().to_string()],
                vec![target_territory.as_str().to_string()],
            );
            conquered.payload.insert("gang_name".into(), winner_name);
            conquered.payload.insert("territory".into(), t_name.clone());
            world.push_event(conquered);
            if rng.gen::<f32>() < 0.25 {
                let detail = format!(
                    "{} 强攻 {} 时, {} 的军师先被石灰粉糊眼, 仍宣称大捷",
                    attacker_name, t_name, attacker_name
                );
                let mut absurd = GameEvent::new(
                    EventType::AbsurdAccident,
                    EventSeverity::Major,
                    vec![gang_id.as_str().to_string()],
                    vec![],
                );
                absurd.payload.insert("detail".into(), detail);
                world.push_event(absurd);
            }
        }
        let _ = defender_name;
    }
}

fn decay_memories(world: &mut WorldState) {
    for person in world.persons.values_mut() {
        for rel in person.relationships.values_mut() {
            if rel.abs() > 0.01 {
                *rel *= 0.99;
            }
        }
    }
}

fn check_gang_defeat(world: &mut WorldState) {
    let territories = world.territory_list();
    let mut gang_ids: Vec<EntityId> = world.gangs.values().map(|g| g.id.clone()).collect();
        gang_ids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    for gid in gang_ids {
        let alive_members = world
            .gang(&gid)
            .map(|g| g.member_ids.iter().filter(|m| world.person(m).map(|p| p.alive).unwrap_or(false)).count())
            .unwrap_or(0);
        let income = world.gang(&gid).map(|g| g.daily_income(&territories)).unwrap_or(0);
        if alive_members == 0 || (income == 0 && alive_members < 2) {
            if let Ok(gang) = world.gang_mut(&gid) {
                if !gang.defeated {
                    gang.defeated = true;
                    let name = gang.name.clone();
                    let mut event = GameEvent::new(
                        EventType::GangDefeated,
                        EventSeverity::Critical,
                        vec![gid.as_str().to_string()],
                        vec![],
                    );
                    event.payload.insert("gang_name".into(), name);
                    world.push_event(event);
                }
            }
        }
    }
}
