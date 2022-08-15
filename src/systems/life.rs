use bevy::{
    prelude::*, //default bevy
};

use crate::{
    AppState,
    SessionResource,
};

pub const LIFE_FORM_SIZE: f32 = 150.0;

#[derive(Default)]
pub struct LifeTag;

#[derive(Default)]
pub struct LifeSystem {}

fn create_life_xyz(
    n:usize,
    x:usize,
    y:usize,
    z:usize,
) -> bevy::prelude::Transform {
    let mut transform_new_life: Transform;
    //rotate it if it's every third life form (todo: as this rotations only have 4 variants they could exist outside this loop!)
    //TODO consider if n == 0 and n == 1 could/should actually be identical blocks
    if n == 0 {//white
        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32) * LIFE_FORM_SIZE,
            (y as f32) * LIFE_FORM_SIZE,
            (z as f32) * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI*0.75);
        transform_new_life.rotate_y(std::f32::consts::FRAC_PI_2);
        transform_new_life.rotate_z(std::f32::consts::PI);
    } else if n == 1 {//red
        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32) * LIFE_FORM_SIZE,
            (y as f32) * LIFE_FORM_SIZE,
            (z as f32) * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI*1.75);
        transform_new_life.rotate_y(0.0);
        transform_new_life.rotate_z(std::f32::consts::FRAC_PI_2);
    } else if n == 2 || n == 3 {//light blue and dark blue
        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            (x as f32-1.0) * LIFE_FORM_SIZE,
            (y as f32+1.0) * LIFE_FORM_SIZE,
            (z as f32-1.0) * LIFE_FORM_SIZE
        );
        
        transform_new_life.rotate_x(std::f32::consts::PI*0.75);
        transform_new_life.rotate_y(0.0);
        transform_new_life.rotate_z(0.0);
    } else {//light grey and dark grey
        // position the life form in 3d space
        transform_new_life = Transform::from_xyz(
            x as f32 * LIFE_FORM_SIZE,
            y as f32 * LIFE_FORM_SIZE,
            z as f32 * LIFE_FORM_SIZE
        );

        transform_new_life.rotate_x(std::f32::consts::FRAC_PI_4);
        transform_new_life.rotate_y(std::f32::consts::PI);
        transform_new_life.rotate_z(0.0);
    }

    //set size of tetrahedrons and return
    transform_new_life.with_scale(Vec3::new(LIFE_FORM_SIZE, LIFE_FORM_SIZE, LIFE_FORM_SIZE))
}

#[derive(Component)]
pub struct Life;

// we use an enum to get around the fact that even entity id 0 is valid
#[derive(Clone)]
#[derive(Copy)]
pub enum LifeDataContainer {
    Alive(Entity),
    Dead(bool),
}

