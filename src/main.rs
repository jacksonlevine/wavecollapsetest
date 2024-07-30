
mod shader;
mod camera;
mod texture;
mod statics;
mod maincomponents;
mod systems;

use std::{f32::consts::PI, sync::Mutex};

use bevy_ecs::{schedule::Schedule, system::{ResMut, Resource}, world::{Mut, World}};
use camera::Camera;
use glfw::{ffi::glfwGetTime, Action, Context, Key};
use gl::{types::*, ShaderSource};
use once_cell::sync::Lazy;
use shader::Shader;
use texture::Texture;
use tracing::info;

use glam::*;

use gl::*;

use crate::systems::*;

use statics::*;
use maincomponents::*;



#[derive(Resource, Clone)]
struct Vars {
    first_mouse: bool,
    mouse_focused: bool,
    sensitivity: f32
}

impl Default for Vars {
    fn default() -> Self {
        Self {
            first_mouse: true,
            mouse_focused: false,
            sensitivity: 1.0
        }
    }
}


pub fn cursor_pos(xpos: f64, ypos: f64, world: &mut World) {
    let mut vars =(*( world.resource_mut::<Vars>())).clone();
    let mut camera = world.resource_mut::<Camera>();
    if vars.mouse_focused {
        static mut LASTX: f64 = 0.0;
        static mut LASTY: f64 = 0.0;

        if vars.first_mouse {
            unsafe {
                LASTX = xpos;
                LASTY = ypos;
            }
            vars.first_mouse = false;
        }

        unsafe {
            let x_offset = (xpos - LASTX) * vars.sensitivity as f64;
            let y_offset = (LASTY - ypos) * vars.sensitivity as f64;

            LASTY = ypos;
            LASTX = xpos;

            camera.yaw += x_offset as f32;
            camera.pitch += y_offset as f32;

            camera.pitch = camera.pitch.clamp(-89.0, 89.0);

            camera.direction.x =
                camera.yaw.to_radians().cos() as f32 * camera.pitch.to_radians().cos() as f32;
            camera.direction.y = camera.pitch.to_radians().sin();
            camera.direction.z =
                camera.yaw.to_radians().sin() * camera.pitch.to_radians().cos();
            camera.direction = camera.direction.normalize();

            camera.right = Vec3::new(0.0, 1.0, 0.0)
                .cross(camera.direction)
                .normalize();
            camera.up = camera.direction.cross(camera.right).normalize();

            camera.recalculate();
            #[cfg(feature = "show_cam_pos")]
            println!(
                "Cam dir: {}, {}, {}",
                camlock.direction.x, camlock.direction.y, camlock.direction.z
            );
        }
    }
    drop(camera);
    *(world.resource_mut::<Vars>()) = vars;
    
}

pub fn set_mouse_focused(tf: bool, mut vars: Mut<Vars>) {
    if tf {
        vars.mouse_focused = true;
    } else {
        vars.mouse_focused = false;
        vars.first_mouse = true;
    }
}







