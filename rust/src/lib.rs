// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

mod core;
mod nodes;

use godot::prelude::*;
pub use nodes::DemoController;

struct Game1Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Game1Extension {}
