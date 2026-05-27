// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventSeverity {
    Critical,
    Major,
    Normal,
    Minor,
}

impl EventSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::Major => "major",
            Self::Normal => "normal",
            Self::Minor => "minor",
        }
    }

    pub fn show_in_player_log(self) -> bool {
        matches!(self, Self::Critical | Self::Major | Self::Normal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    EconomySettled,
    FinancialCrisis,
    PersonRecruited,
    PersonLeftGang,
    PersonDefected,
    MoneyStolen,
    PersonPunished,
    PersonRewarded,
    DuelStarted,
    PersonInjured,
    PersonKilled,
    BattleStarted,
    BattleResolved,
    TerritoryConquered,
    RelationshipChanged,
    MemoryAdded,
    OfficialReported,
    AbsurdAccident,
    RumorSpread,
    GangDefeated,
    PlayerCommandQueued,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EconomySettled => "EconomySettled",
            Self::FinancialCrisis => "FinancialCrisis",
            Self::PersonRecruited => "PersonRecruited",
            Self::PersonLeftGang => "PersonLeftGang",
            Self::PersonDefected => "PersonDefected",
            Self::MoneyStolen => "MoneyStolen",
            Self::PersonPunished => "PersonPunished",
            Self::PersonRewarded => "PersonRewarded",
            Self::DuelStarted => "DuelStarted",
            Self::PersonInjured => "PersonInjured",
            Self::PersonKilled => "PersonKilled",
            Self::BattleStarted => "BattleStarted",
            Self::BattleResolved => "BattleResolved",
            Self::TerritoryConquered => "TerritoryConquered",
            Self::RelationshipChanged => "RelationshipChanged",
            Self::MemoryAdded => "MemoryAdded",
            Self::OfficialReported => "OfficialReported",
            Self::AbsurdAccident => "AbsurdAccident",
            Self::RumorSpread => "RumorSpread",
            Self::GangDefeated => "GangDefeated",
            Self::PlayerCommandQueued => "PlayerCommandQueued",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameEvent {
    pub schema_version: u32,
    pub tick: u32,
    pub event_id: String,
    pub event_type: EventType,
    pub severity: EventSeverity,
    pub actors: Vec<String>,
    pub targets: Vec<String>,
    pub payload: HashMap<String, String>,
    pub caused_by_event_id: Option<String>,
    pub log_text: Option<String>,
}

impl GameEvent {
    pub fn new(
        event_type: EventType,
        severity: EventSeverity,
        actors: Vec<String>,
        targets: Vec<String>,
    ) -> Self {
        Self {
            schema_version: 1,
            tick: 0,
            event_id: String::new(),
            event_type,
            severity,
            actors,
            targets,
            payload: HashMap::new(),
            caused_by_event_id: None,
            log_text: None,
        }
    }

    pub fn with_payload(mut self, key: &str, value: impl Into<String>) -> Self {
        self.payload.insert(key.to_string(), value.into());
        self
    }

    pub fn with_cause(mut self, id: Option<String>) -> Self {
        self.caused_by_event_id = id;
        self
    }

    pub fn with_log(mut self, text: impl Into<String>) -> Self {
        self.log_text = Some(text.into());
        self
    }
}
