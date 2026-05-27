// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimError {
    InvalidEntityId(String),
    EntityNotFound(String),
    InvalidCommand(String),
    InvalidRange { field: String, value: String },
    GangHasNoTerritory(String),
    PersonDead(String),
    TerritoryNotControllable(String),
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEntityId(id) => write!(f, "invalid entity id: {id}"),
            Self::EntityNotFound(id) => write!(f, "entity not found: {id}"),
            Self::InvalidCommand(msg) => write!(f, "invalid command: {msg}"),
            Self::InvalidRange { field, value } => {
                write!(f, "invalid range for {field}: {value}")
            }
            Self::GangHasNoTerritory(id) => write!(f, "gang has no territory: {id}"),
            Self::PersonDead(id) => write!(f, "person is dead: {id}"),
            Self::TerritoryNotControllable(id) => write!(f, "territory not controllable: {id}"),
        }
    }
}

impl std::error::Error for SimError {}

pub type SimResult<T> = Result<T, SimError>;
