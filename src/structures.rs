//! Visual primitive definitions for rendering Screeps structures.
//!
//! Each structure type is described as a sequence of drawing primitives with
//! relative offsets from the structure's center tile. The data is ported from
//! the community [RoomVisual.js](https://github.com/screepers/RoomVisual).

use crate::colors;

/// A single drawing primitive with coordinates relative to the structure center.
#[derive(Clone, Debug)]
pub enum VisualPrimitive {
    Circle {
        dx: f32,
        dy: f32,
        radius: f32,
        fill: Option<&'static str>,
        stroke: Option<&'static str>,
        stroke_width: f32,
        opacity: f32,
    },
    Rect {
        dx: f32,
        dy: f32,
        width: f32,
        height: f32,
        fill: Option<&'static str>,
        stroke: Option<&'static str>,
        stroke_width: f32,
        opacity: f32,
    },
    Poly {
        /// Relative offsets from center; will be translated by (x, y) at render time.
        points: &'static [(f32, f32)],
        fill: Option<&'static str>,
        stroke: Option<&'static str>,
        stroke_width: f32,
        opacity: f32,
    },
}

// ---------------------------------------------------------------------------
// Extension
// ---------------------------------------------------------------------------

const EXTENSION: &[VisualPrimitive] = &[
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.5,
        fill: Some(colors::DARK),
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.35,
        fill: Some(colors::GRAY),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Spawn
// ---------------------------------------------------------------------------

const SPAWN: &[VisualPrimitive] = &[
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.65,
        fill: Some(colors::DARK),
        stroke: Some("#CCCCCC"),
        stroke_width: 0.10,
        opacity: 1.0,
    },
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.40,
        fill: Some(colors::ENERGY),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Power Spawn
// ---------------------------------------------------------------------------

const POWER_SPAWN: &[VisualPrimitive] = &[
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.65,
        fill: Some(colors::DARK),
        stroke: Some(colors::POWER),
        stroke_width: 0.10,
        opacity: 1.0,
    },
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.40,
        fill: Some(colors::ENERGY),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Link
// ---------------------------------------------------------------------------

static LINK_OUTER: [(f32, f32); 5] = [
    (0.0, -0.5),
    (0.4, 0.0),
    (0.0, 0.5),
    (-0.4, 0.0),
    (0.0, -0.5),
];

static LINK_INNER: [(f32, f32); 5] = [
    (0.0, -0.3),
    (0.25, 0.0),
    (0.0, 0.3),
    (-0.25, 0.0),
    (0.0, -0.3),
];

const LINK: &[VisualPrimitive] = &[
    VisualPrimitive::Poly {
        points: &LINK_OUTER,
        fill: Some(colors::DARK),
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
    VisualPrimitive::Poly {
        points: &LINK_INNER,
        fill: Some(colors::GRAY),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Terminal
// ---------------------------------------------------------------------------

static TERMINAL_OUTER: [(f32, f32); 9] = [
    (0.0, -0.8),
    (0.55, -0.55),
    (0.8, 0.0),
    (0.55, 0.55),
    (0.0, 0.8),
    (-0.55, 0.55),
    (-0.8, 0.0),
    (-0.55, -0.55),
    (0.0, -0.8),
];

static TERMINAL_INNER: [(f32, f32); 9] = [
    (0.0, -0.65),
    (0.45, -0.45),
    (0.65, 0.0),
    (0.45, 0.45),
    (0.0, 0.65),
    (-0.45, 0.45),
    (-0.65, 0.0),
    (-0.45, -0.45),
    (0.0, -0.65),
];

const TERMINAL: &[VisualPrimitive] = &[
    VisualPrimitive::Poly {
        points: &TERMINAL_OUTER,
        fill: Some(colors::DARK),
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
    VisualPrimitive::Poly {
        points: &TERMINAL_INNER,
        fill: Some(colors::LIGHT),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
    VisualPrimitive::Rect {
        dx: -0.45,
        dy: -0.45,
        width: 0.9,
        height: 0.9,
        fill: Some(colors::GRAY),
        stroke: Some(colors::DARK),
        stroke_width: 0.1,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Lab
// ---------------------------------------------------------------------------

static LAB_BOX: [(f32, f32); 4] = [(-0.45, 0.3), (-0.45, 0.55), (0.45, 0.55), (0.45, 0.3)];

const LAB: &[VisualPrimitive] = &[
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: -0.025,
        radius: 0.55,
        fill: Some(colors::DARK),
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: -0.025,
        radius: 0.40,
        fill: Some(colors::GRAY),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
    VisualPrimitive::Rect {
        dx: -0.45,
        dy: 0.3,
        width: 0.9,
        height: 0.25,
        fill: Some(colors::DARK),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
    VisualPrimitive::Poly {
        points: &LAB_BOX,
        fill: None,
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Tower
// ---------------------------------------------------------------------------

const TOWER: &[VisualPrimitive] = &[
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.6,
        fill: Some(colors::DARK),
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
    VisualPrimitive::Rect {
        dx: -0.4,
        dy: -0.3,
        width: 0.8,
        height: 0.6,
        fill: Some(colors::GRAY),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
    VisualPrimitive::Rect {
        dx: -0.2,
        dy: -0.9,
        width: 0.4,
        height: 0.5,
        fill: Some(colors::LIGHT),
        stroke: Some(colors::DARK),
        stroke_width: 0.07,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Road
// ---------------------------------------------------------------------------

const ROAD_VIS: &[VisualPrimitive] = &[VisualPrimitive::Circle {
    dx: 0.0,
    dy: 0.0,
    radius: 0.175,
    fill: Some(colors::ROAD),
    stroke: None,
    stroke_width: 0.0,
    opacity: 1.0,
}];

// ---------------------------------------------------------------------------
// Rampart
// ---------------------------------------------------------------------------

const RAMPART: &[VisualPrimitive] = &[VisualPrimitive::Circle {
    dx: 0.0,
    dy: 0.0,
    radius: 0.65,
    fill: Some("#434C43"),
    stroke: Some("#5D735F"),
    stroke_width: 0.10,
    opacity: 1.0,
}];

// ---------------------------------------------------------------------------
// Wall
// ---------------------------------------------------------------------------

const WALL: &[VisualPrimitive] = &[VisualPrimitive::Circle {
    dx: 0.0,
    dy: 0.0,
    radius: 0.40,
    fill: Some(colors::DARK),
    stroke: Some(colors::LIGHT),
    stroke_width: 0.05,
    opacity: 1.0,
}];

// ---------------------------------------------------------------------------
// Storage
// ---------------------------------------------------------------------------

static STORAGE_OUTLINE: [(f32, f32); 9] = [
    (-0.45, -0.55),
    (0.0, -0.65),
    (0.45, -0.55),
    (0.55, 0.0),
    (0.45, 0.55),
    (0.0, 0.65),
    (-0.45, 0.55),
    (-0.55, 0.0),
    (-0.45, -0.55),
];

const STORAGE: &[VisualPrimitive] = &[
    VisualPrimitive::Poly {
        points: &STORAGE_OUTLINE,
        fill: Some(colors::DARK),
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
    VisualPrimitive::Rect {
        dx: -0.35,
        dy: -0.45,
        width: 0.7,
        height: 0.9,
        fill: Some(colors::ENERGY),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Observer
// ---------------------------------------------------------------------------

const OBSERVER: &[VisualPrimitive] = &[
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.45,
        fill: Some(colors::DARK),
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
    VisualPrimitive::Circle {
        dx: 0.225,
        dy: 0.0,
        radius: 0.20,
        fill: Some(colors::OUTLINE),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Nuker
// ---------------------------------------------------------------------------

static NUKER_OUTER: [(f32, f32); 6] = [
    (0.0, -1.0),
    (-0.47, 0.2),
    (-0.5, 0.5),
    (0.5, 0.5),
    (0.47, 0.2),
    (0.0, -1.0),
];

static NUKER_INNER: [(f32, f32); 4] = [(0.0, -0.80), (-0.40, 0.2), (0.40, 0.2), (0.0, -0.80)];

const NUKER: &[VisualPrimitive] = &[
    VisualPrimitive::Poly {
        points: &NUKER_OUTER,
        fill: Some(colors::DARK),
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
    VisualPrimitive::Poly {
        points: &NUKER_INNER,
        fill: Some(colors::GRAY),
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.01,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Container
// ---------------------------------------------------------------------------

const CONTAINER: &[VisualPrimitive] = &[
    VisualPrimitive::Rect {
        dx: -0.225,
        dy: -0.3,
        width: 0.45,
        height: 0.6,
        fill: Some(colors::GRAY),
        stroke: Some(colors::DARK),
        stroke_width: 0.09,
        opacity: 1.0,
    },
    VisualPrimitive::Rect {
        dx: -0.17,
        dy: 0.07,
        width: 0.34,
        height: 0.2,
        fill: Some(colors::ENERGY),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Factory
// ---------------------------------------------------------------------------

static FACTORY_OUTLINE: [(f32, f32); 28] = [
    (-0.68, -0.11),
    (-0.84, -0.18),
    (-0.84, -0.32),
    (-0.44, -0.44),
    (-0.32, -0.84),
    (-0.18, -0.84),
    (-0.11, -0.68),
    (0.11, -0.68),
    (0.18, -0.84),
    (0.32, -0.84),
    (0.44, -0.44),
    (0.84, -0.32),
    (0.84, -0.18),
    (0.68, -0.11),
    (0.68, 0.11),
    (0.84, 0.18),
    (0.84, 0.32),
    (0.44, 0.44),
    (0.32, 0.84),
    (0.18, 0.84),
    (0.11, 0.68),
    (-0.11, 0.68),
    (-0.18, 0.84),
    (-0.32, 0.84),
    (-0.44, 0.44),
    (-0.84, 0.32),
    (-0.84, 0.18),
    (-0.68, 0.11),
];

static FACTORY_SPIKES: [(f32, f32); 28] = [
    (-0.4, -0.1),
    (-0.8, -0.2),
    (-0.8, -0.3),
    (-0.4, -0.4),
    (-0.3, -0.8),
    (-0.2, -0.8),
    (-0.1, -0.4),
    (0.1, -0.4),
    (0.2, -0.8),
    (0.3, -0.8),
    (0.4, -0.4),
    (0.8, -0.3),
    (0.8, -0.2),
    (0.4, -0.1),
    (0.4, 0.1),
    (0.8, 0.2),
    (0.8, 0.3),
    (0.4, 0.4),
    (0.3, 0.8),
    (0.2, 0.8),
    (0.1, 0.4),
    (-0.1, 0.4),
    (-0.2, 0.8),
    (-0.3, 0.8),
    (-0.4, 0.4),
    (-0.8, 0.3),
    (-0.8, 0.2),
    (-0.4, 0.1),
];

const FACTORY: &[VisualPrimitive] = &[
    // Outer outline
    VisualPrimitive::Poly {
        points: &FACTORY_OUTLINE,
        fill: None,
        stroke: Some(colors::OUTLINE),
        stroke_width: 0.05,
        opacity: 1.0,
    },
    // Outer circle
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.65,
        fill: Some("#232323"),
        stroke: Some("#140a0a"),
        stroke_width: 0.035,
        opacity: 1.0,
    },
    // Spikes
    VisualPrimitive::Poly {
        points: &FACTORY_SPIKES,
        fill: Some(colors::GRAY),
        stroke: Some("#140a0a"),
        stroke_width: 0.04,
        opacity: 1.0,
    },
    // Factory level circle
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.54,
        fill: Some("#302a2a"),
        stroke: Some("#140a0a"),
        stroke_width: 0.04,
        opacity: 1.0,
    },
    // Inner black circle
    VisualPrimitive::Circle {
        dx: 0.0,
        dy: 0.0,
        radius: 0.42,
        fill: Some("#140a0a"),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
    // Inner rect
    VisualPrimitive::Rect {
        dx: -0.24,
        dy: -0.24,
        width: 0.48,
        height: 0.48,
        fill: Some("#3f3f3f"),
        stroke: None,
        stroke_width: 0.0,
        opacity: 1.0,
    },
];

// ---------------------------------------------------------------------------
// Extractor (not in RoomVisual.js — use a simple marker)
// ---------------------------------------------------------------------------

const EXTRACTOR: &[VisualPrimitive] = &[VisualPrimitive::Circle {
    dx: 0.0,
    dy: 0.0,
    radius: 0.5,
    fill: Some(colors::DARK),
    stroke: Some(colors::OUTLINE),
    stroke_width: 0.05,
    opacity: 1.0,
}];

// ---------------------------------------------------------------------------
// Default fallback
// ---------------------------------------------------------------------------

const DEFAULT_STRUCTURE: &[VisualPrimitive] = &[VisualPrimitive::Circle {
    dx: 0.0,
    dy: 0.0,
    radius: 0.35,
    fill: Some(colors::LIGHT),
    stroke: Some(colors::DARK),
    stroke_width: 0.20,
    opacity: 1.0,
}];

// ---------------------------------------------------------------------------
// Lookup
// ---------------------------------------------------------------------------

use crate::StructureType;

/// Returns the visual primitives for the given structure type.
///
/// The returned slice describes the drawing operations needed to render
/// the structure, with coordinates relative to the structure's center tile.
pub fn structure_primitives(structure_type: StructureType) -> &'static [VisualPrimitive] {
    match structure_type {
        StructureType::Extension => EXTENSION,
        StructureType::Spawn => SPAWN,
        StructureType::PowerSpawn => POWER_SPAWN,
        StructureType::Link => LINK,
        StructureType::Terminal => TERMINAL,
        StructureType::Lab => LAB,
        StructureType::Tower => TOWER,
        StructureType::Road => ROAD_VIS,
        StructureType::Rampart => RAMPART,
        StructureType::Wall => WALL,
        StructureType::Storage => STORAGE,
        StructureType::Observer => OBSERVER,
        StructureType::Nuker => NUKER,
        StructureType::Container => CONTAINER,
        StructureType::Factory => FACTORY,
        StructureType::Extractor => EXTRACTOR,
        _ => DEFAULT_STRUCTURE,
    }
}
