use std::marker::PhantomData;

use glam::{Vec2, Vec3};

pub struct DefaultVertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texcoord: Vec2,
}

pub trait Vertex {
    fn write_to_buffer(&self, buffer: &mut Vec<f32>);
}

impl Vertex for DefaultVertex {
    fn write_to_buffer(&self, buffer: &mut Vec<f32>) {
        buffer.extend_from_slice(&self.position.to_array());
        buffer.extend_from_slice(&self.normal.to_array());
        buffer.extend_from_slice(&self.texcoord.to_array());
    }
}

pub struct Mesh<V: Vertex = DefaultVertex> {
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<u32>,
    vertex_count: u32,
    _pd: PhantomData<fn(&V)>,
}

impl<V: Vertex> Mesh<V> {
    pub fn new() -> Self {
        Self {
            vertex_buffer: Vec::new(),
            vertex_count: 0,
            index_buffer: Vec::new(),
            _pd: PhantomData,
        }
    }

    pub fn add_vertex(&mut self, vertex: V) {
        vertex.write_to_buffer(&mut self.vertex_buffer);
        self.vertex_count += 1;
    }

    pub fn add_index(&mut self, index: u32) {
        self.index_buffer.push(index);
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn vertex_data(&self) -> &[f32] {
        &self.vertex_buffer
    }

    pub fn indices(&self) -> &[u32] {
        &self.index_buffer
    }
}
