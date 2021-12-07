use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use crate::mc::block::{Block};
use crate::mc::datapack::{FaceTexture, TagOrResource, NamespacedResource};
use crate::mc::datapack;
use crate::mc::resource::ResourceProvider;
use crate::model::MeshVertex;
use crate::render::atlas::{ATLAS_DIMENSIONS, TextureManager};
use crate::texture::UV;
use crate::mc::block::blockstate::BlockstateVariantModelDefinitionRotations;
use cgmath::Vector3;

#[derive(Debug)]
pub struct BlockModelFaces {
    pub north: Option<[MeshVertex; 6]>,
    pub east: Option<[MeshVertex; 6]>,
    pub south: Option<[MeshVertex; 6]>,
    pub west: Option<[MeshVertex; 6]>,
    pub up: Option<[MeshVertex; 6]>,
    pub down: Option<[MeshVertex; 6]>,
}

#[derive(Debug)]
///Makes chunk mesh baking a bit faster
pub enum CubeOrComplexMesh {
    Cube(BlockModelFaces),
    Custom(Vec<BlockModelFaces>),
}

pub struct BlockstateVariantMesh {
    pub name: NamespacedResource,
    pub shape: CubeOrComplexMesh,
    pub transparent_or_complex: bool
}

impl BlockstateVariantMesh {
    pub fn absolute_atlas_uv(
        face: &FaceTexture,
        tex_manager: &TextureManager
    ) -> Option<UV> {
        let atlases = tex_manager.atlases.load();

        let atlas_uv = atlases.block.map.get(face.texture
            .as_resource()
            .expect(
                &format!("{:?}", face)
            )
        ).copied().unwrap();

        let face_uv = &face.uv;

        const ATLAS: f32 = ATLAS_DIMENSIONS as f32;

        let adjusted_uv = (
            (
                (atlas_uv.0.0 + face_uv.0.0) / ATLAS,
                (atlas_uv.0.1 + face_uv.0.1) / ATLAS
            ),
            (
                (atlas_uv.1.0 + face_uv.1.0) / ATLAS,
                (atlas_uv.1.1 + face_uv.1.1) / ATLAS
            )
        );

        Some(adjusted_uv)
    }

