//! DOM observer modules: MutationObserver, IntersectionObserver, ResizeObserver.

#![allow(dead_code)]

pub mod intersection_compute;
pub mod intersection_observer;
pub mod intersection_types;
pub mod mutation_delivery;
pub mod mutation_observer;
pub mod mutation_record;
pub mod mutation_types;
pub mod resize_observer;
pub mod resize_types;

#[cfg(test)]
mod mutation_tests;
