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

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0., //
        0.5, -0.5, 0., //
        0., 0.5, 0.,
    ];

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
            gl::ClearColor(0.5, 0., 0.5, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
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
