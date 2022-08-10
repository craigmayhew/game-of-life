use bevy::{
    prelude::*, //default bevy
    window::PresentMode // needed to specify window info
};
use bevy_obj::*; // immport wavefront obj files

use crate::{
    LIFE_FORM_SIZE,
    SessionResource,
};

#[derive(Default)]
pub struct LifeTag;

#[derive(Default)]
pub struct LifeSystem {}

fn create_life(
    n:usize,
    x:usize,
    y:usize,
    z:usize,
    asset_server: &Res<AssetServer>,
    commands: &Commands,
) -> (bevy::prelude::Transform, [f32; 3]) {
    let mut transform_new_life: Transform;
    
    let color;
    let mesh: bevy::prelude::Handle<Mesh>;
    //rotate it if it's every third life form (todo: as this rotations only have 4 variants they could exist outside this loop!)
    if n == 0 {//white DONE DO NOT MOVE OR ROTATE!
        color = [1.0, 1.0, 1.0];

        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32) * LIFE_FORM_SIZE,
            (y as f32) * LIFE_FORM_SIZE,
            (z as f32) * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI*0.75);
        transform_new_life.rotate_y(std::f32::consts::FRAC_PI_2);
        transform_new_life.rotate_z(std::f32::consts::PI);
    } else if n == 1 {//red DONE DO NOT MOVE OR ROTATE!
        color = [0.6, 0.2, 1.0];

        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32) * LIFE_FORM_SIZE,
            (y as f32) * LIFE_FORM_SIZE,
            (z as f32) * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI*1.75);
        transform_new_life.rotate_y(0.0);
        transform_new_life.rotate_z(std::f32::consts::FRAC_PI_2);
    } else if n == 2 {//light blue DONE DO NOT MOVE OR ROTATE!
        color = [0.5, 0.5, 1.0];

        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32-1.0) * LIFE_FORM_SIZE,
            (y as f32+1.0) * LIFE_FORM_SIZE,
            (z as f32-1.0) * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI*0.75);
        transform_new_life.rotate_y(0.0);
        transform_new_life.rotate_z(0.0);
    } else if n == 3 {//dark blue DONE DO NOT MOVE OR ROTATE!
        color = [0.2, 0.2, 0.7];

        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32-1.0) * LIFE_FORM_SIZE,
            (y as f32+1.0) * LIFE_FORM_SIZE,
            (z as f32-1.0) as f32 * LIFE_FORM_SIZE
        );

        transform_new_life.rotate_x(std::f32::consts::PI*0.75);
        transform_new_life.rotate_y(0.0);
        transform_new_life.rotate_z(0.0);
    } else if n == 4 {//light grey DONE DO NOT MOVE OR ROTATE!
        color = [0.5, 0.5, 0.5];

        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            x as f32 * LIFE_FORM_SIZE,
            y as f32 * LIFE_FORM_SIZE,
            z as f32 * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI/4.0);
        transform_new_life.rotate_y(std::f32::consts::PI);
        transform_new_life.rotate_z(0.0);
    } else {//dark grey DONE DO NOT MOVE OR ROTATE!
        color = [0.2, 0.2, 0.2];

        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            x as f32 * LIFE_FORM_SIZE,
            y as f32 * LIFE_FORM_SIZE,
            z as f32 * LIFE_FORM_SIZE
        );

        transform_new_life.rotate_x(std::f32::consts::PI/4.0);
        transform_new_life.rotate_y(std::f32::consts::PI);
        transform_new_life.rotate_z(0.0);
    }

    //set size of tetrahedrons
    transform_new_life.with_scale(Vec3::new(LIFE_FORM_SIZE, LIFE_FORM_SIZE, LIFE_FORM_SIZE));

    (transform_new_life,color)
}

#[derive(Component)]
pub struct Life;

