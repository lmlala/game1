// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use std::collections::HashSet;

use super::types::{EventSeverity, GameEvent};

#[derive(Debug, Clone, Default)]
pub struct EventStore {
    events: Vec<GameEvent>,
}

impl EventStore {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn all(&self) -> &[GameEvent] {
        &self.events
    }

    pub fn events_for_tick(&self, tick: u32) -> Vec<&GameEvent> {
        self.events.iter().filter(|e| e.tick == tick).collect()
    }

    pub fn recent_player_logs(&self, limit: usize) -> Vec<String> {
        self.events
            .iter()
            .filter(|e| e.severity.show_in_player_log())
            .filter_map(|e| e.log_text.clone())
            .rev()
            .take(limit)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    pub fn logs_for_entity(&self, entity_id: &str, limit: usize) -> Vec<String> {
        self.events
            .iter()
            .filter(|e| {
                e.actors.iter().any(|a| a == entity_id)
                    || e.targets.iter().any(|t| t == entity_id)
                    || e.payload.values().any(|v| v.contains(entity_id))
            })
            .filter(|e| e.severity.show_in_player_log())
            .filter_map(|e| e.log_text.clone())
            .rev()
            .take(limit)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    pub fn distinct_event_types(&self) -> HashSet<String> {
        self.events
            .iter()
            .map(|e| e.event_type.as_str().to_string())
            .collect()
    }

    pub fn count_by_severity(&self, severity: EventSeverity) -> usize {
        self.events
            .iter()
            .filter(|e| e.severity == severity)
            .count()
    }
}
