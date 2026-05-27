// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::core::content::build_demo_v1;
use crate::core::error::SimResult;
use crate::core::events::{EventSeverity, EventType, GameEvent};
use crate::core::rules::run_tick_systems;
use crate::core::sim::commands::{apply_command, PlayerCommand};
use crate::core::sim::metrics::MetricsSnapshot;
use crate::core::world::WorldState;

pub struct SimRunner {
    pub world: WorldState,
    rng: StdRng,
    pub paused: bool,
    pending_commands: Vec<PlayerCommand>,
}

impl SimRunner {
    pub fn new(seed: u64) -> Self {
        Self {
            world: build_demo_v1(seed),
            rng: StdRng::seed_from_u64(seed),
            paused: false,
            pending_commands: Vec::new(),
        }
    }

    pub fn reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn queue_command(&mut self, command: PlayerCommand) -> SimResult<()> {
        command.validate(&self.world)?;
        let desc = command.describe();
        self.pending_commands.push(command);
        let mut event = GameEvent::new(
            EventType::PlayerCommandQueued,
            EventSeverity::Normal,
            vec!["gang:black_tiger".into()],
            vec![],
        );
        event.payload.insert("command".into(), desc);
        self.world.push_event(event);
        Ok(())
    }

    pub fn advance_tick(&mut self) -> SimResult<()> {
        if self.paused {
            return Ok(());
        }
        self.world.tick += 1;
        let commands: Vec<PlayerCommand> = self.pending_commands.drain(..).collect();
        for cmd in commands {
            apply_command(&mut self.world, &cmd)?;
            if let PlayerCommand::Recruit { gang_id, budget } = &cmd {
                self.try_recruit(gang_id, *budget);
            }
        }
        run_tick_systems(&mut self.world, &mut self.rng);
        Ok(())
    }

    pub fn advance_ticks(&mut self, count: u32) -> SimResult<()> {
        for _ in 0..count {
            self.advance_tick()?;
        }
        Ok(())
    }

    pub fn metrics(&self) -> MetricsSnapshot {
        MetricsSnapshot::from_world(&self.world)
    }

    fn try_recruit(&mut self, gang_id: &crate::core::ids::EntityId, budget: i64) {
        if self.rng.gen::<f32>() > 0.55 {
            return;
        }
        let idx = self.rng.gen_range(1000..9999);
        let person_key = format!("recruit_{idx}");
        let id = crate::core::ids::EntityId::person(&person_key);
        let name = format!("路人{idx}");
        let person = crate::core::world::Person {
            id: id.clone(),
            name: name.clone(),
            gang_id: gang_id.clone(),
            role: "杂役".into(),
            alive: true,
            hp: 55,
            attack: 5,
            defense: 4,
            desires: Default::default(),
            personality: Default::default(),
            status: Default::default(),
            relationships: Default::default(),
            memories: Vec::new(),
            loyalty: 0.4,
        };
        self.world.persons.insert(id.as_str().to_string(), person);
        if let Ok(gang) = self.world.gang_mut(gang_id) {
            gang.member_ids.push(id.clone());
        }
        let gang_name = self
            .world
            .gang(gang_id)
            .map(|g| g.name.clone())
            .unwrap_or_default();
        let mut event = GameEvent::new(
            EventType::PersonRecruited,
            EventSeverity::Normal,
            vec![id.as_str().to_string()],
            vec![gang_id.as_str().to_string()],
        );
        event.payload.insert("person_name".into(), name);
        event.payload.insert("gang_name".into(), gang_name);
        self.world.push_event(event);
        let _ = budget;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::content::player_gang_id;
    use crate::core::ids::EntityId;

    #[test]
    fn demo_world_loads() {
        let sim = SimRunner::new(42);
        assert!(sim.world.gangs.len() >= 3);
        assert!(sim.world.persons.len() >= 24);
        assert!(sim.world.territories.len() >= 6);
    }

    #[test]
    fn deterministic_with_same_seed() {
        let mut a = SimRunner::new(7);
        let mut b = SimRunner::new(7);
        a.advance_ticks(10).unwrap();
        b.advance_ticks(10).unwrap();
        let mut types_a: Vec<_> = a
            .world
            .events
            .distinct_event_types()
            .into_iter()
            .collect();
        let mut types_b: Vec<_> = b
            .world
            .events
            .distinct_event_types()
            .into_iter()
            .collect();
        types_a.sort();
        types_b.sort();
        assert_eq!(types_a, types_b);
    }

    #[test]
    fn reward_command_changes_money() {
        let mut sim = SimRunner::new(99);
        let person = EntityId::person("accountant_black");
        let before = sim.world.gang(&player_gang_id()).unwrap().money;
        sim.queue_command(PlayerCommand::RewardPerson {
            person_id: person,
            money: 50,
        })
        .unwrap();
        sim.advance_tick().unwrap();
        let after = sim.world.gang(&player_gang_id()).unwrap().money;
        assert!(after < before);
    }

    #[test]
    fn hundred_ticks_not_empty() {
        let mut sim = SimRunner::new(123);
        sim.advance_ticks(100).unwrap();
        let m = sim.metrics();
        assert!(m.distinct_event_types >= 5);
        assert!(m.alive_persons > 0);
        assert!(m.active_gangs > 0);
    }
}
