// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use super::types::{EventSeverity, EventType};

pub fn event_type_label_zh(event_type: &EventType) -> &'static str {
    match event_type {
        EventType::EconomySettled => "日结",
        EventType::FinancialCrisis => "财政危机",
        EventType::PersonRecruited => "招募",
        EventType::PersonLeftGang => "离帮",
        EventType::PersonDefected => "叛逃",
        EventType::MoneyStolen => "盗银",
        EventType::PersonPunished => "处罚",
        EventType::PersonRewarded => "奖赏",
        EventType::DuelStarted => "决斗",
        EventType::PersonInjured => "负伤",
        EventType::PersonKilled => "身亡",
        EventType::BattleStarted => "开战",
        EventType::BattleResolved => "战事收场",
        EventType::TerritoryConquered => "夺地盘",
        EventType::RelationshipChanged => "关系变化",
        EventType::MemoryAdded => "记忆",
        EventType::OfficialReported => "报官",
        EventType::AbsurdAccident => "荒诞意外",
        EventType::RumorSpread => "流言",
        EventType::GangDefeated => "帮派覆灭",
        EventType::PlayerCommandQueued => "帮主号令",
    }
}

pub fn severity_label_zh(severity: EventSeverity) -> &'static str {
    match severity {
        EventSeverity::Critical => "危急",
        EventSeverity::Major => "重大",
        EventSeverity::Normal => "普通",
        EventSeverity::Minor => "琐碎",
    }
}

pub fn all_filter_event_types() -> Vec<(&'static str, &'static str)> {
    use EventType::*;
    [
        (EconomySettled, "日结"),
        (FinancialCrisis, "财政危机"),
        (PersonRecruited, "招募"),
        (PersonLeftGang, "离帮"),
        (PersonDefected, "叛逃"),
        (MoneyStolen, "盗银"),
        (PersonPunished, "处罚"),
        (PersonRewarded, "奖赏"),
        (DuelStarted, "决斗"),
        (PersonInjured, "负伤"),
        (PersonKilled, "身亡"),
        (BattleStarted, "开战"),
        (BattleResolved, "战事收场"),
        (TerritoryConquered, "夺地盘"),
        (OfficialReported, "报官"),
        (AbsurdAccident, "荒诞意外"),
        (RumorSpread, "流言"),
        (GangDefeated, "帮派覆灭"),
        (PlayerCommandQueued, "帮主号令"),
    ]
    .into_iter()
    .map(|(t, label)| (t.as_str(), label))
    .collect()
}

pub fn all_filter_severities() -> Vec<(&'static str, &'static str)> {
    use EventSeverity::*;
    [
        (Critical, "危急"),
        (Major, "重大"),
        (Normal, "普通"),
        (Minor, "琐碎"),
    ]
    .into_iter()
    .map(|(s, label)| (s.as_str(), label))
    .collect()
}

pub fn parse_event_type_key(key: &str) -> Option<EventType> {
    if key.is_empty() || key == "*" {
        return None;
    }
    all_filter_event_types()
        .into_iter()
        .find(|(k, _)| *k == key)
        .map(|(k, _)| parse_event_type_key_strict(k))
        .flatten()
}

fn parse_event_type_key_strict(key: &str) -> Option<EventType> {
    use EventType::*;
    Some(match key {
        "EconomySettled" => EconomySettled,
        "FinancialCrisis" => FinancialCrisis,
        "PersonRecruited" => PersonRecruited,
        "PersonLeftGang" => PersonLeftGang,
        "PersonDefected" => PersonDefected,
        "MoneyStolen" => MoneyStolen,
        "PersonPunished" => PersonPunished,
        "PersonRewarded" => PersonRewarded,
        "DuelStarted" => DuelStarted,
        "PersonInjured" => PersonInjured,
        "PersonKilled" => PersonKilled,
        "BattleStarted" => BattleStarted,
        "BattleResolved" => BattleResolved,
        "TerritoryConquered" => TerritoryConquered,
        "RelationshipChanged" => RelationshipChanged,
        "MemoryAdded" => MemoryAdded,
        "OfficialReported" => OfficialReported,
        "AbsurdAccident" => AbsurdAccident,
        "RumorSpread" => RumorSpread,
        "GangDefeated" => GangDefeated,
        "PlayerCommandQueued" => PlayerCommandQueued,
        _ => return None,
    })
}

pub fn parse_severity_key(key: &str) -> Option<EventSeverity> {
    if key.is_empty() || key == "*" {
        return None;
    }
    match key {
        "critical" => Some(EventSeverity::Critical),
        "major" => Some(EventSeverity::Major),
        "normal" => Some(EventSeverity::Normal),
        "minor" => Some(EventSeverity::Minor),
        _ => None,
    }
}
