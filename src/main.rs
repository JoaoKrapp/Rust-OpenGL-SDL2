use sdl2::event::Event;
use crate::winsdl::Winsdl;

mod winsdl;
mod program;
mod windsdl;
mod program;


const WIDTH : usize = 800;
const HEIGHT : usize = 600;


fn main() {

    // Sdl window
    let mut windsdl = Winsdl::new(WIDTH, HEIGHT).unwrap();

    // OpenGL context load
    let _gl_context = windsdl.window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| windsdl.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);



    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.3,0.3,0.5, 1.0);
    }

    'running : loop {

        for event in windsdl.event_pump.poll_iter() {

            match event {
                Event::Quit { .. } => break 'running,
                _ => { }
            }

        }

        // Game loop
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        windsdl.window.gl_swap_window();

    }
}
