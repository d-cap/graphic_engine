extern crate nalgebra_glm as glm;

use camera::Camera;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use shader::Shader;
use utils::{to_radians, Input};

mod camera;
mod shader;
mod utils;

const FPS_CAP: f32 = (1.0 / 60.0) * 1000.0;

fn main() {
    let sdl_context = sdl2::init().expect("Impossible to load sdl");

    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().capture(true);
    sdl_context.mouse().set_relative_mouse_mode(true);

    let video_system = sdl_context.video().expect("No video subsystem available");

    let gl_attr = video_system.gl_attr();
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let width = 1400 as i32;
    let height = 900 as i32;
    let window_init = video_system
        .window("Game", width as u32, height as u32)
        .opengl()
        .input_grabbed()
        .borderless()
        // .fullscreen()
        .build();

    let window = window_init.expect("Window not initialized");

    let gl_contex_init = window.gl_create_context();

    let _gl_context = gl_contex_init.expect("Gl context not initialized");
    let _gl = gl::load_with(|s| video_system.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, width, height);
    }

    let mut event_pump = sdl_context.event_pump().expect("No event pump");

    let mut timer = sdl_context.timer().expect("No timer");

    let mut running = true;
    let mut last_update = timer.ticks();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let shader = Shader::new(
        "shader/triangle_vertex_shader.vs",
        "shader/triangle_fragment_shader.fs",
    );

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, -0.5, 0.0, 0.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 1.0, //
        0.5, 0.5, 0.5, 1.0, 1.0, //
        -0.5, 0.5, 0.5, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        -0.5, 0.5, 0.5, 1.0, 0.0, //
        -0.5, 0.5, -0.5, 1.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        -0.5, 0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        0.5, -0.5, -0.5, 0.0, 1.0, //
        0.5, -0.5, -0.5, 0.0, 1.0, //
        0.5, -0.5, 0.5, 0.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        0.5, -0.5, -0.5, 1.0, 1.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        -0.5, 0.5, 0.5, 0.0, 0.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, //
    ];

    let cube_positions = vec![
        glm::Vec3::new(0.0, 0.0, 0.0),
        glm::Vec3::new(2.0, 5.0, -15.0),
        glm::Vec3::new(-1.5, -2.2, -2.5),
        glm::Vec3::new(-3.8, -2.0, -12.3),
        glm::Vec3::new(2.4, -0.4, -3.5),
        glm::Vec3::new(-1.7, 3.0, -7.5),
        glm::Vec3::new(1.3, -2.0, -2.5),
        glm::Vec3::new(1.5, 2.0, -2.5),
        glm::Vec3::new(1.5, 0.2, -1.5),
        glm::Vec3::new(-1.3, 1.0, -1.5),
    ];

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as _,
            vertices.as_ptr() as _,
            gl::STATIC_DRAW,
        );
    }

    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as _,
            0 as _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as _,
            (3 * std::mem::size_of::<f32>()) as _,
        );
        gl::EnableVertexAttribArray(1);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let image_texture_1 = image::open("images/container.jpg").unwrap();
    let texture_1 = create_texture(false, image_texture_1);

    let image_texture_2 = image::open("images/awesomeface.png").unwrap().flipv();
    let texture_2 = create_texture(true, image_texture_2);

    shader.use_shader();
    shader.set_i32("texture1", 0);
    shader.set_i32("texture2", 1);

    let mut old_input = Input::new(width as f32 / 2., height as f32 / 2.);
    let mut new_input;

    let mut camera = Camera::new(
        glm::Vec3::new(0., 0., 3.),
        glm::Vec3::new(0., 0., -1.),
        glm::Vec3::new(0., 1., 0.),
        2.5,
        width as f32,
        height as f32,
        0.1,
    );

    while running {
        let milliseconds = timer.ticks();
        let seconds = milliseconds as f32 / 1000.;
        let start = timer.performance_counter();
        let delta = (milliseconds - last_update) as f32 / 1000.;

        new_input = old_input.clone();
        new_input.delta_time = delta;
        new_input.mouse_scroll = 0.;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    new_input.up.ended_down = true;
                    new_input.up.half_transition_count += 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    new_input.up.ended_down = false;
                    new_input.up.half_transition_count = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    new_input.right.ended_down = true;
                    new_input.right.half_transition_count += 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    new_input.right.ended_down = false;
                    new_input.right.half_transition_count = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    new_input.down.ended_down = true;
                    new_input.down.half_transition_count += 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    new_input.down.ended_down = false;
                    new_input.down.half_transition_count = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    new_input.left.ended_down = true;
                    new_input.left.half_transition_count += 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    new_input.left.ended_down = false;
                    new_input.left.half_transition_count = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    new_input.left_bracket.ended_down = true;
                    new_input.left_bracket.half_transition_count += 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    new_input.left_bracket.ended_down = false;
                    new_input.left_bracket.half_transition_count = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    new_input.right_bracket.ended_down = true;
                    new_input.right_bracket.half_transition_count += 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    new_input.right_bracket.ended_down = false;
                    new_input.right_bracket.half_transition_count = 0;
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    new_input.mouse.x += xrel as f32;
                    new_input.mouse.y += yrel as f32;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    new_input.mouse_left.ended_down = true;
                    new_input.mouse_left.half_transition_count += 1;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    new_input.mouse_left.ended_down = false;
                    new_input.mouse_left.half_transition_count = 0;
                }
                Event::MouseWheel { y, .. } => {
                    new_input.mouse_scroll = y as f32;
                }
                _ => {}
            }
        }

        if new_input.up.ended_down {
            camera.move_forward(new_input.delta_time);
        }

        if new_input.down.ended_down {
            camera.move_backward(new_input.delta_time);
        }

        if new_input.left.ended_down {
            camera.move_left(new_input.delta_time);
        }

        if new_input.right.ended_down {
            camera.move_right(new_input.delta_time);
        }

        if old_input.mouse_scroll != new_input.mouse_scroll {
            camera.change_fov(new_input.mouse_scroll);
        }

        if old_input.mouse != new_input.mouse {
            camera.move_mouse(new_input.mouse.x, new_input.mouse.y);
        }

        shader.set_mat4_f32("view", camera.view_matrix());

        shader.set_mat4_f32("projection", camera.projection_matrix());

        // Update last_update for delta
        last_update = timer.ticks();

        // Render
        unsafe {
            gl::ClearColor(0.5, 0.5, 0.6, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            shader.use_shader();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_2);
            gl::BindVertexArray(vao);
        }

        for (i, c) in cube_positions.iter().enumerate() {
            unsafe {
                let model = glm::rotate(
                    &glm::translate(&glm::Mat4::identity(), &c),
                    to_radians(20. * i as f32),
                    &glm::Vec3::new(1., 0.5, 0.3),
                );
                shader.set_mat4_f32("model", model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        unsafe {
            gl::BindVertexArray(0);
        };

        let end = timer.performance_counter();
        let elapsed = (end - start) as f32 / timer.performance_frequency() as f32 * 1000.0;

        if (FPS_CAP - elapsed) > 0.0 {
            timer.delay((FPS_CAP - elapsed).floor() as u32);
        }

        old_input = new_input;

        window.gl_swap_window();
    }
}

fn create_texture(include_alpha: bool, image: image::DynamicImage) -> gl::types::GLuint {
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as _,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
        let image_ptr = if include_alpha {
            image.to_rgba8().to_vec()
        } else {
            image.to_rgb8().to_vec()
        };
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            if include_alpha { gl::RGBA } else { gl::RGB } as _,
            image.width() as _,
            image.height() as _,
            0,
            if include_alpha { gl::RGBA } else { gl::RGB },
            gl::UNSIGNED_BYTE,
            image_ptr.as_ptr() as _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    texture
}
