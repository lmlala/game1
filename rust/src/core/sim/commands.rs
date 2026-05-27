// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use crate::core::error::{SimError, SimResult};
use crate::core::ids::EntityId;
use crate::core::world::{GangStrategy, WorldState};

#[derive(Debug, Clone)]
pub enum PlayerCommand {
    Expand {
        gang_id: EntityId,
        territory_id: EntityId,
    },
    Defend {
        gang_id: EntityId,
        territory_id: EntityId,
    },
    RewardPerson {
        person_id: EntityId,
        money: i64,
    },
    PunishPerson {
        person_id: EntityId,
    },
    Recruit {
        gang_id: EntityId,
        budget: i64,
    },
}

impl PlayerCommand {
    pub fn validate(&self, world: &WorldState) -> SimResult<()> {
        match self {
            Self::Expand { gang_id, territory_id } | Self::Defend { gang_id, territory_id } => {
                let gang = world.gang(gang_id)?;
                if gang.defeated {
                    return Err(SimError::InvalidCommand("gang defeated".into()));
                }
                let territory = world.territory(territory_id)?;
                if !territory.controllable {
                    return Err(SimError::TerritoryNotControllable(
                        territory_id.as_str().to_string(),
                    ));
                }
            }
            Self::RewardPerson { person_id, money } => {
                let person = world.person(person_id)?;
                if !person.alive {
                    return Err(SimError::PersonDead(person_id.as_str().to_string()));
                }
                if *money <= 0 || *money > 500 {
                    return Err(SimError::InvalidRange {
                        field: "money".into(),
                        value: money.to_string(),
                    });
                }
                let gang = world.gang(&person.gang_id)?;
                if gang.money < *money {
                    return Err(SimError::InvalidCommand("insufficient gang money".into()));
                }
            }
            Self::PunishPerson { person_id } => {
                let person = world.person(person_id)?;
                if !person.alive {
                    return Err(SimError::PersonDead(person_id.as_str().to_string()));
                }
            }
            Self::Recruit { gang_id, budget } => {
                let gang = world.gang(gang_id)?;
                if gang.defeated {
                    return Err(SimError::InvalidCommand("gang defeated".into()));
                }
                if *budget <= 0 || *budget > 300 {
                    return Err(SimError::InvalidRange {
                        field: "budget".into(),
                        value: budget.to_string(),
                    });
                }
                if gang.money < *budget {
                    return Err(SimError::InvalidCommand("insufficient recruit budget".into()));
                }
            }
        }
        Ok(())
    }

    pub fn describe(&self) -> String {
        match self {
            Self::Expand { territory_id, .. } => format!("扩张 {}", territory_id.as_str()),
            Self::Defend { territory_id, .. } => format!("防守 {}", territory_id.as_str()),
            Self::RewardPerson { person_id, money } => {
                format!("奖赏 {} {} 两", person_id.as_str(), money)
            }
            Self::PunishPerson { person_id } => format!("处罚 {}", person_id.as_str()),
            Self::Recruit { budget, .. } => format!("招募 预算 {} 两", budget),
        }
    }
}

pub fn apply_command(world: &mut WorldState, command: &PlayerCommand) -> SimResult<()> {
    command.validate(world)?;
    match command {
        PlayerCommand::Expand {
            gang_id,
            territory_id,
        } => {
            let gang = world.gang_mut(gang_id)?;
            gang.strategy = GangStrategy::Expand;
            gang.focus_territory = Some(territory_id.clone());
        }
        PlayerCommand::Defend {
            gang_id,
            territory_id,
        } => {
            let gang = world.gang_mut(gang_id)?;
            gang.strategy = GangStrategy::Defend;
            gang.focus_territory = Some(territory_id.clone());
        }
        PlayerCommand::RewardPerson { person_id, money } => {
            let person_gang = world.person(person_id)?.gang_id.clone();
            let person_name = world.person(person_id)?.name.clone();
            {
                let gang = world.gang_mut(&person_gang)?;
                gang.money -= *money;
            }
            {
                let person = world.person_mut(person_id)?;
                person.loyalty = (person.loyalty + 0.15).min(1.0);
            }
            let mut event = crate::core::events::GameEvent::new(
                crate::core::events::EventType::PersonRewarded,
                crate::core::events::EventSeverity::Normal,
                vec![person_id.as_str().to_string()],
                vec![person_gang.as_str().to_string()],
            );
            event
                .payload
                .insert("person_name".into(), person_name);
            event
                .payload
                .insert("amount".into(), money.to_string());
            world.push_event(event);
        }
        PlayerCommand::PunishPerson { person_id } => {
            let person_name = world.person(person_id)?.name.clone();
            let target = person_id.as_str().to_string();
            {
                let person = world.person_mut(person_id)?;
                person.loyalty = (person.loyalty - 0.2).max(0.0);
                person
                    .relationships
                    .entry("gang:black_tiger".into())
                    .and_modify(|v| *v -= 0.1)
                    .or_insert(-0.1);
            }
            let mut event = crate::core::events::GameEvent::new(
                crate::core::events::EventType::PersonPunished,
                crate::core::events::EventSeverity::Major,
                vec![target],
                vec![],
            );
            event.payload.insert("person_name".into(), person_name);
            world.push_event(event);
        }
        PlayerCommand::Recruit { gang_id, budget } => {
            let gang = world.gang_mut(gang_id)?;
            gang.money -= *budget;
            gang.influence += 0.05;
        }
    }
    Ok(())
}
