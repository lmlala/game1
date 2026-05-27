// -- coding: utf-8 --
// Project: game1
// Created Date: 2026-05-27
// Author: liming
// Agent: Cursor
// Email: lmlala@aliyun.com
// Copyright (c) 2025 FiuAI

use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;

struct Game1Extension;

#[gdextension]
unsafe impl ExtensionLibrary for Game1Extension {}

/// 示例 Rust 节点: 在编辑器里把 Sprite2D 改为 Player 类型即可挂载
#[derive(GodotClass)]
#[class(base = Sprite2D)]
struct Player {
    angular_speed: f64,
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("game1_core: Player initialized");
        Self {
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
    }
}
