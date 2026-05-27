// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use godot::builtin::{Array, Dictionary, PackedStringArray, Variant};
use godot::prelude::*;

use crate::core::events::query::{event_type_key, severity_key, severity_label, type_label};
use crate::core::events::{all_filter_event_types, all_filter_severities, event_summary, EventHistoryFilters, GameEvent};
use crate::core::ids::EntityId;
use crate::core::sim::SimRunner;
use crate::core::world::WorldState;

type VarDict = Dictionary<Variant, Variant>;

pub fn build_filter_meta(runner: &SimRunner) -> VarDict {
    let mut gangs_ids = PackedStringArray::new();
    let mut gangs_labels = PackedStringArray::new();
    gangs_ids.push("*");
    gangs_labels.push("全部帮派");
    let mut gangs: Vec<_> = runner.world.gangs.values().collect();
    gangs.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
    for gang in gangs {
        if gang.defeated {
            continue;
        }
        gangs_ids.push(gang.id.as_str());
        gangs_labels.push(gang.name.as_str());
    }

    let mut type_ids = PackedStringArray::new();
    let mut type_labels = PackedStringArray::new();
    type_ids.push("*");
    type_labels.push("全部类型");
    for (key, label) in all_filter_event_types() {
        type_ids.push(key);
        type_labels.push(label);
    }

    let mut sev_ids = PackedStringArray::new();
    let mut sev_labels = PackedStringArray::new();
    sev_ids.push("*");
    sev_labels.push("全部级别");
    for (key, label) in all_filter_severities() {
        sev_ids.push(key);
        sev_labels.push(label);
    }

    let mut entity_ids = PackedStringArray::new();
    let mut entity_labels = PackedStringArray::new();
    entity_ids.push("*");
    entity_labels.push("全部相关人");
    let mut persons: Vec<_> = runner.world.persons.values().collect();
    persons.sort_by(|a, b| a.name.cmp(&b.name));
    for person in persons {
        entity_ids.push(person.id.as_str());
        entity_labels.push(format!("{} [{}]", person.name, person.role).as_str());
    }

    let mut d = VarDict::new();
    d.set("gang_ids", &gangs_ids);
    d.set("gang_labels", &gangs_labels);
    d.set("type_ids", &type_ids);
    d.set("type_labels", &type_labels);
    d.set("severity_ids", &sev_ids);
    d.set("severity_labels", &sev_labels);
    d.set("entity_ids", &entity_ids);
    d.set("entity_labels", &entity_labels);
    d
}

pub fn query_history(
    runner: &SimRunner,
    gang_id: &str,
    event_type: &str,
    severity: &str,
    entity_id: &str,
    limit: i32,
) -> Array<Variant> {
    let filters = EventHistoryFilters::from_godot(gang_id, event_type, severity, entity_id, limit);
    let hits = runner
        .world
        .events
        .query_history_newest_first(&runner.world, &filters);
    let mut arr = Array::<Variant>::new();
    for event in hits {
        arr.push(&Variant::from(event_row_dict(event)));
    }
    arr
}

pub fn build_event_detail(runner: &SimRunner, event_id: &str) -> VarDict {
    let mut panel = VarDict::new();
    let Some(event) = runner.world.events.find_by_id(event_id) else {
        panel.set("found", false);
        panel.set("title", "未找到事件");
        return panel;
    };
    fill_detail_panel(&mut panel, &runner.world, event);
    panel
}

fn fill_detail_panel(panel: &mut VarDict, world: &WorldState, event: &GameEvent) {
    panel.set("found", true);
    panel.set("id", event.event_id.as_str());
    panel.set(
        "title",
        format!("第{}天 · {}", event.tick, type_label(event)).as_str(),
    );
    panel.set("tick", event.tick as i32);
    panel.set("type", event_type_key(event));
    panel.set("type_label", type_label(event));
    panel.set("severity", severity_key(event));
    panel.set("severity_label", severity_label(event));
    panel.set("log", event_summary(event).as_str());

    let mut attrs = VarDict::new();
    attrs.set("事件编号", event.event_id.as_str());
    attrs.set("发生天数", event.tick as i32);
    attrs.set("类型", type_label(event));
    attrs.set("级别", severity_label(event));
    if let Some(cause) = &event.caused_by_event_id {
        attrs.set("起因事件", cause.as_str());
    }
    for (k, v) in &event.payload {
        attrs.set(k.as_str(), v.as_str());
    }
    panel.set("attributes", &attrs);

    let mut participants = Array::<Variant>::new();
    let mut seen = std::collections::HashSet::new();
    for id in event.actors.iter().chain(event.targets.iter()) {
        if !seen.insert(id.clone()) {
            continue;
        }
        participants.push(&Variant::from(entity_ref_dict(world, id)));
    }
    panel.set("participants", &participants);

    let related = world.events.related_events(event, 12);
    let mut related_arr = Array::<Variant>::new();
    for rel in related {
        let mut row = VarDict::new();
        row.set("id", rel.event_id.as_str());
        row.set("tick", rel.tick as i32);
        row.set("summary", event_summary(rel).as_str());
        row.set("type_label", type_label(rel));
        related_arr.push(&Variant::from(row));
    }
    panel.set("related_events", &related_arr);
}

fn event_row_dict(event: &GameEvent) -> VarDict {
    let mut d = VarDict::new();
    d.set("id", event.event_id.as_str());
    d.set("tick", event.tick as i32);
    d.set("type", event_type_key(event));
    d.set("type_label", type_label(event));
    d.set("severity", severity_key(event));
    d.set("severity_label", severity_label(event));
    d.set("summary", event_summary(event).as_str());
    d
}

fn entity_ref_dict(world: &WorldState, entity_id: &str) -> VarDict {
    let mut d = VarDict::new();
    d.set("id", entity_id);
    if entity_id.starts_with("person:") {
        let eid = EntityId::new(entity_id);
        if let Ok(person) = world.person(&eid) {
            d.set("kind", "person");
            d.set(
                "label",
                format!("{} [{}]", person.name, person.role).as_str(),
            );
            return d;
        }
    }
    if entity_id.starts_with("gang:") {
        let eid = EntityId::new(entity_id);
        if let Ok(gang) = world.gang(&eid) {
            d.set("kind", "gang");
            d.set("label", gang.name.as_str());
            return d;
        }
    }
    if entity_id.starts_with("territory:") {
        let eid = EntityId::new(entity_id);
        if let Ok(t) = world.territory(&eid) {
            d.set("kind", "territory");
            d.set("label", t.name.as_str());
            return d;
        }
    }
    d.set("kind", "unknown");
    d.set("label", entity_id);
    d
}