pub fn run(
    mut commands: Commands,
    mut session: ResMut<SessionResource>,
    state: ResMut<State<AppState>>,
) {
    match state.current() {
        AppState::InGame => {},
        _ => {return},
    }

    // first generation, generate random life
    if session.generation == 1 {
        let life_to_create: Vec<Vec<Vec<Vec<LifeDataContainer>>>> = session.life.clone();
        for (n, vec1) in life_to_create.iter().enumerate() {
            for (x, vec2) in vec1.iter().enumerate() {
                for (y, vec3) in vec2.iter().enumerate() {
                    for (z, _empty_entity_id) in vec3.iter().enumerate() {
                        //randomly generate initial life in the universe
                        if rand::random::<bool>() {
                            //create no life here
                            continue;
                        }

                        let transform_new_life: bevy::prelude::Transform = create_life_xyz(n, x, y, z);
                    
                        // make the life form exist!
                        session.life[n][x][y][z] = LifeDataContainer::Alive(commands.spawn_bundle(PbrBundle {
                            mesh: session.life_form_meshes[n%2].clone(),
                            material: session.life_form_materials[n].clone(),
                            transform: transform_new_life,
                            ..Default::default()
                        }).insert(Life).id());
                        
                        //increment life counter
                        session.counter += 1;
                    }
                }
            }
        }
        session.generation = 2;
    } else if session.counter > 1 { // while there is life
        let last_gen: Vec<Vec<Vec<Vec<LifeDataContainer>>>> = session.life.clone();
        let mut next_gen = vec![vec![vec![vec![LifeDataContainer::Dead(true); crate::UNIVERSE_SIZE]; crate::UNIVERSE_SIZE]; crate::UNIVERSE_SIZE]; 6];
        /*
        white touches dark blue and dark grey in the same xyz and light blue in the y below
        red touches light grey and light blue in same xyz and the dark grey in the y above
        light blue touches red and dark blue in the same xyz and white in the y above
        dark blue touches light blue and white in same xyz and red and dark grey either side (need to check if thats x or z)
        light grey touches dark grey and red in the same xyz and light blue and white either side (need to check if thats x or z)
        dark grey touches light grey and white in the same xyz and red in the y below
        */
        
        for (n, vec1) in last_gen.iter().enumerate() {
            for (x, vec2) in vec1.iter().enumerate() {
                for (y, vec3) in vec2.iter().enumerate() {
                    for (z, entity_life) in vec3.iter().enumerate() {
                        let mut neighbours: usize = 0;
                        /*
                          // the x>0 y>0 and z>0 are for checking if we are not needing to wrap the universe
                          // the x==0 y==0 z==0 checks are if we are wrapping the universe to find out neighbour
                        */
                        if n == 0 {
                            //CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            // white touches dark blue and dark grey in the same xyz and light blue in the y below
                            if y > 0 && let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z] {neighbours += 1} // touches light blue below
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[2][x][crate::UNIVERSE_SIZE-1][z] {neighbours += 1} // touches light blue below (on the other side of the universe)
                            if z > 0 && let LifeDataContainer::Alive(_) = last_gen[4][x][y][z-1] {neighbours += 1} // touches light grey
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[4][x][y][crate::UNIVERSE_SIZE-1] {neighbours += 1} // touches light grey (on the other side of the universe)
                            
                            // 6 EDGE CHECKS
                            //touches 1 and 2 in z-1 x-1
                            if x > 0 {
                                if z > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x-1][y][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][z-1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x-1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                }
                            } else if z > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[1][crate::UNIVERSE_SIZE-1][y][z-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[2][crate::UNIVERSE_SIZE-1][y][z-1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[1][crate::UNIVERSE_SIZE-1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[2][crate::UNIVERSE_SIZE-1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                            }
                            //touches 1 in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                            if z > 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y][z-1] {neighbours += 1;}
                            //touches 1 in x+1 y-1
                            if crate::UNIVERSE_SIZE > x+1 {
                                if y > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x+1][y-1][z] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[1][x+1][y][crate::UNIVERSE_SIZE-1] {
                                    neighbours += 1;
                                }
                            } else if y > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[1][0][y-1][z] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[1][0][crate::UNIVERSE_SIZE-1][z] {
                                neighbours += 1;
                            }
                            //touches 3 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[3][crate::UNIVERSE_SIZE-1][y][z] {neighbours += 1;}
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x-1][y][z] {neighbours += 1;}
                            //touches 1 in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][crate::UNIVERSE_SIZE-1][z] {neighbours += 1;}
                            if y > 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y-1][z] {neighbours += 1;}
                            //rememebr z goes down as you move forward
                        } else if n == 1 {// red touches light grey and light blue in same xyz and the dark grey in the y above and dark blue in z-1
                            //CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y>0 and z>0 checks if we are the edge of the univ
                            if crate::UNIVERSE_SIZE > y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z  ] {neighbours += 1;} // touches dark grey above
                            if crate::UNIVERSE_SIZE == y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][0][z  ] {neighbours += 1;} // touches dark grey above (on the other side of the universe)
                            if z > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x][y  ][z-1] {neighbours += 1;} // touches dark blue in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[3][x][y  ][crate::UNIVERSE_SIZE-1] {neighbours += 1;} // touches dark blue in z-1 (on the other side of the universe)
                            // 6 EDGE CHECKS
                            //touches 0 in y+1
                            if crate::UNIVERSE_SIZE >  y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][z] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][0  ][z] {neighbours += 1;}
                            //touches 2 in z+1
                            if crate::UNIVERSE_SIZE >  z+1 && let LifeDataContainer::Alive(_) = last_gen[2][x][y][z+1] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == z+1 && let LifeDataContainer::Alive(_) = last_gen[2][x][y][0  ] {neighbours += 1;}
                            //touches 0 in z+1
                            if crate::UNIVERSE_SIZE >  z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y][z+1] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y][0  ] {neighbours += 1;}
                            //touches 0 in y+1 z+1
                            if crate::UNIVERSE_SIZE > z+1 {
                                if crate::UNIVERSE_SIZE > y+1 {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][z+1] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[0][x][0][z+1] {
                                    neighbours += 1;
                                }
                            } else if crate::UNIVERSE_SIZE > y+1 {
                                if let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][0] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[0][x][0][0] {
                                neighbours += 1;
                            }
                            //touches 2 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[2][crate::UNIVERSE_SIZE-1][y][z] {neighbours += 1;}
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][z] {neighbours += 1;}
                            //touches 3 in x-1 y+1
                            if crate::UNIVERSE_SIZE > y+1 {
                                if x > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x-1][y+1][z] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[3][crate::UNIVERSE_SIZE-1][y+1][z] {
                                    neighbours += 1;
                                }
                            } else if x > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[3][x-1][0][z] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[3][crate::UNIVERSE_SIZE-1][0][z] {
                                neighbours += 1;
                            }
                        } else if n == 2 {// light blue touches red and dark blue in the same xyz and white in the y above
                            // CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y >0 checks if we are the edge of the univ
                            if crate::UNIVERSE_SIZE > y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][z  ] {neighbours += 1;} // touches white above
                            if crate::UNIVERSE_SIZE == y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][0][z  ] {neighbours += 1;} // touches white above (on the other side of the universe)
                            if crate::UNIVERSE_SIZE > x+1 && let LifeDataContainer::Alive(_) = last_gen[4][x+1][y  ][z] {neighbours += 1;} // touches in x+1
                            if crate::UNIVERSE_SIZE == x+1 && let LifeDataContainer::Alive(_) = last_gen[4][0][y  ][z] {neighbours += 1;} // touches in x+1 (on the other side of the universe)
                            // 9 EDGE CHECKS
                            //touches 5 in x+1
                            if crate::UNIVERSE_SIZE >  x+1 && let LifeDataContainer::Alive(_) = last_gen[5][x+1][y][z] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == x+1 && let LifeDataContainer::Alive(_) = last_gen[5][0  ][y][z] {neighbours += 1;}
                            //touches 5 in y+1
                            if crate::UNIVERSE_SIZE >  y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][0  ][z] {neighbours += 1;}
                            //touches 3 in z+1
                            if crate::UNIVERSE_SIZE >  z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][z+1] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][0  ] {neighbours += 1;}
                            //touches 1 in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                            if z > 0  && let LifeDataContainer::Alive(_) = last_gen[1][x][y][z-1] {neighbours += 1;}
                            //touches 4 and 5 y+1 z-1
                            if crate::UNIVERSE_SIZE > y+1 {
                                if z > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x][y+1][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z-1] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[3][crate::UNIVERSE_SIZE-1][y+1][z] {
                                    neighbours += 1;
                                }
                            } else if z > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[4][x][0][z-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][x][0][z-1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[4][x][0][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][x][0][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                            }
                            //touches 0 and 5 in x+1 z+1
                            if crate::UNIVERSE_SIZE > x+1 {
                                if crate::UNIVERSE_SIZE > z+1 {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x+1][y][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x+1][y][z+1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x+1][y][0] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x+1][y][0] {neighbours += 1;}
                                }
                            } else if crate::UNIVERSE_SIZE > z+1 {
                                if let LifeDataContainer::Alive(_) = last_gen[0][0][y][z+1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][0][y][z+1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[0][0][y][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][0][y][0] {neighbours += 1;}
                            }
                            //touches 5 x+1 y+1
                            if crate::UNIVERSE_SIZE > x+1 {
                                if crate::UNIVERSE_SIZE > y+1 {
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x+1][y+1][z] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[5][x+1][0][z] {
                                    neighbours += 1;
                                }
                            } else if crate::UNIVERSE_SIZE > y+1 {
                                if let LifeDataContainer::Alive(_) = last_gen[5][0][y+1][z] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[5][0][0][z] {
                                neighbours += 1;
                            }
                        } else if n == 3 {// dark blue touches light blue and white in same xyz and red and dark grey either side (need to check if thats x or z)
                            // CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y >0 checks if we are the edge of the univ
                            if crate::UNIVERSE_SIZE > z+1 && let LifeDataContainer::Alive(_) = last_gen[1][x  ][y][z+1] {neighbours += 1;} // touches red
                            if crate::UNIVERSE_SIZE == z+1 && let LifeDataContainer::Alive(_) = last_gen[1][x  ][y][0] {neighbours += 1;} // touches red (on the other side of the universe)
                            if crate::UNIVERSE_SIZE > x+1 && let LifeDataContainer::Alive(_) = last_gen[5][x+1][y  ][z] {neighbours += 1;} // touches dark grey
                            if crate::UNIVERSE_SIZE == x+1 && let LifeDataContainer::Alive(_) = last_gen[5][0][y  ][z] {neighbours += 1;} // touches dark grey (on the other side of the universe)
                            // 9 EDGE CHECKS
                            //touches 4 in x+1
                            if crate::UNIVERSE_SIZE >  x+1 && let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][z] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == x+1 && let LifeDataContainer::Alive(_) = last_gen[4][0  ][y][z] {neighbours += 1;}
                            //touches 0 in y+1
                            if crate::UNIVERSE_SIZE >  y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y+1][z] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == y+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][0  ][z] {neighbours += 1;}
                            //touches 2 in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[2][x][crate::UNIVERSE_SIZE-1][z] {neighbours += 1;}
                            if y > 0  && let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z] {neighbours += 1;}
                            // touches 1 and 4 in x+1 y-1
                            if crate::UNIVERSE_SIZE > x+1 {
                                if y > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x+1][y-1][z] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y-1][z] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x+1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                }
                            } else if y > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[1][0][y-1][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[4][0][y-1][z] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[1][0][crate::UNIVERSE_SIZE-1][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[4][0][crate::UNIVERSE_SIZE-1][z] {neighbours += 1;}
                            }
                            //touches 4 in z-1
                            if z == 0 && let LifeDataContainer::Alive(_) = last_gen[4][x][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                            if z > 0  && let LifeDataContainer::Alive(_) = last_gen[4][x][y][z-1] {neighbours += 1;}
                            //touches 4 in x+1 z-1
                            if crate::UNIVERSE_SIZE > x+1 {
                                if z > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][z-1] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[4][x+1][y][crate::UNIVERSE_SIZE-1] {
                                    neighbours += 1;
                                }
                            } else if z > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[4][0][y][z-1] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[4][0][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                            //touches 4 and 5 y+1 z-1
                            if crate::UNIVERSE_SIZE > y+1 {
                                if z > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x][y+1][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z-1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[4][x][y+1][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                }
                            } else if z > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[4][x][0][z-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][x][0][z-1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[4][x][0][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[5][x][0][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                            }
                        } else if n == 4 {// light grey touches dark grey and red in the same xyz and light blue and white either side (need to check if thats x or z)
                            // CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[5][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y >0 checks if we are the edge of the univ
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x-1][y  ][z  ] {neighbours += 1;} // touches dark blue
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[3][crate::UNIVERSE_SIZE-1][y  ][z  ] {neighbours += 1;} // touches dark blue (on the other side of the universe)
                            if crate::UNIVERSE_SIZE > z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x  ][y  ][z+1] {neighbours += 1;} // touches white
                            if crate::UNIVERSE_SIZE == z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x  ][y  ][0] {neighbours += 1;} // touches white (on the other side of the universe)
                            // 8 EDGE CHECKS
                            // touches 3 in z+1
                            if crate::UNIVERSE_SIZE >  z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][z+1] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == z+1 && let LifeDataContainer::Alive(_) = last_gen[3][x][y][0  ] {neighbours += 1;}
                            // touches 3 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[3][crate::UNIVERSE_SIZE-1][y][z] {neighbours += 1;}
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x-1][y][z] {neighbours += 1;}
                            // touches 5 in y+1
                            if crate::UNIVERSE_SIZE >  y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][y+1][z] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == y+1 && let LifeDataContainer::Alive(_) = last_gen[5][x][0  ][z] {neighbours += 1;}
                            // touches 1 in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x][crate::UNIVERSE_SIZE-1][z] {neighbours += 1;}
                            if y > 0  && let LifeDataContainer::Alive(_) = last_gen[1][x][y-1][z] {neighbours += 1;}
                            // touches 0 and 3 in x-1 y+1
                            if crate::UNIVERSE_SIZE > y+1 {
                                if x > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][x-1][y+1][z] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x-1][y+1][z] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[0][crate::UNIVERSE_SIZE-1][y+1][z] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][crate::UNIVERSE_SIZE-1][y+1][z] {neighbours += 1;}
                                }
                            } else if x > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[0][x-1][0][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x-1][0][z] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[0][crate::UNIVERSE_SIZE-1][0][z] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][crate::UNIVERSE_SIZE-1][0][z] {neighbours += 1;}
                            }
                            // touches 2 and 3 y-1 z+1
                            if crate::UNIVERSE_SIZE > z+1 {
                                if y > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x][y-1][z+1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x][crate::UNIVERSE_SIZE-1][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x][crate::UNIVERSE_SIZE-1][z+1] {neighbours += 1;}
                                }
                            } else if y > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x][y-1][0] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x][crate::UNIVERSE_SIZE-1][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x][crate::UNIVERSE_SIZE-1][0] {neighbours += 1;}
                            }
                        } else if n == 5 {// dark grey touches light grey and white in the same xyz and red in the y below and dark blue in x+1
                            // CHECK 5 NEIGHBOURS IN SAME CUBE
                            if let LifeDataContainer::Alive(_) = last_gen[0][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[1][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[2][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[3][x][y][z] {neighbours += 1}
                            if let LifeDataContainer::Alive(_) = last_gen[4][x][y][z] {neighbours += 1}
                            // 2 FACE CHECKS
                            //the y >0 checks if we are the edge of the univ
                            if y > 0 && let LifeDataContainer::Alive(_) = last_gen[1][x  ][y-1][z] {neighbours += 1;} // touches red below
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[1][x  ][crate::UNIVERSE_SIZE-1][z] {neighbours += 1;} // touches red below (on the other side of the universe)
                            if x > 0 && let LifeDataContainer::Alive(_) = last_gen[3][x-1][y  ][z] {neighbours += 1;} // touches dark blue in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[3][crate::UNIVERSE_SIZE-1][y  ][z] {neighbours += 1;} // touches dark blue in x-1 (on the other side of the universe)
                            // 8 EDGE CHECKS
                            // touches 2 let LifeDataContainer::Alive(_) = in y-1
                            if y == 0 && let LifeDataContainer::Alive(_) = last_gen[2][x][crate::UNIVERSE_SIZE-1][z] {neighbours += 1;}
                            if y > 0  && let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z] {neighbours += 1;}
                            // touches 0 in z+1
                            if crate::UNIVERSE_SIZE >  z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y][z+1] {neighbours += 1;}
                            if crate::UNIVERSE_SIZE == z+1 && let LifeDataContainer::Alive(_) = last_gen[0][x][y][0  ] {neighbours += 1;}
                            // touches 2 in x-1
                            if x == 0 && let LifeDataContainer::Alive(_) = last_gen[2][crate::UNIVERSE_SIZE-1][y][z] {neighbours += 1;}
                            if x > 0  && let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][z] {neighbours += 1;}
                            // touches 2 in x-1 y-1
                            if y > 0 {
                                if x > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y-1][z] {neighbours += 1;}
                                } else if let LifeDataContainer::Alive(_) = last_gen[2][crate::UNIVERSE_SIZE-1][y-1][z] {neighbours += 1;}
                            } else if x > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x-1][crate::UNIVERSE_SIZE-1][z] {neighbours += 1;}
                            } else if let LifeDataContainer::Alive(_) = last_gen[2][crate::UNIVERSE_SIZE-1][crate::UNIVERSE_SIZE-1][z] {neighbours += 1;}
                            // touches 1 and 2 in x-1 z-1
                            if z > 0 {
                                if x > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][x-1][y][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][z-1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[1][crate::UNIVERSE_SIZE-1][y][z-1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[2][crate::UNIVERSE_SIZE-1][y][z-1] {neighbours += 1;}
                                }
                            } else if x > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[1][x-1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[2][x-1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[1][crate::UNIVERSE_SIZE-1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[2][crate::UNIVERSE_SIZE-1][y][crate::UNIVERSE_SIZE-1] {neighbours += 1;}
                            }
                            // touches 2 and 3 y-1 z+1
                            if crate::UNIVERSE_SIZE > z+1 {
                                if y > 0 {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x][y-1][z+1] {neighbours += 1;}
                                } else {
                                    if let LifeDataContainer::Alive(_) = last_gen[2][x][crate::UNIVERSE_SIZE-1][z+1] {neighbours += 1;}
                                    if let LifeDataContainer::Alive(_) = last_gen[3][x][crate::UNIVERSE_SIZE-1][z+1] {neighbours += 1;}
                                }
                            } else if y > 0 {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x][y-1][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x][y-1][0] {neighbours += 1;}
                            } else {
                                if let LifeDataContainer::Alive(_) = last_gen[2][x][crate::UNIVERSE_SIZE-1][0] {neighbours += 1;}
                                if let LifeDataContainer::Alive(_) = last_gen[3][x][crate::UNIVERSE_SIZE-1][0] {neighbours += 1;}
                            }
                        } else {
                            println!("Error: n was not in 0-5");
                        }
                        match entity_life {
                            LifeDataContainer::Alive(ent) => {//if alive in last gen
                                if neighbours > 3 || neighbours == 1 || neighbours == 0 {
                                    commands.entity(ent.to_owned()).despawn();
                                    next_gen[n][x][y][z] = LifeDataContainer::Dead(true);

                                    session.counter -= 1;
                                } else {
                                    //continue to be alive
                                    next_gen[n][x][y][z] = last_gen[n][x][y][z];
                                }
                            },
                            LifeDataContainer::Dead(_) => {// if dead in last gen
                                //if neighbours = 3 then become alive
                                if neighbours == 3 {
                                    let transform_new_life: bevy::prelude::Transform = create_life_xyz(n, x, y, z);
                                
                                    // make the life form exist!
                                    next_gen[n][x][y][z] = LifeDataContainer::Alive(commands.spawn_bundle(PbrBundle {
                                        mesh: session.life_form_meshes[n%2].clone(),
                                        material: session.life_form_materials[n].clone(),
                                        transform: transform_new_life,
                                        ..Default::default()
                                    }).insert(Life).id());
                                    
                                    //increment life counter
                                    session.counter += 1;
                                }
                            },
                        }
                    }
                }
            }
        }

        session.life = next_gen;
        session.generation += 1;
    }

    //println!("Total of lifeform entitys: {}", total_entities.to_string());
}