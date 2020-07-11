use amethyst::{
    assets::{AssetLoaderSystemData, Handle},
    core::{Transform},
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
        let mut entities_count:usize = 0;
        
        for (_life, transform) in (&life_tag, &mut transforms).join() {
            entities_count += 1;
            
            if entities_count < 2 || total_entities-entities_count<3 {

                let mut transform_new_life = transform.clone();
                transform_new_life.append_translation_xyz(0.0, 0.0, -crate::LIFE_FORM_SIZE);

                lazy_update.exec(move |world| {
                    //loading tetra mesh 
                    let mesh_tetra = load_mesh(world, "mesh/tetra.obj");
                    
                    let red = load_colour_texture(world, 1.0, 0.0, 0.0, 1.0);

                    //load material
                    let default_material = world.read_resource::<MaterialDefaults>().0.clone();

                    let colour_material = load_material_with_colour(world, red, default_material);

                    let translation = transform_new_life.translation();

                    world
                        .create_entity()
                        .named(format!("Life Form {},{},{}", translation.x.to_string(),translation.to_string(),translation.z.to_string()))
                        .with(mesh_tetra)
                        .with(LifeTag)
                        .with(colour_material)
                        .with(transform_new_life.clone())
                        .build();
                });
            }
        }

        println!("Total of lifeforms: {}", total_entities.to_string());
        println!("Number of lifeforms: {}", entities_count.to_string());
    }
}