pub fn bind_old_geometry(
    vbov: GLuint,
    vbouv: GLuint,
    vdata: &[f32],
    uvdata: &[f32],
    shader: &Shader,
    vao: GLuint
) {
    unsafe {
        // Upload vertex data to named buffer
        gl::NamedBufferData(
            vbov,
            (vdata.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vdata.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            info!("Bind world geom err (vbov): {}", error);
        }

        // Bind vertex buffer to the vertex array object
        gl::VertexArrayVertexBuffer(vao, 0, vbov, 0, (5 * std::mem::size_of::<f32>()) as GLsizei);
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            info!("OpenGL Error after associating vbov with vao: {}", error);
        }

        // Position attribute
        let pos_attrib = gl::GetAttribLocation(shader.shader_id, b"position\0".as_ptr() as *const i8);
        if pos_attrib == -1 {
            info!("Error: position attribute not found in shader.");
        } else {
            gl::EnableVertexArrayAttrib(vao, pos_attrib as GLuint);
            gl::VertexArrayAttribFormat(
                vao,
                pos_attrib as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
            );
            gl::VertexArrayAttribBinding(vao, pos_attrib as GLuint, 0);
        }

        // Block brightness attribute
        let brightness_attrib = gl::GetAttribLocation(shader.shader_id, b"blockRgb\0".as_ptr() as *const i8);
        if brightness_attrib == -1 {
            info!("Error: blockRgb attribute not found in shader.");
        } else {
            gl::EnableVertexArrayAttrib(vao, brightness_attrib as GLuint);
            gl::VertexArrayAttribIFormat(
                vao,
                brightness_attrib as GLuint,
                1,
                gl::UNSIGNED_INT,
                (3 * std::mem::size_of::<f32>()) as GLuint,
            );
            gl::VertexArrayAttribBinding(vao, brightness_attrib as GLuint, 0);
        }

        // Ambient brightness attribute
        let amb_brightness = gl::GetAttribLocation(shader.shader_id, b"ambientBright\0".as_ptr() as *const i8);
        if amb_brightness == -1 {
            info!("Error: ambientBright attribute not found in shader.");
        } else {
            gl::EnableVertexArrayAttrib(vao, amb_brightness as GLuint);
            gl::VertexArrayAttribFormat(
                vao,
                amb_brightness as GLuint,
                1,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as GLuint,
            );
            gl::VertexArrayAttribBinding(vao, amb_brightness as GLuint, 0);
        }

        // Upload UV data to named buffer
        gl::NamedBufferData(
            vbouv,
            (uvdata.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            uvdata.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            info!("Bind world geom err (vbouv): {}", error);
        }

        // Bind UV buffer to the vertex array object
        gl::VertexArrayVertexBuffer(vao, 1, vbouv, 0, (4 * std::mem::size_of::<f32>()) as GLsizei);
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            info!("OpenGL Error after associating vbouv with vao: {}", error);
        }

        // UV attribute
        let uv_attrib = gl::GetAttribLocation(shader.shader_id, b"uv\0".as_ptr() as *const i8);
        if uv_attrib == -1 {
            info!("Error: uv attribute not found in shader.");
        } else {
            gl::EnableVertexArrayAttrib(vao, uv_attrib as GLuint);
            gl::VertexArrayAttribFormat(
                vao,
                uv_attrib as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                0,
            );
            gl::VertexArrayAttribBinding(vao, uv_attrib as GLuint, 1);
        }
    }
}