    pub fn bake_block_model(
        model: &datapack::BlockModel,
        rp: &dyn ResourceProvider,
        tex_manager: &TextureManager,
        transform: &BlockstateVariantModelDefinitionRotations
    ) -> Option<Self> {
        let texture_ids = &model.textures;

        // let textures: HashMap<String, UV> = texture_ids.iter().map(|(key, identifier)| {
        //     Some(
        //         (key.clone(),
        //          tex_manager.atlases.load()
        //              .block.map.get(
        //                  identifier.as_resource()?
        //              ).copied()?
        //         )
        //     )
        // }).collect::<Option<HashMap<String, UV>>>()?;

        let is_cube = model.elements.len() == 1 && {
            let first = model.elements.first().unwrap();

            first.from.0 == 0.0
                && first.from.1 == 0.0
                && first.from.2 == 0.0
                && first.to.0 == 1.0
                && first.to.1 == 1.0
                && first.to.2 == 1.0
        };

        let mut results = model
            .elements
            .iter()
            .map(|element| {
                //Face textures

                let north = element.face_textures.north.as_ref().and_then(|tex| {
                    Some(Self::absolute_atlas_uv(
                        tex,
                        tex_manager,
                    )?)
                });

                let east = element.face_textures.east.as_ref().and_then(|tex| {
                    Some(Self::absolute_atlas_uv(
                        tex,
                        tex_manager,
                    )?)
                });

                let south = element.face_textures.south.as_ref().and_then(|tex| {
                    Some(Self::absolute_atlas_uv(
                        tex,
                        tex_manager,
                    )?)
                });

                let west = element.face_textures.west.as_ref().and_then(|tex| {
                    Some(Self::absolute_atlas_uv(
                        tex,
                        tex_manager,
                    )?)
                });

                let up = element.face_textures.up.as_ref().and_then(|tex| {
                    Some(Self::absolute_atlas_uv(
                        tex,
                        tex_manager,
                    )?)
                });

                let down = element.face_textures.down.as_ref().and_then(|tex| {
                    Some(Self::absolute_atlas_uv(
                        tex,
                        tex_manager,
                    )?)
                });

                let a = [1.0 - element.from.0, element.from.1, element.from.2];
                let b = [1.0 - element.to.0, element.from.1, element.from.2];
                let c = [1.0 - element.to.0, element.to.1, element.from.2];
                let d = [1.0 - element.from.0, element.to.1, element.from.2];
                let e = [1.0 - element.from.0, element.from.1, element.to.2];
                let f = [1.0 - element.to.0, element.from.1, element.to.2];
                let g = [1.0 - element.to.0, element.to.1, element.to.2];
                let h = [1.0 - element.from.0, element.to.1, element.to.2];

                // let a = Vector3::from(a)

                #[rustfmt::skip]
                    let faces = BlockModelFaces {
                    south: south.map(|south| {[
                        MeshVertex { position: e, tex_coords: [south.1.0, south.1.1], normal: [0.0, 0.0, -1.0] },
                        MeshVertex { position: h, tex_coords: [south.1.0, south.0.1], normal: [0.0, 0.0, -1.0] },
                        MeshVertex { position: f, tex_coords: [south.0.0, south.1.1], normal: [0.0, 0.0, -1.0] },
                        MeshVertex { position: h, tex_coords: [south.1.0, south.0.1], normal: [0.0, 0.0, -1.0] },
                        MeshVertex { position: g, tex_coords: [south.0.0, south.0.1], normal: [0.0, 0.0, -1.0] },
                        MeshVertex { position: f, tex_coords: [south.0.0, south.1.1], normal: [0.0, 0.0, -1.0] },
                    ]}),
                    west: west.map(|west| {[
                        MeshVertex { position: g, tex_coords: [west.1.0, west.0.1], normal: [-1.0, 0.0, 0.0] },
                        MeshVertex { position: b, tex_coords: [west.0.0, west.1.1], normal: [-1.0, 0.0, 0.0] },
                        MeshVertex { position: f, tex_coords: [west.1.0, west.1.1], normal: [-1.0, 0.0, 0.0] },
                        MeshVertex { position: c, tex_coords: [west.0.0, west.0.1], normal: [-1.0, 0.0, 0.0] },
                        MeshVertex { position: b, tex_coords: [west.0.0, west.1.1], normal: [-1.0, 0.0, 0.0] },
                        MeshVertex { position: g, tex_coords: [west.1.0, west.0.1], normal: [-1.0, 0.0, 0.0] },
                    ]}),
                    north: north.map(|north| {[
                        MeshVertex { position: c, tex_coords: [north.1.0, north.0.1], normal: [0.0, 0.0, 1.0] },
                        MeshVertex { position: a, tex_coords: [north.0.0, north.1.1], normal: [0.0, 0.0, 1.0] },
                        MeshVertex { position: b, tex_coords: [north.1.0, north.1.1], normal: [0.0, 0.0, 1.0] },
                        MeshVertex { position: d, tex_coords: [north.0.0, north.0.1], normal: [0.0, 0.0, 1.0] },
                        MeshVertex { position: a, tex_coords: [north.0.0, north.1.1], normal: [0.0, 0.0, 1.0] },
                        MeshVertex { position: c, tex_coords: [north.1.0, north.0.1], normal: [0.0, 0.0, 1.0] },
                    ]}),
                    east: east.map(|east| {[
                        MeshVertex { position: e, tex_coords: [east.0.0, east.1.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: a, tex_coords: [east.1.0, east.1.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: d, tex_coords: [east.1.0, east.0.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: d, tex_coords: [east.1.0, east.0.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: h, tex_coords: [east.0.0, east.0.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: e, tex_coords: [east.0.0, east.1.1], normal: [1.0, 0.0, 0.0] },
                    ]}),
                    up: up.map(|up| {[
                        MeshVertex { position: g, tex_coords: [up.1.0, up.0.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: h, tex_coords: [up.0.0, up.0.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: d, tex_coords: [up.0.0, up.1.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: c, tex_coords: [up.1.0, up.1.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: g, tex_coords: [up.1.0, up.0.1], normal: [1.0, 0.0, 0.0] },
                        MeshVertex { position: d, tex_coords: [up.0.0, up.1.1], normal: [1.0, 0.0, 0.0] },
                    ]}),
                    down: down.map(|down| {[
                        MeshVertex { position: f, tex_coords: [down.0.0, down.1.1], normal: [0.0, -1.0, 0.0] },
                        MeshVertex { position: b, tex_coords: [down.0.0, down.0.1], normal: [0.0, -1.0, 0.0] },
                        MeshVertex { position: a, tex_coords: [down.1.0, down.0.1], normal: [0.0, -1.0, 0.0] },
                        MeshVertex { position: f, tex_coords: [down.0.0, down.1.1], normal: [0.0, -1.0, 0.0] },
                        MeshVertex { position: a, tex_coords: [down.1.0, down.0.1], normal: [0.0, -1.0, 0.0] },
                        MeshVertex { position: e, tex_coords: [down.1.0, down.1.1], normal: [0.0, -1.0, 0.0] },
                    ]}),
                };

                Some(faces)
            })
            .collect::<Option<Vec<BlockModelFaces>>>()?;

        //TODO
        let has_transparency = false;

        Some(Self {
            name: model.id.clone(),
            shape: if is_cube {
                CubeOrComplexMesh::Cube(results.pop().unwrap())
            } else {
                CubeOrComplexMesh::Custom(results)
            },
            transparent_or_complex: !is_cube || has_transparency
        })
    }
}