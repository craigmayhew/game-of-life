use amethyst::{
    assets::{AssetLoaderSystemData},
    core::{Named, Transform},
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

#[derive(Default)]
pub struct LifeSystem {}

impl<'s> System<'s> for LifeSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadStorage<'s, Named>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, lazy_update, names, mut transforms): Self::SystemData) {
        let total_entities:usize = (&entities).join().count();
        let mut entities_count:usize = 0;
        
        for (entity, name, transform) in (&entities, &names, &mut transforms).join() {
            entities_count += 1;
            
            if &name.name[..9] == "Life Form" && (entities_count < 2 || total_entities-entities_count<3) {

                let mut transform_new_life = transform.clone();
                transform_new_life.append_translation_xyz(0.0, 0.0, -crate::LIFE_FORM_SIZE);

                lazy_update.exec(move |world| {
                    //loading tetra mesh 
                    let mesh_tetra = world.exec(|loader: AssetLoaderSystemData<'_, Mesh> | {
                        loader.load("mesh/tetra.obj", ObjFormat, ())
                    });
                    
                    let red = world.exec(|loader: AssetLoaderSystemData<Texture> | {
                        loader.load_from_data(
                            load_from_linear_rgba(LinSrgba::new(1.0, 0.0, 0.0, 1.0)).into(),
                            (),
                        )
                    });

                    //load material
                    let default_material = world.read_resource::<MaterialDefaults>().0.clone();

                    let colour = world.exec(|loader: AssetLoaderSystemData<Material> | {
                        loader.load_from_data(
                            Material {
                                albedo: red,
                                ..default_material.clone()
                            },
                            (),
                        )
                    });

                    let translation = transform_new_life.translation();

                    world
                        .create_entity()
                        .named(format!("Life Form {},{},{}", translation.x.to_string(),translation.to_string(),translation.z.to_string()))
                        .with(mesh_tetra)
                        .with(colour)
                        .with(transform_new_life.clone())
                        .build();
                });
            }
        }

        println!("Total of lifeforms: {}", total_entities.to_string());
        println!("Number of lifeforms: {}", entities_count.to_string());
    }
}