// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use std::collections::HashMap;

use crate::core::error::{SimError, SimResult};
use crate::core::events::{EventStore, GameEvent};
use crate::core::ids::EntityId;
use crate::core::world::{Gang, Item, Person, Territory};

pub const PLAYER_GANG_ID: &str = "gang:black_tiger";

#[derive(Debug, Clone)]
pub struct WorldState {
    pub tick: u32,
    pub seed: u64,
    pub persons: HashMap<String, Person>,
    pub gangs: HashMap<String, Gang>,
    pub territories: HashMap<String, Territory>,
    pub items: HashMap<String, Item>,
    pub events: EventStore,
    pub player_log: Vec<String>,
    pub event_seq: u32,
}

impl WorldState {
    pub fn new(seed: u64) -> Self {
        Self {
            tick: 0,
            seed,
            persons: HashMap::new(),
            gangs: HashMap::new(),
            territories: HashMap::new(),
            items: HashMap::new(),
            events: EventStore::new(),
            player_log: Vec::new(),
            event_seq: 0,
        }
    }

    pub fn next_event_id(&mut self) -> String {
        self.event_seq += 1;
        format!("tick-{:04}-{:04}", self.tick, self.event_seq)
    }

    pub fn push_event(&mut self, mut event: GameEvent) {
        event.event_id = self.next_event_id();
        event.tick = self.tick;
        crate::core::logging::render_event_log(&mut event);
        if let Some(log) = &event.log_text {
            if event.severity.show_in_player_log() {
                self.player_log.push(log.clone());
            }
        }
        self.events.push(event);
    }

    pub fn person(&self, id: &EntityId) -> SimResult<&Person> {
        self.persons
            .get(id.as_str())
            .ok_or_else(|| SimError::EntityNotFound(id.as_str().to_string()))
    }

    pub fn person_mut(&mut self, id: &EntityId) -> SimResult<&mut Person> {
        self.persons
            .get_mut(id.as_str())
            .ok_or_else(|| SimError::EntityNotFound(id.as_str().to_string()))
    }

    pub fn gang(&self, id: &EntityId) -> SimResult<&Gang> {
        self.gangs
            .get(id.as_str())
            .ok_or_else(|| SimError::EntityNotFound(id.as_str().to_string()))
    }

    pub fn gang_mut(&mut self, id: &EntityId) -> SimResult<&mut Gang> {
        self.gangs
            .get_mut(id.as_str())
            .ok_or_else(|| SimError::EntityNotFound(id.as_str().to_string()))
    }

    pub fn territory(&self, id: &EntityId) -> SimResult<&Territory> {
        self.territories
            .get(id.as_str())
            .ok_or_else(|| SimError::EntityNotFound(id.as_str().to_string()))
    }

    pub fn territory_mut(&mut self, id: &EntityId) -> SimResult<&mut Territory> {
        self.territories
            .get_mut(id.as_str())
            .ok_or_else(|| SimError::EntityNotFound(id.as_str().to_string()))
    }

    pub fn item(&self, id: &EntityId) -> SimResult<&Item> {
        self.items
            .get(id.as_str())
            .ok_or_else(|| SimError::EntityNotFound(id.as_str().to_string()))
    }

    pub fn alive_person_ids(&self) -> Vec<EntityId> {
        let mut ids: Vec<EntityId> = self
            .persons
            .values()
            .filter(|p| p.alive)
            .map(|p| p.id.clone())
            .collect();
        ids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
        ids
    }

    pub fn territory_list(&self) -> Vec<Territory> {
        self.territories.values().cloned().collect()
    }
}
