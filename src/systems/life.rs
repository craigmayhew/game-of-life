use amethyst::{
    assets::{AssetLoaderSystemData, Handle},
    core::{Transform},
    core::math::Vector3,
    ecs::*,
    prelude::WithNamed,//needed to allow the use of world.create_entity().named("something")
    renderer::{
        formats::mesh::ObjFormat,
        Texture,
        Material,
        MaterialDefaults,
        Mesh,
        palette::rgb::LinSrgba,
        rendy::{
            texture::palette::load_from_linear_rgba,
        },
    },
};

use crate::{
    LIFE_FORM_SIZE,
    SessionResource,
};

pub fn load_mesh(world: &mut World, mesh_obj_filename: &str) -> Handle<Mesh> {
    world.exec(|loader: AssetLoaderSystemData<'_, Mesh> | {
        loader.load(mesh_obj_filename, ObjFormat, ())
    })
}

pub fn load_colour_texture(world: &mut World, r: f32, g: f32, b: f32, a: f32) -> Handle<Texture>{
    world.exec(|loader: AssetLoaderSystemData<Texture> | {
        loader.load_from_data(
            load_from_linear_rgba(LinSrgba::new(r, g, b, a)).into(),
            (),
        )
    })
}

pub fn load_material_with_colour(world: &mut World, colour: Handle<Texture>, default_material: Material) -> Handle<Material>{
    world.exec(|loader: AssetLoaderSystemData<Material> | {
        loader.load_from_data(
            Material {
                albedo: colour,
                ..default_material
            },
            (),
        )
    })
}

#[derive(Default)]
pub struct LifeTag;

impl Component for LifeTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct LifeSystem {}

impl<'s> System<'s> for LifeSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadStorage<'s, LifeTag>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, lazy_update, life_tag, mut transforms): Self::SystemData) {
        let total_entities:usize = (&entities).join().count();

        /*todo:
          1) [x] read session resource
          2) determine which new life to create + create it
          3) removed the need for the init life function in main?
          4) something to do wit hstoring a delta
          5) delete life if necessary
        */

        lazy_update.exec(move |world| {
            let life_to_create: Vec<Vec<Vec<usize>>>;
            {
                let fetched = world.try_fetch_mut::<SessionResource>();
                if let Some(fetched_something) = fetched {
                    life_to_create = fetched_something.life.clone();
                } else{
                    //todo: something is horribly wrong if this line runs
                    //      because we setup SessionResource in main
                    //      must be a better way of dealing with this
                    life_to_create = vec![vec![vec![0; 1]; 1]; 1];
                }
            }

            let mut transform_new_life = Transform::default();
            //set size of tetrahedrons
            let scale = Vector3::new(LIFE_FORM_SIZE, LIFE_FORM_SIZE, LIFE_FORM_SIZE);
            transform_new_life.set_scale(scale);

            //loading tetra mesh
            
            let red = load_colour_texture(world, 1.0, 0.0, 1.0, 1.0);

            //load material
            let default_material = world.read_resource::<MaterialDefaults>().0.clone();

            let colour_material = load_material_with_colour(world, red, default_material);

            for (x, vec2) in life_to_create.iter().enumerate() {
                for (y, vec3) in vec2.iter().enumerate() {
                    for (z, _bool_life) in vec3.iter().enumerate() {
                        transform_new_life.set_translation_xyz(
                            x as f32 * LIFE_FORM_SIZE,
                            y as f32 * LIFE_FORM_SIZE,
                            z as f32 * LIFE_FORM_SIZE
                        );
                        let translation = transform_new_life.translation();

                        world
                            .create_entity()
                            .named(format!("Life Form {},{},{}", translation.x.to_string(),translation.to_string(),translation.z.to_string()))
                            .with(mesh_tetra.clone())
                            .with(LifeTag)
                            .with(colour_material.clone())
                            .with(transform_new_life.clone())
                            .build();
            let mesh_tetra = load_mesh(world, "mesh/sommerville-hill-tetrahedron.obj");
                    }
                }
            }
        });

        println!("Total of lifeforms: {}", total_entities.to_string());
    }
}