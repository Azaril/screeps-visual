//! Backend-agnostic rendering of structure visuals.
//!
//! Implement [`VisualBackend`] for your drawing surface, then call
//! [`render_structure`] to emit the correct primitives for any structure type.

use crate::structures::{structure_primitives, VisualPrimitive};
use crate::StructureType;

/// Trait for anything that can draw the basic visual primitives.
///
/// Coordinates are absolute room positions (0..49).
#[allow(clippy::too_many_arguments)]
pub trait VisualBackend {
    fn circle(
        &mut self,
        x: f32,
        y: f32,
        radius: f32,
        fill: Option<&str>,
        stroke: Option<&str>,
        stroke_width: f32,
        opacity: f32,
    );

    fn rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        fill: Option<&str>,
        stroke: Option<&str>,
        stroke_width: f32,
        opacity: f32,
    );

    fn poly(
        &mut self,
        points: &[(f32, f32)],
        fill: Option<&str>,
        stroke: Option<&str>,
        stroke_width: f32,
        opacity: f32,
    );

    fn line(
        &mut self,
        from: (f32, f32),
        to: (f32, f32),
        color: Option<&str>,
        width: f32,
        opacity: f32,
    );
}

/// Render a structure at the given room position using the provided backend.
///
/// `opacity` is multiplied with each primitive's own opacity, allowing the
/// caller to fade the entire structure (e.g. for planned vs built).
pub fn render_structure<V: VisualBackend>(
    vis: &mut V,
    x: f32,
    y: f32,
    structure_type: StructureType,
    opacity: f32,
) {
    let primitives = structure_primitives(structure_type);

    for prim in primitives {
        match prim {
            VisualPrimitive::Circle {
                dx,
                dy,
                radius,
                fill,
                stroke,
                stroke_width,
                opacity: prim_opacity,
            } => {
                vis.circle(
                    x + dx,
                    y + dy,
                    *radius,
                    *fill,
                    *stroke,
                    *stroke_width,
                    opacity * prim_opacity,
                );
            }
            VisualPrimitive::Rect {
                dx,
                dy,
                width,
                height,
                fill,
                stroke,
                stroke_width,
                opacity: prim_opacity,
            } => {
                vis.rect(
                    x + dx,
                    y + dy,
                    *width,
                    *height,
                    *fill,
                    *stroke,
                    *stroke_width,
                    opacity * prim_opacity,
                );
            }
            VisualPrimitive::Poly {
                points,
                fill,
                stroke,
                stroke_width,
                opacity: prim_opacity,
            } => {
                // Translate relative points to absolute positions.
                let abs_points: Vec<(f32, f32)> =
                    points.iter().map(|(px, py)| (x + px, y + py)).collect();

                vis.poly(
                    &abs_points,
                    *fill,
                    *stroke,
                    *stroke_width,
                    opacity * prim_opacity,
                );
            }
        }
    }
}
