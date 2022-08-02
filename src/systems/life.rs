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
          4) something to do with storing a delta
          5) delete life if necessary
        */
        // this if statement is hard coded to 3 because we currently have 2 entities at startup (maybe the camera and the sun?)
        if total_entities < 3 {
            lazy_update.exec(move |world| {
                //TODO could this be fetch instead of fetch_mut?
                let life_to_create: Vec<Vec<Vec<Vec<bool>>>> = world.fetch_mut::<SessionResource>().life.clone();

                //set size of tetrahedrons
                let scale = Vector3::new(LIFE_FORM_SIZE, LIFE_FORM_SIZE, LIFE_FORM_SIZE);

                //loading tetra mesh
                let mesh_tetra = load_mesh(world, "mesh/hill-tetrahedron.obj");
                let mesh_tetra_mirror = load_mesh(world, "mesh/hill-tetrahedron-mirrored.obj");

                for (n, vec1) in life_to_create.iter().enumerate() {
                    for (x, vec2) in vec1.iter().enumerate() {
                        for (y, vec3) in vec2.iter().enumerate() {
                            // if bool_life is 1 then create a life form here.
                            //TODO Could be used to load a "save"
                            for (z, bool_life) in vec3.iter().enumerate() {
                                if !bool_life {
                                    //no life here
                                    continue;
                                }
                                
                                let mut transform_new_life = Transform::default();
                                transform_new_life.set_scale(scale);
                                
                                let color;
                                let mesh;
                                //rotate it if it's every third life form (todo: as this rotations only have 4 variants they could exist outside this loop!)
                                if n == 0 {//white DONE DO NOT MOVE OR ROTATE!
                                    color = load_colour_texture(world, 1.0, 1.0, 1.0, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        (x as f32) * LIFE_FORM_SIZE,
                                        (y as f32) * LIFE_FORM_SIZE,
                                        (z as f32) * LIFE_FORM_SIZE
                                    );
                                    
                                    transform_new_life.set_rotation_euler(std::f32::consts::PI*0.75, std::f32::consts::FRAC_PI_2, std::f32::consts::PI);

                                    mesh = mesh_tetra_mirror.clone();
                                } else if n == 1 {//red DONE DO NOT MOVE OR ROTATE!
                                    color = load_colour_texture(world, 0.6, 0.2, 0.2, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        (x as f32) * LIFE_FORM_SIZE,
                                        (y as f32) * LIFE_FORM_SIZE,
                                        (z as f32) * LIFE_FORM_SIZE
                                    );

                                    transform_new_life.set_rotation_euler(std::f32::consts::PI*1.75, 0.0, std::f32::consts::FRAC_PI_2);

                                    mesh = mesh_tetra.clone();
                                } else if n == 2 {//light blue DONE DO NOT MOVE OR ROTATE!
                                    color = load_colour_texture(world, 0.5, 0.5, 1.0, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        (x as f32-1.0) * LIFE_FORM_SIZE,
                                        (y as f32+1.0) * LIFE_FORM_SIZE,
                                        (z as f32-1.0) * LIFE_FORM_SIZE
                                    );
                                    
                                    transform_new_life.set_rotation_euler(std::f32::consts::PI*0.75, 0.0, 0.0);

                                    mesh = mesh_tetra_mirror.clone();
                                } else if n == 3 {//dark blue DONE DO NOT MOVE OR ROTATE!
                                    color = load_colour_texture(world, 0.2, 0.2, 0.7, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        (x as f32-1.0) * LIFE_FORM_SIZE,
                                        (y as f32+1.0) * LIFE_FORM_SIZE,
                                        (z as f32-1.0) as f32 * LIFE_FORM_SIZE
                                    );

                                    transform_new_life.set_rotation_euler(std::f32::consts::PI*0.75, 0.0, 0.0);

                                    mesh = mesh_tetra.clone();
                                } else if n == 4 {//light grey DONE DO NOT MOVE OR ROTATE!
                                    color = load_colour_texture(world, 0.5, 0.5, 0.5, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        x as f32 * LIFE_FORM_SIZE,
                                        y as f32 * LIFE_FORM_SIZE,
                                        z as f32 * LIFE_FORM_SIZE
                                    );
                                    
                                    transform_new_life.set_rotation_euler(std::f32::consts::PI/4.0, std::f32::consts::PI, 0.0);

                                    mesh = mesh_tetra_mirror.clone();
                                } else {//dark grey DONE DO NOT MOVE OR ROTATE!
                                    color = load_colour_texture(world, 0.2, 0.2, 0.2, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        x as f32 * LIFE_FORM_SIZE,
                                        y as f32 * LIFE_FORM_SIZE,
                                        z as f32 * LIFE_FORM_SIZE
                                    );

                                    transform_new_life.set_rotation_euler(std::f32::consts::PI/4.0, std::f32::consts::PI, 0.0);

                                    mesh = mesh_tetra.clone();
                                }

                                // give the life form a colour
                                let default_material = world.read_resource::<MaterialDefaults>().0.clone();
                                let colour_material = load_material_with_colour(world, color, default_material);

                                let xyz = transform_new_life.translation();

                                // make the life form exist!
                                world.create_entity()
                                    .named(format!("Life Form {},{},{} {}", xyz.x.to_string(),xyz.y.to_string(),xyz.z.to_string(),n.to_string()))
                                    .with(mesh)
                                    .with(LifeTag)
                                    .with(colour_material.clone())
                                    .with(transform_new_life.clone())
                                    .build();

                                //increment life counter
                                world.fetch_mut::<SessionResource>().counter += 1;
                            }
                        }
                    }
                }
            });
        }

        println!("Total of lifeforms: {}", total_entities.to_string());
    }
}