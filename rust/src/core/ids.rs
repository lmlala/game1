// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use crate::core::error::{SimError, SimResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityId(pub String);

impl EntityId {
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    pub fn person(name: &str) -> Self {
        Self(format!("person:{name}"))
    }

    pub fn gang(name: &str) -> Self {
        Self(format!("gang:{name}"))
    }

    pub fn territory(name: &str) -> Self {
        Self(format!("territory:{name}"))
    }

    pub fn item(name: &str) -> Self {
        Self(format!("item:{name}"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn kind(&self) -> SimResult<&str> {
        self.0
            .split_once(':')
            .map(|(k, _)| k)
            .ok_or_else(|| SimError::InvalidEntityId(self.0.clone()))
    }
}

impl From<&str> for EntityId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
