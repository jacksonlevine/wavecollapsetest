use bevy_ecs::prelude::*;
use gl::types::*;
use glfw::{ffi::glfwGetTime, Action, Key};
use crate::{bind_old_geometry_no_upload, camera::Camera, cursor_pos, draw_old_geometry, maincomponents::*, shader::Shader, JControls, JModel, MODELS};


#[derive(Resource, Default)]
pub struct DeltaTime {
    delta_time: f32
}

pub fn handle_controls(
    mut controls: Mut<JControls>,
    event: glfw::WindowEvent
) {

        match event {
            // glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            //     window.set_should_close(true)
            // }
            glfw::WindowEvent::Key(key, _, action, _) => {
                match key {
                    Key::W => {
                        unsafe {
                            controls.f = (action == Action::Press || action == Action::Repeat);
                        }
                        
                    }
                    Key::S => {
                        unsafe {
                            controls.b = (action == Action::Press || action == Action::Repeat);
                        }
                        
                    }
                    Key::D => {
                        unsafe {
                            controls.r = (action == Action::Press || action == Action::Repeat);
                        }
                        
                    }
                    Key::A => {
                        unsafe {
                            controls.l = (action == Action::Press || action == Action::Repeat);
                        }
                        
                    }
                    _ => {

                    }
                }
            }
            glfw::WindowEvent::MouseButton(mousebutton, action, _) => {


                  
                    
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {

                
            }
            _ => {}

            
        
    }
}

pub fn player_respond_to_controls(
    mut query2: Query<(&PlayerCamHere, &mut Position, &ModelIndex)>, 
    delta_time: Res<DeltaTime>, 
    mut camera: ResMut<Camera>,
    controls: Res<JControls>
) {
    let delta_time = delta_time.delta_time;
    for (_, mut myposition, mymodel) in &mut query2 {
        {
            unsafe {

                if controls.f {
                    let mut dir = camera.direction.clone();
                    dir.y = 0.0;
                    camera.position += dir * delta_time;
                }
                if controls.b {
                    let mut dir = camera.direction.clone();
                    dir.y = 0.0;
                    camera.position -= dir * delta_time;
                }
                if controls.r {
                    let mut dir = camera.right.clone();
                    dir.y = 0.0;
                    camera.position -= dir * delta_time;
                }
                if controls.l {
                    let mut dir = camera.right.clone();
                    dir.y = 0.0;
                    camera.position += dir * delta_time;
                }
                if controls.f || controls.b || controls.r || controls.l {
                    camera.recalculate();
                }
            }
            (*myposition) = Position { pos: camera.position };
        }
    }

}

pub fn draw_all_and_update_delta_time(query: Query<(&Position, &ModelIndex)>, mut dt: ResMut<DeltaTime>, camera: Res<Camera>, shader: Res<Shader>) {
    draw_all_jmodels(query, camera, shader);

    static mut PREV_TIME: f32 = 0.0;
    unsafe {
        let current_time = unsafe { glfwGetTime() as f32 };
        (*dt.as_mut()) = DeltaTime {
            delta_time: (current_time - PREV_TIME).min(0.05)
        };
        PREV_TIME = current_time;
    }
}

pub fn draw_all_jmodels(query: Query<(&Position, &ModelIndex)>, camera: Res<Camera>, shader: Res<Shader>) {

    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
    }

    // static mut VVBO: GLuint = 0;
    // static mut UVVBO: GLuint = 0;

    for (pos, modelindex) in &query {
        let jm = (&modelindex.jmodel).clone();


        let ind: usize = jm.into();
        let model: &JModel = &MODELS[ind];

        unsafe {
            gl::BindVertexArray(model.vao);
        }


        let verts = &model.verts;





        // unsafe {
        //     if VVBO == 0 {
        //         gl::CreateBuffers(1, &mut VVBO);
        //         gl::CreateBuffers(1, &mut UVVBO);
        //     }
        // }
        
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(
                    shader.shader_id,
                    b"transpos\0".as_ptr() as *const i8,
                ),
                pos.pos.x,
                pos.pos.y,
                pos.pos.z
            );
        }


        


        unsafe {
            bind_old_geometry_no_upload(model.vbo, model.uvbo, &shader, model.vao);
            draw_old_geometry(model.vbo, model.uvbo, &camera, &shader, verts.len(), model.vao);
        }
    }
}

