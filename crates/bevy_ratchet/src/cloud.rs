use bevy::{prelude::*, reflect::serde};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CloudSpec {
    Circle {
        #[serde(default = "one")]         radius: f32,
        #[serde(default = "twelve")]      rings: usize,
        #[serde(default = "twenty_four")] points_per_ring: usize,
        #[serde(default)]                 center: Vec3,
    },
    Rect {
        #[serde(default = "two")]         width: f32,
        #[serde(default = "one")]         height: f32,
        #[serde(default = "twenty")]      cols: usize,
        #[serde(default = "ten")]         rows: usize,
        #[serde(default)]                 center: Vec3,
    },
}

fn one() -> f32 { 1.0 }
fn two() -> f32 { 2.0 }
fn ten() -> f32 { 10.0 }
fn twelve() -> f32 { 12.0 }
fn twenty() -> f32 { 20.0 }
fn twenty_four() -> f32 { 24.0 }