pub fn draw_old_geometry(vvbo: GLuint, uvvbo: GLuint, camera: &Camera, shader: &Shader, length: usize, vao: GLuint) {


    unsafe {
        gl::BindVertexArray(vao);
        gl::UseProgram(shader.shader_id);
    }
    

        static mut MVP_LOC: i32 = -1;
        static mut CAM_POS_LOC: i32 = 0;
        static mut AMBIENT_BRIGHT_MULT_LOC: i32 = 0;
        static mut VIEW_DISTANCE_LOC: i32 = 0;
        static mut UNDERWATER_LOC: i32 = 0;
        static mut CAM_DIR_LOC: i32 = 0;
        static mut SUNSET_LOC: i32 = 0;
        static mut SUNRISE_LOC: i32 = 0;
        static mut WALKBOB_LOC: i32 = 0;
        unsafe {
            if MVP_LOC == -1 {

                MVP_LOC =
                    gl::GetUniformLocation(shader.shader_id, b"mvp\0".as_ptr() as *const i8);
                //info!("MVP LOC: {}", MVP_LOC);

                WALKBOB_LOC = gl::GetUniformLocation(
                    shader.shader_id,
                    b"walkbob\0".as_ptr() as *const i8,
                );

                CAM_POS_LOC = gl::GetUniformLocation(
                    shader.shader_id,
                    b"camPos\0".as_ptr() as *const i8,
                );
                AMBIENT_BRIGHT_MULT_LOC = gl::GetUniformLocation(
                    shader.shader_id,
                    b"ambientBrightMult\0".as_ptr() as *const i8,
                );
                VIEW_DISTANCE_LOC = gl::GetUniformLocation(
                    shader.shader_id,
                    b"viewDistance\0".as_ptr() as *const i8,
                );
                UNDERWATER_LOC = gl::GetUniformLocation(
                    shader.shader_id,
                    b"underWater\0".as_ptr() as *const i8,
                );
                CAM_DIR_LOC = gl::GetUniformLocation(
                    shader.shader_id,
                    b"camDir\0".as_ptr() as *const i8,
                );
                SUNSET_LOC = gl::GetUniformLocation(
                    shader.shader_id,
                    b"sunset\0".as_ptr() as *const i8,
                );
                SUNRISE_LOC = gl::GetUniformLocation(
                    shader.shader_id,
                    b"sunrise\0".as_ptr() as *const i8,
                );
            }


            gl::UniformMatrix4fv(MVP_LOC, 1, gl::FALSE, camera.mvp.to_cols_array().as_ptr());
            gl::Uniform3f(
                CAM_POS_LOC,
                camera.position.x,
                camera.position.y,
                camera.position.z,
            );
            gl::Uniform1f(AMBIENT_BRIGHT_MULT_LOC, 1.0);
            gl::Uniform1f(VIEW_DISTANCE_LOC, 8.0);
            gl::Uniform1f(UNDERWATER_LOC, 0.0);
            gl::Uniform3f(
                CAM_DIR_LOC,
                camera.direction.x,
                camera.direction.y,
                camera.direction.z,
            );
            gl::Uniform1f(SUNSET_LOC, 0.0);
            gl::Uniform1f(WALKBOB_LOC, 0.0);
            gl::Uniform1f(SUNRISE_LOC, 0.0);
            gl::Uniform1i(
                gl::GetUniformLocation(
                    shader.shader_id,
                    b"ourTexture\0".as_ptr() as *const i8,
                ),
                0,
            );
            // let fc = Planets::get_fog_col(self.chunksys.read().unwrap().planet_type as u32);
            // gl::Uniform4f(
            //     FOGCOL_LOC,
            //     fc.0, 
            //     fc.1,
            //     fc.2,
            //     fc.3
            // );

        }

        //bind_old_geometry_no_upload(vvbo, uvvbo, &shader);



        unsafe {
            //gl::Disable(gl::CULL_FACE);
            gl::DrawArrays(gl::TRIANGLES, 0, length as i32 / 5);
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                info!("OpenGL Error after drawing arrays: {}", error);
            }
            //gl::Enable(gl::CULL_FACE);
            // info!("Chunk rending!");
        }
    }



#[derive(Resource, Default)]
pub struct JControls {
    f: bool,
    b: bool,
    r: bool,
    l: bool
}

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();


    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));



    let (mut window, events) = glfw.create_window(1280, 720, "3d test", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));


    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CW);
    }


    let mut world = World::new();

    world.insert_resource(Vars::default());
    world.insert_resource(DeltaTime::default());
    world.insert_resource(Camera::new());
    world.insert_resource(JControls::default());
    world.insert_resource(Texture::default());
    world.insert_resource(Shader::new("oldvert.glsl", "oldfrag.glsl"));

    LOAD_IN_ALL_MODELS(world.resource::<Shader>());


    world.spawn((
        Position { pos: Vec3::ZERO },
        PlayerCamHere{},
        ModelIndex{ jmodel: JModelIndex::PlayerModel}
    ));


    world.spawn((
        Position { pos: Vec3::ZERO },
        ModelIndex{ jmodel: JModelIndex::TestTreeBush}
    ));


    world.spawn((
        Position { pos: Vec3::new(4.0, 0.0, 0.0) },
        ModelIndex{ jmodel: JModelIndex::PlayerModel}
    ));

    world.spawn((
        Position { pos: Vec3::new(-4.0, 0.0, 4.0) },
        ModelIndex{ jmodel: JModelIndex::TestTreeBush}
    ));

    let mut schedule = Schedule::default();

    schedule.add_systems((player_respond_to_controls, draw_all_and_update_delta_time));

    while !window.should_close() {

        
        schedule.run(&mut world);


        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_controls(world.resource_mut::<JControls>(), event.clone());

            match event {
                glfw::WindowEvent::Key(key, code, action, _) => {
                    if key == Key::Escape {
                        window.set_should_close(true);
                    }
                }
                glfw::WindowEvent::MouseButton(mousebutton, action, _) => {


                    if mousebutton == glfw::MouseButtonLeft {
                        if action == Action::Press {
                            window.set_cursor_mode(glfw::CursorMode::Disabled);
                            set_mouse_focused(true, world.resource_mut::<Vars>());
                        }
                    }
                
                        
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    cursor_pos(xpos, ypos, &mut world);
                    
                }
                _ => {

                }
            }
        }
    }
}


