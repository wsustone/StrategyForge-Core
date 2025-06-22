use bevy::{
    prelude::*,
    sprite::{
        Mesh2dHandle,
        MaterialMesh2dBundle,
    },
    render::{
        mesh::{Mesh, Indices},
        render_asset::RenderAssetUsages,
    },
    math::{Vec2, Vec3},
};

// Helper function to create a quad mesh
fn create_quad_mesh(width: f32, height: f32) -> Mesh {
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    
    let positions = vec![
        // bottom left
        Vec3::new(-half_width, -half_height, 0.0),
        // bottom right
        Vec3::new(half_width, -half_height, 0.0),
        // top right
        Vec3::new(half_width, half_height, 0.0),
        // top left
        Vec3::new(-half_width, half_height, 0.0),
    ];
    
    let normals = vec![
        Vec3::Z,
        Vec3::Z,
        Vec3::Z,
        Vec3::Z,
    ];
    
    let uvs = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ];
    
    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);
    
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);
    
    mesh
}
use super::WorldConfig;

/// Component for the map border
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MapBorder;

/// System to spawn the map border
pub fn spawn_map_border(
    mut commands: Commands,
    world_config: Res<WorldConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Border color (semi-transparent black)
    let border_color = Color::srgba(0.1, 0.1, 0.1, 0.8);
    let border_material = materials.add(border_color);
    
    // Get border dimensions
    let border_width = world_config.border_width;
    
    // Create border mesh (a large quad with a hole in the middle)
    let border_mesh = meshes.add(
        create_quad_mesh(
            world_config.width_meters + border_width * 2.0,
            world_config.height_meters + border_width * 2.0,
        )
    );
    
    // Get playable area dimensions
    let playable_width = world_config.playable_width();
    let playable_height = world_config.playable_height();
    
    // Spawn the outer border (full size)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(border_mesh),
            material: border_material.clone(),
            transform: Transform::from_xyz(
                world_config.width_meters / 2.0,
                world_config.height_meters / 2.0,
                0.0,
            ),
            ..default()
        },
        MapBorder,
    ));
    
    // Create inner quad (hole in the middle)
    let inner_mesh = meshes.add(
        create_quad_mesh(playable_width, playable_height)
    );
    
    // Spawn the inner quad (transparent)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(inner_mesh),
            material: materials.add(Color::NONE),
            transform: Transform::from_xyz(
                world_config.width_meters / 2.0,
                world_config.height_meters / 2.0,
                0.1, // Slightly above the border
            ),
            ..default()
        },
        MapBorder,
    ));
}

/// Plugin for the map border
pub struct MapBorderPlugin;

impl Plugin for MapBorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map_border);
    }
}
