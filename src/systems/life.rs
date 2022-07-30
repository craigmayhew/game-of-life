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
        // this if statement is hard coded to 3 because we currently have 2 entities at startup (maybe the camera and the sun?)
        if total_entities < 3 {
            lazy_update.exec(move |world| {
                let life_to_create: Vec<Vec<Vec<Vec<usize>>>> = world.fetch_mut::<SessionResource>().life.clone();

                //set size of tetrahedrons
                let scale = Vector3::new(LIFE_FORM_SIZE, LIFE_FORM_SIZE, LIFE_FORM_SIZE);

                //loading tetra mesh
                let mesh_tetra = load_mesh(world, "mesh/sommerville-hill-tetrahedron.obj");

                for (n, vec1) in life_to_create.iter().enumerate() {
                    let mut red = 0.0;
                    for (x, vec2) in vec1.iter().enumerate() {
                        let mut green = 0.0;
                        red+=1.0;
                        for (y, vec3) in vec2.iter().enumerate() {
                            let mut blue = 0.0;
                            green+=1.0;
                            for (z, _bool_life) in vec3.iter().enumerate() {
                                blue+=1.0;

                                let mut transform_new_life = Transform::default();
                                transform_new_life.set_scale(scale);
                                
                                let color;
                                //rotate it if it's every third life form (todo: as this rotations only have 4 variants they could exist outside this loop!)
                                if n == 1 {//red
                                    color = load_colour_texture(world, 0.5, 0.0, 0.0, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        (x as f32 + std::f32::consts::SQRT_2) as f32 * LIFE_FORM_SIZE,
                                        y as f32 * std::f32::consts::SQRT_2 * LIFE_FORM_SIZE,
                                        (z as f32 + std::f32::consts::SQRT_2) as f32 * LIFE_FORM_SIZE
                                    ); 
                                    
                                    transform_new_life.set_rotation_x_axis(std::f32::consts::FRAC_PI_2);
                                    transform_new_life.set_rotation_y_axis(3.0*std::f32::consts::FRAC_PI_2);
                                    transform_new_life.set_rotation_z_axis(std::f32::consts::PI);
                                    
                                } else if n == 2 { //terqouise to white DONE
                                    color = load_colour_texture(world, (1.0/red), 1.0, (1.0/blue) as f32, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        (x as f32 + 1.0) as f32 * LIFE_FORM_SIZE,
                                        y as f32 * std::f32::consts::SQRT_2 * LIFE_FORM_SIZE,
                                        (z as f32 + std::f32::consts::SQRT_2) as f32 * LIFE_FORM_SIZE
                                    ); 
                                    
                                    transform_new_life.set_rotation_x_axis(std::f32::consts::FRAC_PI_2);
                                    transform_new_life.set_rotation_y_axis(std::f32::consts::PI);
                                } else if n == 3 {//light grey DONE
                                    color = load_colour_texture(world, 0.5, 0.5, 0.5 as f32, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        (x as f32 + 1.0) * LIFE_FORM_SIZE,
                                        ((y as f32 * std::f32::consts::SQRT_2) + 1.0*std::f32::consts::SQRT_2) * LIFE_FORM_SIZE,
                                        (z as f32) * LIFE_FORM_SIZE
                                    );
                                    
                                    transform_new_life.set_rotation_x_axis(std::f32::consts::FRAC_PI_2);
                                    transform_new_life.set_rotation_y_axis(std::f32::consts::FRAC_PI_2);
                                    transform_new_life.set_rotation_z_axis(std::f32::consts::PI);
                                } else {//dark grey DONE DO NOT MOVE OR ROTATE!
                                    color = load_colour_texture(world, 0.2, 0.2, 0.2, 1.0);

                                    // position the life form in 3d space
                                    transform_new_life.set_translation_xyz(
                                        x as f32 * LIFE_FORM_SIZE,
                                        y as f32 * std::f32::consts::SQRT_2 * LIFE_FORM_SIZE,
                                        z as f32 * LIFE_FORM_SIZE
                                    );

                                    transform_new_life.set_rotation_x_axis(0.0);
                                    transform_new_life.set_rotation_y_axis(0.0);
                                }

                                let translation = transform_new_life.translation();

                                // give the life form a colour
                                
                                let default_material = world.read_resource::<MaterialDefaults>().0.clone();
                                let colour_material = load_material_with_colour(world, color, default_material);

                                // make the life form exist!
                                world.create_entity()
                                    .named(format!("Life Form {},{},{}", translation.x.to_string(),translation.y.to_string(),translation.z.to_string()))
                                    .with(mesh_tetra.clone())
                                    .with(LifeTag)
                                    .with(colour_material.clone())
                                    .with(transform_new_life.clone())
                                    .build();
                            }
                        }
                    }
                }
            });
        }

        println!("Total of lifeforms: {}", total_entities.to_string());
    }
}