pub fn bind_old_geometry_no_upload(
    vbov: GLuint,
    vbouv: GLuint,
    shader: &Shader,
    vao: GLuint
) {
    unsafe {


        // Bind vertex buffer to the vertex array object
        gl::VertexArrayVertexBuffer(vao, 0, vbov, 0, (5 * std::mem::size_of::<f32>()) as GLsizei);
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            info!("OpenGL Error after associating vbov with vao: {}", error);
        }

        // Position attribute
        let pos_attrib = gl::GetAttribLocation(shader.shader_id, b"position\0".as_ptr() as *const i8);
        gl::EnableVertexArrayAttrib(vao, pos_attrib as GLuint);
        gl::VertexArrayAttribFormat(
            vao,
            pos_attrib as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            0,
        );
        gl::VertexArrayAttribBinding(vao, pos_attrib as GLuint, 0);

        // Block brightness attribute
        let brightness_attrib = gl::GetAttribLocation(shader.shader_id, b"blockRgb\0".as_ptr() as *const i8);
        gl::EnableVertexArrayAttrib(vao, brightness_attrib as GLuint);
        gl::VertexArrayAttribIFormat(
            vao,
            brightness_attrib as GLuint,
            1,
            gl::UNSIGNED_INT,
            (3 * std::mem::size_of::<u32>()) as GLuint,
        );
        gl::VertexArrayAttribBinding(vao, brightness_attrib as GLuint, 0);

        // Ambient brightness attribute
        let amb_brightness = gl::GetAttribLocation(shader.shader_id, b"ambientBright\0".as_ptr() as *const i8);
        gl::EnableVertexArrayAttrib(vao, amb_brightness as GLuint);
        gl::VertexArrayAttribFormat(
            vao,
            amb_brightness as GLuint,
            1,
            gl::FLOAT,
            gl::FALSE,
            (4 * std::mem::size_of::<f32>()) as GLuint,
        );
        gl::VertexArrayAttribBinding(vao, amb_brightness as GLuint, 0);


        // Bind UV buffer to the vertex array object
        gl::VertexArrayVertexBuffer(vao, 1, vbouv, 0, (4 * std::mem::size_of::<f32>()) as GLsizei);
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            info!("OpenGL Error after associating vbouv with vao: {}", error);
        }

        // UV attribute
        let uv_attrib = gl::GetAttribLocation(shader.shader_id, b"uv\0".as_ptr() as *const i8);
        gl::EnableVertexArrayAttrib(vao, uv_attrib as GLuint);
        gl::VertexArrayAttribFormat(
            vao,
            uv_attrib as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE,
            0,
        );
        gl::VertexArrayAttribBinding(vao, uv_attrib as GLuint, 1);

        // // UV base attribute
        // let uv_attrib2 = gl::GetAttribLocation(shader.shader_id, b"uvbase\0".as_ptr() as *const i8);
        // gl::EnableVertexArrayAttrib(vao, uv_attrib2 as GLuint);
        // gl::VertexArrayAttribFormat(
        //     vao,
        //     uv_attrib2 as GLuint,
        //     2,
        //     gl::FLOAT,
        //     gl::FALSE,
        //     (2 * std::mem::size_of::<f32>()) as GLuint,
        // );
        // gl::VertexArrayAttribBinding(vao, uv_attrib2 as GLuint, 1);
    }
}
