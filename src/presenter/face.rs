use bevy::asset::{embedded_asset, load_embedded_asset, RenderAssetUsages};
use bevy::mesh::Indices;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

use super::{HeadCamera, HeadMaterialSet};

pub fn register_assets(app: &mut App) {
    embedded_asset!(app, "../../assets/presenter/cgi-closed.png");
    embedded_asset!(app, "../../assets/presenter/cgi-slight.png");
    embedded_asset!(app, "../../assets/presenter/cgi-medium.png");
    embedded_asset!(app, "../../assets/presenter/cgi-wide.png");
}

fn relief_depth(u: f32, v: f32) -> f32 {
    let x = (u - 0.5) * 2.0;
    let y = (0.57 - v) * 2.0;

    // One continuous depth field for the complete portrait. The broad head
    // volume, projected nose, jaw, shoulders and chest all share the same UVs.
    let head_mask = (1.0 - (x / 0.72).powi(2) - ((y - 0.18) / 0.86).powi(2)).max(0.0);
    let head = head_mask.sqrt() * 0.72;
    let nose = (-((x / 0.13).powi(2) + ((y - 0.13) / 0.34).powi(2)) * 3.2).exp() * 0.38;
    let brow = (-((x / 0.55).powi(2) + ((y - 0.43) / 0.15).powi(2)) * 2.5).exp() * 0.10;
    let chin = (-((x / 0.34).powi(2) + ((y + 0.48) / 0.19).powi(2)) * 2.8).exp() * 0.13;
    let bust_mask = ((v - 0.66) / 0.34).clamp(0.0, 1.0);
    let shoulders = bust_mask * (1.0 - x.abs() * 0.20) * 0.27;
    head + nose + brow + chin + shoulders
}

fn portrait_relief_mesh() -> Mesh {
    let columns = 52;
    let rows = 72;
    let width = 5.15;
    let height = 7.75;
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for row in 0..=rows {
        let v = row as f32 / rows as f32;
        for column in 0..=columns {
            let u = column as f32 / columns as f32;
            let x = (u - 0.5) * width;
            let y = (0.5 - v) * height;
            positions.push([x, y, relief_depth(u, v)]);
            uvs.push([u, v]);
            normals.push([0.0, 0.0, 1.0]);
        }
    }

    for row in 0..rows {
        for column in 0..columns {
            let stride = columns + 1;
            let a = row * stride + column;
            let b = a + 1;
            let c = a + stride;
            let d = c + 1;
            indices
                .extend_from_slice(&[a as u32, c as u32, b as u32, b as u32, c as u32, d as u32]);
        }
    }

    // Derive smooth normals from the depth field so lighting follows the facial relief.
    let epsilon = 1.0 / columns as f32;
    for row in 0..=rows {
        let v = row as f32 / rows as f32;
        for column in 0..=columns {
            let u = column as f32 / columns as f32;
            let dz_du = (relief_depth((u + epsilon).min(1.0), v)
                - relief_depth((u - epsilon).max(0.0), v))
                / (2.0 * epsilon * width);
            let dz_dv = (relief_depth(u, (v + epsilon).min(1.0))
                - relief_depth(u, (v - epsilon).max(0.0)))
                / (2.0 * epsilon * height);
            normals[row * (columns + 1) + column] =
                Vec3::new(-dz_du, dz_dv, 1.0).normalize().to_array();
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn portrait_material(texture: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(texture),
        alpha_mode: AlphaMode::Mask(0.20),
        unlit: true,
        cull_mode: None,
        ..default()
    }
}

pub fn spawn_presenter(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let images = [
        load_embedded_asset!(&*assets, "../../assets/presenter/cgi-closed.png"),
        load_embedded_asset!(&*assets, "../../assets/presenter/cgi-slight.png"),
        load_embedded_asset!(&*assets, "../../assets/presenter/cgi-medium.png"),
        load_embedded_asset!(&*assets, "../../assets/presenter/cgi-wide.png"),
    ];
    let material_handles = images.map(|image| materials.add(portrait_material(image)));

    // The face, glasses, shirt, suit and tie are now one uninterrupted mapped object.
    commands.spawn((
        HeadMaterialSet {
            materials: material_handles.clone(),
        },
        Mesh3d(meshes.add(portrait_relief_mesh())),
        MeshMaterial3d(material_handles[0].clone()),
        Transform::from_xyz(0.0, -0.15, 0.0),
    ));

    commands.spawn((
        HeadCamera,
        Camera3d::default(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 0.61,
            ..default()
        }),
        Transform::from_xyz(0.0, -0.15, 12.2).looking_at(Vec3::new(0.0, -0.15, 0.2), Vec3::Y),
    ));
}
