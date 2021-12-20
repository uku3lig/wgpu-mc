// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct EntityVertex {
//     position: [f32; 3],
//     uv: [f32; 2],
//     normal: [f32; 3],
//     part: u8,
//     ///Doesn't matter what this value is
//     dummy: u8
// }
//
// impl EntityVertex {
//     #[must_use]
//     pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
//         use std::mem;
//         wgpu::VertexBufferLayout {
//             array_stride: mem::size_of::<EntityVertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &[
//                 //Position
//                 wgpu::VertexAttribute {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::Float32x3,
//                 },
//                 //Texcoords
//                 wgpu::VertexAttribute {
//                     offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
//                     shader_location: 1,
//                     format: wgpu::VertexFormat::Float32x2,
//                 },
//                 //Normal
//                 wgpu::VertexAttribute {
//                     offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
//                     shader_location: 2,
//                     format: wgpu::VertexFormat::Float32x3,
//                 },
//                 //Part
//                 wgpu::VertexAttribute {
//                     offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
//                     shader_location: 3,
//                     format: wgpu::VertexFormat::Uint8x2
//                 }
//             ],
//         }
//     }
// }

// struct EntityInstance {
//     transform: [[f32; 3]; 3]
// }
//
// impl EntityInstance {
//
//     pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
//         use std::mem;
//         wgpu::VertexBufferLayout {
//             array_stride: mem::size_of::<EntityVertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &[
//                 //Transform
//                 wgpu::VertexAttribute {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::,
//                 },
//                 //Part
//                 wgpu::VertexAttribute {
//                     offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
//                     shader_location: 3,
//                     format: wgpu::VertexFormat::Uint8x2
//                 }
//             ],
//         }
//     }
//
// }