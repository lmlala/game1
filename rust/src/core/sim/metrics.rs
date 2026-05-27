// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use crate::core::events::EventType;
use crate::core::world::WorldState;

#[derive(Debug, Clone, Default)]
pub struct MetricsSnapshot {
    pub tick: u32,
    pub distinct_event_types: usize,
    pub absurd_accidents: usize,
    pub relationship_driven: usize,
    pub alive_persons: usize,
    pub active_gangs: usize,
    pub territory_changes: usize,
    pub avg_events_per_tick: f32,
}

impl MetricsSnapshot {
    pub fn from_world(world: &WorldState) -> Self {
        let distinct = world.events.distinct_event_types().len();
        let absurd = world
            .events
            .all()
            .iter()
            .filter(|e| e.event_type == EventType::AbsurdAccident)
            .count();
        let relationship = world
            .events
            .all()
            .iter()
            .filter(|e| {
                matches!(
                    e.event_type,
                    EventType::PersonDefected
                        | EventType::DuelStarted
                        | EventType::OfficialReported
                        | EventType::MemoryAdded
                )
            })
            .count();
        let alive_persons = world.persons.values().filter(|p| p.alive).count();
        let active_gangs = world.gangs.values().filter(|g| !g.defeated).count();
        let territory_changes = world
            .events
            .all()
            .iter()
            .filter(|e| e.event_type == EventType::TerritoryConquered)
            .count();
        let tick = world.tick.max(1);
        let avg_events_per_tick = world.events.all().len() as f32 / tick as f32;
        Self {
            tick: world.tick,
            distinct_event_types: distinct,
            absurd_accidents: absurd,
            relationship_driven: relationship,
            alive_persons,
            active_gangs,
            territory_changes,
            avg_events_per_tick,
        }
    }

    pub fn summary_line(&self) -> String {
        format!(
            "事件类型 {} | 荒诞 {} | 关系链 {} | 存活 {} 人 | 帮派 {} | 易主 {}",
            self.distinct_event_types,
            self.absurd_accidents,
            self.relationship_driven,
            self.alive_persons,
            self.active_gangs,
            self.territory_changes
        )
    }
}