pub fn run(
    query: Query<&mut Life, With<Life>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut session: ResMut<SessionResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*todo:
        1) [x] read session resource
        2) determine which new life to create + create it
        3) removed the need for the init life function in main?
        4) something to do with storing a delta
        5) delete life if necessary
    */
    // this if statement is hard coded to 3 because we currently have 2 entities at startup (maybe the camera and the sun?)
    if session.generation == 1 {
        //TODO could this be fetch instead of fetch_mut?
        let life_to_create: Vec<Vec<Vec<Vec<bool>>>> = session.life.clone();
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

                        let transform_new_life: bevy::prelude::Transform;
                        let color: [f32; 3];
                        
                        (transform_new_life,color) = create_life(n, x, y, z, &asset_server, &commands);
                    
                        // make the life form exist!
                        commands.spawn_bundle(PbrBundle {
                            mesh: session.life_form_meshes[n%2].clone(),
                            material: session.life_form_materials[n].clone(),
                            transform: transform_new_life,
                            ..Default::default()
                        }).insert(Life);
                        
                        //increment life counter
                        session.counter += 1;
                    }
                }
            }
        }
    } else {
        //TODO could this be fetch instead of fetch_mut?
        let last_gen: Vec<Vec<Vec<Vec<bool>>>> = session.life.clone();
        let mut next_gen = vec![vec![vec![vec![false; crate::UNIVERSE_SIZE]; crate::UNIVERSE_SIZE]; crate::UNIVERSE_SIZE]; 6];
        /*
        dark blue touches light blue and white in same xyz and red and dark grey either side (need to check if thats x or z)
        light blue touches red and dark blue in the same xyz and white in the y above
        dark grey touches light grey and white in the same xyz and red in the y below
        light grey touches dark grey and red in the same xyz and light blue and white either side (need to check if thats x or z)
        red touches light grey and light blue in same xyz and the dark grey in the y above
        white touches dark blue and dark grey in the same xyz and light blue in the y below
        */
        
        for (n, vec1) in last_gen.iter().enumerate() {
            for (x, vec2) in vec1.iter().enumerate() {
                for (y, vec3) in vec2.iter().enumerate() {
                    for (z, bool_life) in vec3.iter().enumerate() {
                        let mut neighbours: usize = 0;
                        if n == 0 {//white touches dark blue and dark grey in the same xyz and light blue in the y below
                            if last_gen[3][x][y  ][z  ] {neighbours += 1;}
                            if last_gen[5][x][y  ][z  ] {neighbours += 1;}
                            //the y >0 checks if we are the edge of the univ
                            if y > 0 && last_gen[n][x][y-1][z  ] {neighbours += 1;}
                            if z > 0 && last_gen[4][x][y  ][z-1] {neighbours += 1;}
                        }
                        
                        if !bool_life {//if not alive in last gen
                            //if neighbours = 3 then become alive
                            if neighbours == 3 {
                                next_gen[n][x][y][z] = true;

                                let transform_new_life: bevy::prelude::Transform;
                                let color: [f32; 3];
                                
                                (transform_new_life,color) = create_life(n, x, y, z, &asset_server, &commands);
                            
                                // make the life form exist!
                                commands.spawn_bundle(PbrBundle {
                                    mesh: session.life_form_meshes[n%2].clone(),
                                    material: session.life_form_materials[n].clone(),
                                    transform: transform_new_life,
                                    ..Default::default()
                                }).insert(Life);
                                
                                //increment life counter
                                session.counter += 1;
                            } else {
                                next_gen[n][x][y][z] = false;
                            }
                        } else {//if alive in last gen
                            if neighbours == 4 || neighbours == 0 {
                                next_gen[n][x][y][z] = false;
                            } else {
                                next_gen[n][x][y][z] = true;
                                
                                let transform_new_life: bevy::prelude::Transform;
                                let color: [f32; 3];
                                
                                (transform_new_life,color) = create_life(n, x, y, z, &asset_server, &commands);

                                // make the life form exist!
                                commands.spawn_bundle(PbrBundle {
                                    mesh: session.life_form_meshes[n%2].clone(),
                                    material: session.life_form_materials[n].clone(),
                                    transform: transform_new_life,
                                    ..Default::default()
                                }).insert(Life);
                                
                                //increment life counter
                                session.counter += 1;
                            }
                        }
                    }
                }
            }
        }

        session.life = next_gen;
        session.generation += 1;
        println!("Total of in theory: {}", session.counter.to_string());
    }

    //println!("Total of lifeform entitys: {}", total_entities.to_string());
}