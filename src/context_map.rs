use std::f32::consts::PI;

use bevy::{math::vec2, prelude::Vec2};
use once_cell::sync::Lazy;

pub const SEGMENT_OFFSETS_CHASE: isize = 2;
pub const SEGMENT_OFFSETS_AVOID: isize = SEGMENT_OFFSETS_CHASE - 1;
pub const SEGMENT_COUNT: usize = 12;
pub const SEGMENT_SIZE: f32 = PI * 2.0 / SEGMENT_COUNT as f32;
pub const SEGMENT_HALF: f32 = SEGMENT_SIZE / 2.0;

pub enum AddMethod {
    Dot(f32),
    DotWithMinimum(f32, f32),
    DotInverted(f32)
}

static SEGMENT_VECTORS: Lazy<Vec<Vec2>> = Lazy::new(|| (0..SEGMENT_COUNT)
    .map(|i| vec2((i as f32).cos(), (i as f32).sin()))
    .collect::<Vec<Vec2>>());

#[derive(Debug, Clone)]
pub struct ContextMap {
    pub values: [f32; SEGMENT_COUNT],
}

impl ContextMap {
    pub fn new() -> Self {
        Self {
            values: [0.0; SEGMENT_COUNT],
        }
    }

    pub fn print_debug(&self) -> Vec<String> {
        self.values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                format!(
                    "{}: {:.1} <-> {:.1} -> {}",
                    index,
                    (index as f32 * SEGMENT_SIZE - SEGMENT_HALF).to_degrees(),
                    ((index + 1) as f32 * SEGMENT_SIZE - SEGMENT_HALF).to_degrees(),
                    value
                )
            })
            .collect::<Vec<String>>()
    }

    pub fn mask(&self, context_map: &ContextMap) -> ContextMap {
        let mut masked = self.clone();
        // let mut lowest_value = None;
        // let mut segments_to_null = vec![];
        // for &value in context_map.values.iter() {
        //     if lowest_value.unwrap_or(INFINITY) > value {
        //         lowest_value = Some(value);
        //     }
        // };
        // for (index, &value) in context_map.values.iter().enumerate() {
        //     if value > lowest_value.unwrap_or(0.0) {
        //         segments_to_null.push(index);
        //     }
        // };
        // for i in segments_to_null {
        //     self.values[i] = 0.0;
        // }
        for (index, &value) in context_map.values.iter().enumerate() {
            masked.values[index] = (masked.values[index] - value).max(0.0);
        }
        masked
    }

    pub fn add(&mut self, vector: Vec2, add_method: AddMethod) {
        for (index, &segment_vector) in SEGMENT_VECTORS.iter().enumerate() {
            let dot_product = vector.dot(segment_vector);
            let value = match add_method {
                AddMethod::Dot(weight) => (dot_product * weight).max(0.0),
                AddMethod::DotWithMinimum(weight, minimum) => (weight * (minimum + dot_product * minimum)).max(0.0),
                AddMethod::DotInverted(weight) => (1.0 - dot_product * weight).abs()
            };
            self.values[index] = self.values[index].max(value);
        }
    }

    pub fn get_index_offset(&self, index: isize, offset: isize) -> usize {
        ((index + offset + SEGMENT_COUNT as isize) % SEGMENT_COUNT as isize) as usize
    }

    pub fn get_vector(&self, current_vector: &Vec2, segment_offset: isize) -> Vec2 {
        let index = if current_vector == &Vec2::ZERO {
            get_closest(current_vector.clone(), &self).unwrap_or(get_max(&self))
        } else {
            get_max(&self)
        };

        let mut total_vector = Vec2::ZERO;
        for offset in -segment_offset..=segment_offset {
            let offseted_index = self.get_index_offset(index as isize, offset);
            total_vector += SEGMENT_VECTORS[offseted_index] * self.values[offseted_index];
        }
        total_vector.normalize_or_zero()
    }

    pub fn is_empty(&self) -> bool {
        self.values.iter().all(|&v| v == 0.0)
    }
}


pub enum VectorGetter {
    Closest(Vec2),
    Maximum,
}

fn get_closest(v: Vec2, cm: &ContextMap) -> Option<usize> {
    let mut current_data = None;
    for (index, &value) in cm.values.iter().enumerate() {
        if value > 0.0 {
            let difference = SEGMENT_VECTORS[index].dot(v.clone());
            if let Some((_, current_difference)) = current_data {
                if difference > current_difference {
                    current_data = Some((index, difference));
                }
            } else {
                current_data = Some((index, difference));
            }
        }
    }
    current_data.and_then(|d| Some(d.0))
}

fn get_max(cm: &ContextMap) -> usize {
    let mut current_index = 0;
    for (index, &value) in cm.values.iter().skip(1).enumerate() {
        if value > cm.values[current_index] {
            current_index = index;
        }
    }
    current_index
}
