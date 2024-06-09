use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::mouse::MouseUtil;


use sdl2::video::{Window};

pub struct Winsdl {
    pub sdl : Sdl,
    pub window : Window,
    pub event_pump: EventPump,
    pub video_subsystem: VideoSubsystem,
    pub mouse : MouseUtil,
    pub width : usize,
    pub height : usize
}

impl Winsdl {
    pub fn new(width : usize, height : usize) -> Result<Self, &'static str>{
        let sdl : Sdl = sdl2::init().unwrap();
        let video_subsystem : VideoSubsystem = sdl.video().unwrap();


        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window : Window = video_subsystem
            .window("My Window", width as u32, height as u32)
            .opengl()
            .resizable()
            .build()
            .unwrap();


        let mut mouse = video_subsystem.sdl().mouse();

        mouse.set_relative_mouse_mode(true);
        mouse.warp_mouse_in_window(&window, (width / 2) as i32, (height / 2) as i32);

        let event_pump : EventPump = sdl.event_pump().unwrap();


        return Ok(Winsdl {
            sdl,
            window,
            event_pump,
            video_subsystem,
            mouse,
            width,
            height
        });
    }

    pub fn update_mouse_center(&self, x : i32, y : i32){
        self.mouse.warp_mouse_in_window(&self.window, (self.height / 2) as i32, (self.width / 2) as i32)

    }
}

