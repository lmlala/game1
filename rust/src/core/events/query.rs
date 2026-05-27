// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use std::collections::HashSet;

use crate::core::ids::EntityId;
use crate::core::world::WorldState;

use super::labels::{event_type_label_zh, parse_event_type_key, parse_severity_key, severity_label_zh};
use super::store::EventStore;
use super::types::{EventSeverity, GameEvent};

#[derive(Debug, Clone)]
pub struct EventHistoryFilters {
    pub gang_id: Option<String>,
    pub event_type: Option<String>,
    pub severity: Option<String>,
    pub entity_id: Option<String>,
    pub limit: usize,
}

impl Default for EventHistoryFilters {
    fn default() -> Self {
        Self {
            gang_id: None,
            event_type: None,
            severity: None,
            entity_id: None,
            limit: 80,
        }
    }
}

impl EventHistoryFilters {
    pub fn from_godot(
        gang_id: &str,
        event_type: &str,
        severity: &str,
        entity_id: &str,
        limit: i32,
    ) -> Self {
        let limit = if limit <= 0 { 80 } else { limit as usize };
        Self {
            gang_id: non_empty(gang_id),
            event_type: non_empty(event_type),
            severity: non_empty(severity),
            entity_id: non_empty(entity_id),
            limit,
        }
    }
}

fn non_empty(s: &str) -> Option<String> {
    if s.is_empty() || s == "*" {
        None
    } else {
        Some(s.to_string())
    }
}

impl EventStore {
    pub fn find_by_id(&self, event_id: &str) -> Option<&GameEvent> {
        self.all().iter().find(|e| e.event_id == event_id)
    }

    pub fn query_history_newest_first(
        &self,
        world: &WorldState,
        filters: &EventHistoryFilters,
    ) -> Vec<&GameEvent> {
        let type_filter = filters
            .event_type
            .as_deref()
            .and_then(parse_event_type_key);
        let severity_filter = filters
            .severity
            .as_deref()
            .and_then(parse_severity_key);

        self.all()
            .iter()
            .filter(|e| e.severity.show_in_player_log())
            .filter(|e| {
                if let Some(t) = &type_filter {
                    return &e.event_type == t;
                }
                true
            })
            .filter(|e| {
                if let Some(s) = severity_filter {
                    return e.severity == s;
                }
                true
            })
            .filter(|e| {
                if let Some(gid) = &filters.gang_id {
                    return event_involves_gang(e, gid, world);
                }
                true
            })
            .filter(|e| {
                if let Some(eid) = &filters.entity_id {
                    return event_involves_entity(e, eid);
                }
                true
            })
            .rev()
            .take(filters.limit)
            .collect()
    }

    pub fn related_events<'a>(
        &'a self,
        event: &'a GameEvent,
        limit: usize,
    ) -> Vec<&'a GameEvent> {
        let mut out: Vec<&GameEvent> = Vec::new();
        let mut seen = HashSet::new();
        seen.insert(event.event_id.clone());

        if let Some(cause_id) = &event.caused_by_event_id {
            if let Some(cause) = self.find_by_id(cause_id) {
                push_unique(&mut out, &mut seen, cause);
            }
        }

        for e in self.all() {
            if e.caused_by_event_id.as_deref() == Some(event.event_id.as_str()) {
                push_unique(&mut out, &mut seen, e);
            }
        }

        let participants = participant_ids(event);
        for e in self.all() {
            if e.event_id == event.event_id {
                continue;
            }
            if e.tick != event.tick {
                continue;
            }
            if shares_participant(e, &participants) {
                push_unique(&mut out, &mut seen, e);
            }
        }

        for e in self.all() {
            if e.event_id == event.event_id {
                continue;
            }
            if !shares_participant(e, &participants) {
                continue;
            }
            if (e.tick as i32 - event.tick as i32).abs() > 2 {
                continue;
            }
            push_unique(&mut out, &mut seen, e);
        }

        out.into_iter().take(limit).collect()
    }
}

fn push_unique<'a>(out: &mut Vec<&'a GameEvent>, seen: &mut HashSet<String>, e: &'a GameEvent) {
    if seen.insert(e.event_id.clone()) {
        out.push(e);
    }
}

pub fn event_summary(event: &GameEvent) -> String {
    event
        .log_text
        .clone()
        .unwrap_or_else(|| format!("[{}] 无文案", event.event_type.as_str()))
}

pub fn event_involves_entity(event: &GameEvent, entity_id: &str) -> bool {
    event.actors.iter().any(|a| a == entity_id)
        || event.targets.iter().any(|t| t == entity_id)
        || event.payload.values().any(|v| v.contains(entity_id))
}

pub fn event_involves_gang(event: &GameEvent, gang_id: &str, world: &WorldState) -> bool {
    if event.actors.iter().any(|a| a == gang_id) || event.targets.iter().any(|t| t == gang_id) {
        return true;
    }
    for id in event
        .actors
        .iter()
        .chain(event.targets.iter())
        .filter(|id| id.starts_with("person:"))
    {
        let eid = EntityId::new((*id).clone());
        if let Ok(person) = world.person(&eid) {
            if person.gang_id.as_str() == gang_id {
                return true;
            }
        }
    }
    if let Some(name) = event.payload.get("gang_name") {
        if world
            .gangs
            .values()
            .any(|g| g.id.as_str() == gang_id && &g.name == name)
        {
            return true;
        }
    }
    if let Some(attacker) = event.payload.get("attacker_gang") {
        if world
            .gangs
            .values()
            .any(|g| g.id.as_str() == gang_id && g.name == *attacker)
        {
            return true;
        }
    }
    if let Some(winner) = event.payload.get("winner") {
        if world
            .gangs
            .values()
            .any(|g| g.id.as_str() == gang_id && g.name == *winner)
        {
            return true;
        }
    }
    false
}

fn participant_ids(event: &GameEvent) -> HashSet<String> {
    let mut set = HashSet::new();
    for id in event.actors.iter().chain(event.targets.iter()) {
        set.insert(id.clone());
    }
    set
}

fn shares_participant(event: &GameEvent, participants: &HashSet<String>) -> bool {
    event
        .actors
        .iter()
        .chain(event.targets.iter())
        .any(|id| participants.contains(id))
}

pub fn event_type_key(event: &GameEvent) -> &str {
    event.event_type.as_str()
}

pub fn severity_key(event: &GameEvent) -> &str {
    event.severity.as_str()
}

pub fn type_label(event: &GameEvent) -> &'static str {
    event_type_label_zh(&event.event_type)
}

pub fn severity_label(event: &GameEvent) -> &'static str {
    severity_label_zh(event.severity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::events::types::{EventSeverity, EventType, GameEvent};

    #[test]
    fn filters_by_gang_actor() {
        let mut world = WorldState::new(1);
        let mut event = GameEvent::new(
            EventType::EconomySettled,
            EventSeverity::Normal,
            vec!["gang:black_tiger".into()],
            vec![],
        );
        event.log_text = Some("test".into());
        world.push_event(event);
        let filters = EventHistoryFilters {
            gang_id: Some("gang:black_tiger".into()),
            ..Default::default()
        };
        let hits = world.events.query_history_newest_first(&world, &filters);
        assert_eq!(hits.len(), 1);
    }
}
