use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

const FPS_CAP: f32 = (1.0 / 60.0) * 1000.0;

fn main() {
    let sdl = sdl2::init().expect("Impossible to load sdl");

    sdl.mouse().show_cursor(false);
    sdl.mouse().capture(true);
    sdl.mouse().set_relative_mouse_mode(true);

    let video_system = sdl.video().expect("No video subsystem available");

    let gl_attr = video_system.gl_attr();
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let width = 1400 as i32;
    let height = 900 as i32;
    let window_init = video_system
        .window("Game", width as u32, height as u32)
        .opengl()
        // .fullscreen()
        .build();

    let window = window_init.expect("Window not initialized");

    let gl_contex_init = window.gl_create_context();

    let _gl_context = gl_contex_init.expect("Gl context not initialized");
    let _gl = gl::load_with(|s| video_system.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, width, height);
    }

    let mut event_pump = sdl.event_pump().expect("No event pump");

    let mut timer = sdl.timer().expect("No timer");

    let mut running = true;
    let mut last_update = timer.ticks();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let mut attributes = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATOMIC_COUNTERS, &mut attributes) };
    println!("Attrib: {}", attributes);

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0., //
        0.5, -0.5, 0., //
        0., 0.5, 0.,
    ];

    let vertices_ebo: Vec<f32> = vec![
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
    ];
    let indices: Vec<u32> = vec![
        // note that we start from 0!
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

    let vertex_shader_source = std::ffi::CString::new(
        "
        #version 330 core
        layout (location = 0) in vec3 aPos;

        void main()
        {
            gl_Position = vec4(aPos, 1.0f);
        }
    ",
    )
    .unwrap();
    let vertex_shader = compile_shader(vertex_shader_source, gl::VERTEX_SHADER);

    let fragment_shader_source = std::ffi::CString::new(
        "
        #version 330 core
        out vec4 FragColor;

        void main()
        {
            FragColor = vec4(0.5f, 0.1f, 0.3f, 1.0f);
        }
    ",
    )
    .unwrap();
    let fragment_shader = compile_shader(fragment_shader_source, gl::FRAGMENT_SHADER);

    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            panic!("Linking failed");
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

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
            (3 * std::mem::size_of::<f32>()) as _,
            0 as _,
        );
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let cstr = std::ffi::CString::new("ourColor").unwrap();
    while running {
        let milliseconds = timer.ticks();
        let start = timer.performance_counter();
        let delta = (milliseconds - last_update) as f32 / 1000.;

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
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => {}
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {}
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {}
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {}
                Event::MouseWheel { y, .. } => {}
                _ => {}
            }
        }

        // Update last_update for delta
        last_update = timer.ticks();

        // Render
        unsafe {
            gl::ClearColor(0.2, 0.5, 0., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        };

        let end = timer.performance_counter();
        let elapsed = (end - start) as f32 / timer.performance_frequency() as f32 * 1000.0;

        if (FPS_CAP - elapsed) > 0.0 {
            timer.delay((FPS_CAP - elapsed).floor() as u32);
        }

        println!("Delta time: {:#?}", delta);

        window.gl_swap_window();
    }
}

fn compile_shader(source: std::ffi::CString, shader_type: gl::types::GLenum) -> gl::types::GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            panic!("Compiling failed");
        }
    }
    shader
}
