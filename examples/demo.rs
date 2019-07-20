extern crate sdl2;
extern crate imgui;
extern crate imgui_sdl2;
extern crate gl;
extern crate imgui_opengl_renderer;

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video = sdl_context.video().unwrap();

  {
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
  }

  let window = video.window("rust-imgui-sdl2 demo", 1000, 1000)
    .position_centered()
    .resizable()
    .opengl()
    .allow_highdpi()
    .build()
    .unwrap();

  let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
  gl::load_with(|s| video.gl_get_proc_address(s) as _);

  let mut imgui = imgui::Context::create();
  imgui.set_ini_filename(None);


  let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui);

  let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

  let mut event_pump = sdl_context.event_pump().unwrap();


  'running: loop {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    for event in event_pump.poll_iter() {
      imgui_sdl2.handle_event(&mut imgui, &event);
      if imgui_sdl2.ignore_event(&event) { continue; }

      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        _ => {}
      }
    }

    let ui = imgui_sdl2.frame(&window, &mut imgui, &event_pump.mouse_state());
    ui.show_demo_window(&mut true);


    unsafe {
      gl::ClearColor(0.2, 0.2, 0.2, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    renderer.render(ui);

    window.gl_swap_window();

    ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
  }
}
