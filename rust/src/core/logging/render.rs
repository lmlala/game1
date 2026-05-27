// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use crate::core::events::{EventType, GameEvent};

pub fn render_event_log(event: &mut GameEvent) {
    if event.log_text.is_some() {
        return;
    }
    let tick = event.tick;
    let text = match event.event_type {
        EventType::EconomySettled => {
            let gang = event.payload.get("gang_name").cloned().unwrap_or_default();
            let delta = event.payload.get("delta").cloned().unwrap_or_default();
            format!("【第{tick}天】{gang} 完成日结, 财库变动 {delta} 两.")
        }
        EventType::FinancialCrisis => {
            let gang = event.payload.get("gang_name").cloned().unwrap_or_default();
            format!("【第{tick}天】{gang} 财政告急, 帮众开始嘀咕裁员和跳槽.")
        }
        EventType::PersonRecruited => {
            let person = event.payload.get("person_name").cloned().unwrap_or_default();
            let gang = event.payload.get("gang_name").cloned().unwrap_or_default();
            format!("【第{tick}天】{gang} 新招募 {person}, 场面热闹, 账本却更紧了.")
        }
        EventType::PersonLeftGang => {
            let person = event.payload.get("person_name").cloned().unwrap_or_default();
            format!("【第{tick}天】{person} 连夜收拾包袱离帮, 临走还顺走半袋米.")
        }
        EventType::PersonDefected => {
            let person = event.payload.get("person_name").cloned().unwrap_or_default();
            let to = event.payload.get("to_gang").cloned().unwrap_or_default();
            format!("【第{tick}天】{person} 投奔 {to}, 旧帮弟兄气得拍桌.")
        }
        EventType::MoneyStolen => {
            let person = event.payload.get("person_name").cloned().unwrap_or_default();
            let amount = event.payload.get("amount").cloned().unwrap_or_default();
            format!("【第{tick}天】{person} 趁乱偷走 {amount} 两, 还把欠条夹进账本.")
        }
        EventType::PersonPunished => {
            let person = event.payload.get("person_name").cloned().unwrap_or_default();
            format!("【第{tick}天】{person} 被当众处罚, 表面低头, 眼里却记下了仇.")
        }
        EventType::PersonRewarded => {
            let person = event.payload.get("person_name").cloned().unwrap_or_default();
            let amount = event.payload.get("amount").cloned().unwrap_or_default();
            format!("【第{tick}天】奖赏 {person} {amount} 两, 账房们各怀心思.")
        }
        EventType::DuelStarted => {
            let a = event.payload.get("attacker").cloned().unwrap_or_default();
            let b = event.payload.get("defender").cloned().unwrap_or_default();
            format!("【第{tick}天】{a} 与 {b} 当街对峙, 围观群众自觉后退三步.")
        }
        EventType::PersonInjured => {
            let person = event.payload.get("person_name").cloned().unwrap_or_default();
            format!("【第{tick}天】{person} 负伤倒地, 仍嘴硬说只是皮外伤.")
        }
        EventType::PersonKilled => {
            let person = event.payload.get("person_name").cloned().unwrap_or_default();
            format!("【第{tick}天】{person} 身亡, 江湖又多一笔烂账.")
        }
        EventType::BattleStarted => {
            let a = event.payload.get("attacker_gang").cloned().unwrap_or_default();
            let t = event.payload.get("territory").cloned().unwrap_or_default();
            format!("【第{tick}天】{a} 对 {t} 发动攻势, 街面瞬间鸡飞狗跳.")
        }
        EventType::BattleResolved => {
            let winner = event.payload.get("winner").cloned().unwrap_or_default();
            let territory = event.payload.get("territory").cloned().unwrap_or_default();
            format!("【第{tick}天】{territory} 战事收场, {winner} 暂时占了上风.")
        }
        EventType::TerritoryConquered => {
            let gang = event.payload.get("gang_name").cloned().unwrap_or_default();
            let territory = event.payload.get("territory").cloned().unwrap_or_default();
            format!("【第{tick}天】{gang} 拿下 {territory}, 保护费单子立刻重印.")
        }
        EventType::OfficialReported => {
            let reporter = event.payload.get("person_name").cloned().unwrap_or_default();
            format!("【第{tick}天】{reporter} 向官府递了状纸, 黑市空气都冷了一度.")
        }
        EventType::AbsurdAccident => {
            let detail = event.payload.get("detail").cloned().unwrap_or_default();
            format!("【第{tick}天】{detail}")
        }
        EventType::RumorSpread => {
            let topic = event.payload.get("topic").cloned().unwrap_or_default();
            format!("【第{tick}天】江湖传言: {topic}")
        }
        EventType::GangDefeated => {
            let gang = event.payload.get("gang_name").cloned().unwrap_or_default();
            format!("【第{tick}天】{gang} 势力瓦解, 地盘像烫手山芋一样被抢.")
        }
        EventType::PlayerCommandQueued => {
            let cmd = event.payload.get("command").cloned().unwrap_or_default();
            format!("【第{tick}天】帮主下令: {cmd}")
        }
        EventType::RelationshipChanged | EventType::MemoryAdded => {
            return;
        }
    };
    event.log_text = Some(text);
}

