# screeps-visual

Rust crate providing structure rendering definitions for [Screeps](https://screeps.com/) room visuals, ported from the community [RoomVisual.js](https://github.com/screepers/RoomVisual).

Each structure type (spawn, extension, tower, lab, etc.) is described as a sequence of drawing primitives (circles, rects, polygons) with the same colors and proportions used by the original JavaScript implementation.

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
screeps-visual = { git = "https://github.com/Azaril/screeps-visual" }
```

### Implement `VisualBackend`

The crate is backend-agnostic. Implement the `VisualBackend` trait for whatever drawing surface you use (the Screeps `RoomVisual` API, an off-screen buffer, an SVG renderer, etc.):

```rust
use screeps_visual::render::VisualBackend;

struct MyVisualizer { /* ... */ }

impl VisualBackend for MyVisualizer {
    fn circle(
        &mut self, x: f32, y: f32, radius: f32,
        fill: Option<&str>, stroke: Option<&str>,
        stroke_width: f32, opacity: f32,
    ) {
        // draw a circle at (x, y)
    }

    fn rect(
        &mut self, x: f32, y: f32, w: f32, h: f32,
        fill: Option<&str>, stroke: Option<&str>,
        stroke_width: f32, opacity: f32,
    ) {
        // draw a rectangle
    }

    fn poly(
        &mut self, points: &[(f32, f32)],
        fill: Option<&str>, stroke: Option<&str>,
        stroke_width: f32, opacity: f32,
    ) {
        // draw a polygon
    }

    fn line(
        &mut self, from: (f32, f32), to: (f32, f32),
        color: Option<&str>, width: f32, opacity: f32,
    ) {
        // draw a line
    }
}
```

### Render a structure

```rust
use screeps::constants::StructureType;
use screeps_visual::render::render_structure;

// Draw a tower at room position (25, 30) with full opacity:
render_structure(&mut my_visualizer, 25.0, 30.0, StructureType::Tower, 1.0);

// Draw a planned extension at half opacity:
render_structure(&mut my_visualizer, 10.0, 12.0, StructureType::Extension, 0.5);
```

### Access raw primitive data

If you need the primitive definitions directly (e.g. for serialization or analysis), use `structure_primitives`:

```rust
use screeps_visual::structures::{structure_primitives, VisualPrimitive};

let primitives = structure_primitives(StructureType::Lab);
for prim in primitives {
    // inspect or transform each VisualPrimitive
}
```

### Color constants

The `colors` module exposes the standard palette from RoomVisual.js:

```rust
use screeps_visual::colors;

assert_eq!(colors::DARK, "#181818");
assert_eq!(colors::ENERGY, "#FFE87B");
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `screeps-game-api` | yes | Uses the real `StructureType` from screeps-game-api |
| `shim` | no | Provides a standalone `StructureType` enum for use without the game API (e.g. benchmarks, tests) |

## Structure coverage

All structure types from RoomVisual.js are supported:

- Extension, Spawn, Power Spawn
- Link, Terminal, Lab
- Tower, Road, Rampart, Wall
- Storage, Observer, Nuker
- Container, Factory, Extractor

Unknown structure types fall back to a generic marker circle.

## License

Same license as the parent project.
