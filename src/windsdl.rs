use sdl2::{EventPump, Sdl, VideoSubsystem};


use sdl2::video::{Window};

pub struct Winsdl {
    pub sdl : Sdl,
    pub window : Window,
    pub event_pump: EventPump,
    pub video_subsystem: VideoSubsystem
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

        let event_pump : EventPump = sdl.event_pump().unwrap();


        return Ok(Winsdl {
            sdl,
            window,
            event_pump,
            video_subsystem
        });
    }
}

