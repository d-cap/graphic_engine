extern crate nalgebra_glm as glm;

use camera::Camera;
use glfw::{Action, Context, Key, MouseButton};
use shader::Shader;
use stb_image::image::LoadResult;
use utils::Input;

mod camera;
mod shader;
mod utils;

const FPS_CAP: f32 = (1.0 / 60.0) * 1000.0;

fn main() {
    let mut camera = Camera {
        position: glm::Vec3::new(0., 0., 3.),
        ..Camera::default()
    };

    let width = 1400;
    let height = 900;

    let mut running = true;

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(width, height, "GL Engine", glfw::WindowMode::Windowed)
        .expect("Window creation failed");

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);
    window.set_key_polling(true);

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|symbol| window.get_proc_address(symbol));

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let object_shader = Shader::new(
        "shader/object_vertex_shader.vs",
        "shader/object_fragment_shader.fs",
    );

    let light_shader = Shader::new(
        "shader/light_vertex_shader.vs",
        "shader/light_fragment_shader.fs",
    );

    let (object_vao, light_vao) = create_vao();

    let diffuse_texture = create_texture("images/container2.png");

    let specular_texture = create_texture("images/container2_specular.png");

    let mut old_input = Input::new(width as f32 / 2., height as f32 / 2.);
    let mut new_input;

    object_shader.use_shader();
    object_shader.set_i32("material.diffuse", 0);
    object_shader.set_i32("material.specular", 1);
    object_shader.set_f32("material.shininess", 64.);
    object_shader.set_3_f32("light.ambient", 0.2, 0.2, 0.2);
    object_shader.set_3_f32("light.diffuse", 0.5, 0.5, 0.5);
    object_shader.set_3_f32("light.specular", 1.0, 1.0, 1.0);

    let cube_positions: Vec<glm::Vec3> = vec![
        glm::vec3(0., 0., 0.),       //
        glm::vec3(2., 5., -15.0),    //
        glm::vec3(-1.5, -2.2, -2.5), //
        glm::vec3(-3.8, -2., -12.3), //
        glm::vec3(2.4, -0.4, -3.5),  //
        glm::vec3(-1.7, 3., -7.5),   //
        glm::vec3(1.3, -2., -2.5),   //
        glm::vec3(1.5, 2., -2.5),    //
        glm::vec3(1.5, 0.2, -1.5),   //
        glm::vec3(-1.3, 1., -1.5),   //
    ];

    let mut delta: f32;
    let mut last_frame = 0.;

    let mut last_x = width as f32 / 2.;
    let mut last_y = height as f32 / 2.;
    let mut first_mouse = true;

    while running {
        let seconds = glfw.get_time() as f32;
        delta = seconds - last_frame;
        last_frame = seconds;

        new_input = old_input.clone();
        new_input.delta_time = delta;
        new_input.mouse_scroll = 0.;

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, _, _) => running = false,
                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    new_input.up.ended_down = true;
                    new_input.up.half_transition_count += 1;
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
                    new_input.up.ended_down = false;
                    new_input.up.half_transition_count = 0;
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                    new_input.right.ended_down = true;
                    new_input.right.half_transition_count += 1;
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    new_input.right.ended_down = false;
                    new_input.right.half_transition_count = 0;
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                    new_input.down.ended_down = true;
                    new_input.down.half_transition_count += 1;
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
                    new_input.down.ended_down = false;
                    new_input.down.half_transition_count = 0;
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    new_input.left.ended_down = true;
                    new_input.left.half_transition_count += 1;
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
                    new_input.left.ended_down = false;
                    new_input.left.half_transition_count = 0;
                }
                glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => {
                    new_input.left_bracket.ended_down = true;
                    new_input.left_bracket.half_transition_count += 1;
                }
                glfw::WindowEvent::Key(Key::Q, _, Action::Release, _) => {
                    new_input.left_bracket.ended_down = false;
                    new_input.left_bracket.half_transition_count = 0;
                }
                glfw::WindowEvent::Key(Key::E, _, Action::Press, _) => {
                    new_input.right_bracket.ended_down = true;
                    new_input.right_bracket.half_transition_count += 1;
                }
                glfw::WindowEvent::Key(Key::E, _, Action::Release, _) => {
                    new_input.right_bracket.ended_down = false;
                    new_input.right_bracket.half_transition_count = 0;
                }
                glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
                    let x_pos = x_pos as f32;
                    let y_pos = y_pos as f32;
                    if first_mouse {
                        last_x = x_pos;
                        last_y = y_pos;
                        first_mouse = false;
                    }
                    let x_offset = x_pos - last_x;
                    let y_offset = last_y - y_pos;

                    last_x = x_pos;
                    last_y = y_pos;
                    camera.move_mouse(x_offset, y_offset);
                }
                glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                    new_input.mouse_left.ended_down = true;
                    new_input.mouse_left.half_transition_count += 1;
                }
                glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => {
                    new_input.mouse_left.ended_down = false;
                    new_input.mouse_left.half_transition_count = 0;
                }
                glfw::WindowEvent::Scroll(_, y_offset) => {
                    new_input.mouse_scroll = y_offset as f32;
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

        // Update
        let light_pos = glm::Vec3::new(
            2. * seconds.cos(),
            2. * seconds.sin() + 2. * seconds.cos(),
            2. * seconds.sin(),
        );

        let projection_matrix: glm::Mat4 =
            glm::perspective(width as f32 / height as f32, camera.zoom, 0.1, 100.0);

        object_shader.use_shader();
        object_shader.set_mat4_f32("view", camera.view_matrix());
        object_shader.set_mat4_f32("projection", projection_matrix);
        object_shader.set_vec3_f32("lightPos", &light_pos);
        object_shader.set_vec3_f32("cameraPos", &camera.position);
        object_shader.set_vec3_f32("light.position", &camera.position);
        object_shader.set_vec3_f32("light.direction", &camera.front);
        object_shader.set_f32("light.cutOff", 12.5_f32.to_radians().cos());
        object_shader.set_f32("light.outerCutOff", 17.5_f32.to_radians().cos());
        object_shader.set_f32("light.constant", 1.);
        object_shader.set_f32("light.linear", 0.09);
        object_shader.set_f32("light.quadratic", 0.032);

        light_shader.use_shader();
        light_shader.set_mat4_f32("view", camera.view_matrix());
        light_shader.set_mat4_f32("projection", projection_matrix);

        // Render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.);
            // gl::ClearColor(0.5, 0.5, 0.6, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        unsafe {
            object_shader.use_shader();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuse_texture);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specular_texture);
            gl::BindVertexArray(object_vao);
            for (i, c) in cube_positions.iter().enumerate() {
                let angle = 20. * i as f32;
                let model = glm::rotate(
                    &glm::translate(&glm::Mat4::identity(), &c),
                    angle.to_radians(),
                    &glm::Vec3::new(1., 0.3, 0.5),
                );
                object_shader.set_mat4_f32("model", model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        unsafe {
            light_shader.use_shader();
            gl::BindVertexArray(light_vao);
            let model = glm::scale(
                &glm::translate(&glm::Mat4::identity(), &light_pos),
                &glm::Vec3::new(0.25, 0.25, 0.25),
            );
            light_shader.set_mat4_f32("model", model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        unsafe {
            gl::BindVertexArray(0);
        };

        let elapsed = glfw.get_time() as f32 - seconds;
        if (FPS_CAP - elapsed) > 0.0 {
            std::thread::sleep(std::time::Duration::from_millis(
                (FPS_CAP - elapsed).floor() as u64,
            ));
        }

        old_input = new_input;

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn create_texture(image_path: &str) -> gl::types::GLuint {
    let mut texture = 0;
    unsafe {
        let image = stb_image::image::load(image_path);
        let image = match image {
            LoadResult::ImageF32(_) => panic!(),
            LoadResult::ImageU8(image) => image,
            LoadResult::Error(_) => panic!(),
        };
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

        let format = match image.depth {
            1 => gl::RED,
            3 => gl::RGB,
            4 => gl::RGBA,
            _ => panic!(),
        };

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as _,
            image.width as _,
            image.height as _,
            0,
            format,
            gl::UNSIGNED_BYTE,
            image.data.as_ptr() as _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    texture
}

fn create_vao() -> (gl::types::GLuint, gl::types::GLuint) {
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, -0.5, 0., 0., -1., 0., 0., //
        0.5, -0.5, -0.5, 0., 0., -1., 1., 0., //
        0.5, 0.5, -0.5, 0., 0., -1., 1., 1., //
        0.5, 0.5, -0.5, 0., 0., -1., 1., 1., //
        -0.5, 0.5, -0.5, 0., 0., -1., 0., 1., //
        -0.5, -0.5, -0.5, 0., 0., -1., 0., 0., //
        //
        -0.5, -0.5, 0.5, 0., 0., 1., 0., 0., //
        0.5, -0.5, 0.5, 0., 0., 1., 1., 0., //
        0.5, 0.5, 0.5, 0., 0., 1., 1., 1., //
        0.5, 0.5, 0.5, 0., 0., 1., 1., 1., //
        -0.5, 0.5, 0.5, 0., 0., 1., 0., 1., //
        -0.5, -0.5, 0.5, 0., 0., 1., 0., 0., //
        //
        -0.5, 0.5, 0.5, -1., 0., 0., 1., 0., //
        -0.5, 0.5, -0.5, -1., 0., 0., 1., 1., //
        -0.5, -0.5, -0.5, -1., 0., 0., 0., 1., //
        -0.5, -0.5, -0.5, -1., 0., 0., 0., 1., //
        -0.5, -0.5, 0.5, -1., 0., 0., 0., 0., //
        -0.5, 0.5, 0.5, -1., 0., 0., 1., 0., //
        //
        0.5, 0.5, 0.5, 1., 0., 0., 1., 0., //
        0.5, 0.5, -0.5, 1., 0., 0., 1., 1., //
        0.5, -0.5, -0.5, 1., 0., 0., 0., 1., //
        0.5, -0.5, -0.5, 1., 0., 0., 0., 1., //
        0.5, -0.5, 0.5, 1., 0., 0., 0., 0., //
        0.5, 0.5, 0.5, 1., 0., 0., 1., 0., //
        //
        -0.5, -0.5, -0.5, 0., -1., 0., 0., 1., //
        0.5, -0.5, -0.5, 0., -1., 0., 1., 1., //
        0.5, -0.5, 0.5, 0., -1., 0., 1., 0., //
        0.5, -0.5, 0.5, 0., -1., 0., 1., 0., //
        -0.5, -0.5, 0.5, 0., -1., 0., 0., 0., //
        -0.5, -0.5, -0.5, 0., -1., 0., 0., 1., //
        //
        -0.5, 0.5, -0.5, 0., 1., 0., 0., 1., //
        0.5, 0.5, -0.5, 0., 1., 0., 1., 1., //
        0.5, 0.5, 0.5, 0., 1., 0., 1., 0., //
        0.5, 0.5, 0.5, 0., 1., 0., 1., 0., //
        -0.5, 0.5, 0.5, 0., 1., 0., 0., 0., //
        -0.5, 0.5, -0.5, 0., 1., 0., 0., 1.0, //
    ];
    let mut vao = 0;
    let mut light_vao = 0;
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
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as _,
            0 as _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as _,
            (3 * std::mem::size_of::<f32>()) as _,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as _,
            (6 * std::mem::size_of::<f32>()) as _,
        );
        gl::EnableVertexAttribArray(2);

        gl::GenVertexArrays(1, &mut light_vao);
        gl::BindVertexArray(light_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as _,
            0 as _,
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    (vao, light_vao)
}
