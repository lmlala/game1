// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use std::collections::HashMap;

use crate::core::ids::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GangStrategy {
    Balanced,
    Expand,
    Defend,
}

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub about: EntityId,
    pub label: String,
    pub weight: f32,
    pub tick: u32,
}

#[derive(Debug, Clone)]
pub struct Person {
    pub id: EntityId,
    pub name: String,
    pub gang_id: EntityId,
    pub role: String,
    pub alive: bool,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub desires: HashMap<String, f32>,
    pub personality: HashMap<String, f32>,
    pub status: HashMap<String, bool>,
    pub relationships: HashMap<String, f32>,
    pub memories: Vec<MemoryEntry>,
    pub loyalty: f32,
}

#[derive(Debug, Clone)]
pub struct Gang {
    pub id: EntityId,
    pub name: String,
    pub leader_id: EntityId,
    pub money: i64,
    pub influence: f32,
    pub morale: f32,
    pub member_ids: Vec<EntityId>,
    pub strategy: GangStrategy,
    pub focus_territory: Option<EntityId>,
    pub deficit_ticks: u32,
    pub defeated: bool,
}

#[derive(Debug, Clone)]
pub struct Territory {
    pub id: EntityId,
    pub name: String,
    pub revenue: i32,
    pub defense: i32,
    pub controller_gang_id: Option<EntityId>,
    pub controllable: bool,
    pub risk_tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: EntityId,
    pub name: String,
    pub kind: String,
    pub owner_id: Option<EntityId>,
    pub description: String,
    pub value: i32,
}

impl Person {
    pub fn clamp_fields(&mut self) {
        self.hp = self.hp.clamp(0, 100);
        self.loyalty = self.loyalty.clamp(0.0, 1.0);
        for v in self.desires.values_mut() {
            *v = v.clamp(0.0, 1.0);
        }
        for v in self.relationships.values_mut() {
            *v = v.clamp(-1.0, 1.0);
        }
    }
}

impl Gang {
    pub fn daily_income(&self, territories: &[Territory]) -> i64 {
        territories
            .iter()
            .filter(|t| t.controller_gang_id.as_ref() == Some(&self.id))
            .map(|t| i64::from(t.revenue))
            .sum()
    }

    pub fn daily_expense(&self) -> i64 {
        20 + self.member_ids.len() as i64 * 8
    }
}
