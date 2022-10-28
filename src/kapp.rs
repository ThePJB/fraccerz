use crate::gui::*;

use crate::renderers::font_rendering::*;
use crate::renderers::simple_renderer::*;
use crate::texture_buffer::*;
use crate::kmath::*;
use crate::video::*;

use std::collections::HashSet;
use std::time::{SystemTime, Instant, Duration};

pub use glutin::event::VirtualKeyCode;
use glutin::event::ElementState;
use glutin::event::WindowEvent;
use glutin::event::WindowEvent::*;
use glutin::event::Event;
use glutin::event_loop::*;

use glow::HasContext;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeyStatus {
    Pressed,
    JustPressed,
    JustReleased,
    Released,
}


#[derive(Clone)]
pub struct FrameInputState {
    pub screen_rect: Rect,
    pub xres: i32,
    pub yres: i32,
    pub mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    
    pub prev_keys: HashSet<VirtualKeyCode>,
    pub curr_keys: HashSet<VirtualKeyCode>,
    pub repeat_keys: HashSet<VirtualKeyCode>,

    pub lmb: KeyStatus,
    pub rmb: KeyStatus,
    pub mmb: KeyStatus,
    pub scroll_delta: f64,
    pub t: f64,
    pub dt: f64,
    pub frame: u32,
    pub seed: u32,
}

impl FrameInputState {
    pub fn key_held(&self, keycode: VirtualKeyCode) -> bool {
        self.curr_keys.contains(&keycode)
    }
    pub fn key_rising(&self, keycode: VirtualKeyCode) -> bool {
        self.curr_keys.contains(&keycode) && !self.prev_keys.contains(&keycode)
    }
    pub fn key_press_or_repeat(&self, keycode: VirtualKeyCode) -> bool {
        (self.curr_keys.contains(&keycode) && !self.prev_keys.contains(&keycode)) || self.repeat_keys.contains(&keycode)
    }
    pub fn key_falling(&self, keycode: VirtualKeyCode) -> bool {
        !self.curr_keys.contains(&keycode) && self.prev_keys.contains(&keycode)
    }
    pub fn new(a: f64) -> FrameInputState {
        FrameInputState { 
            screen_rect: Rect::new(0.0, 0.0, a, 1.0, ),
            xres: 0,
            yres: 0,
            
            mouse_pos: Vec2::new(0.0, 0.0), 
            mouse_delta: Vec2::new(0.0, 0.0), 
            scroll_delta: 0.0,
            curr_keys: HashSet::new(),
            prev_keys: HashSet::new(),
            repeat_keys: HashSet::new(),
            lmb: KeyStatus::Released, 
            rmb: KeyStatus::Released, 
            mmb: KeyStatus::Released, 
            t: 0.0,
            dt: 0.0,
            frame: 0,
            seed: SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or(Duration::from_nanos(34123123)).subsec_nanos(),
        }
    }
}

pub struct FrameOutputs {
    pub canvas: SimpleCanvas,
    pub set_texture: Vec<(TextureBuffer, usize)>,
    pub draw_texture: Vec<(Rect, usize)>,
    pub glyphs: GlyphBuffer,
}

impl FrameOutputs {
    pub fn new(a: f64) -> FrameOutputs {
        FrameOutputs {
            glyphs: GlyphBuffer::new(),
            canvas: SimpleCanvas::new(a),
            set_texture: Vec::new(),
            draw_texture: Vec::new(),
        }
    }
}

pub struct Application {
    video: Video,
    root_scene: GUI,

    t_last: Instant,
    instant_mouse_pos: Vec2,
    current: FrameInputState,
}

impl Application {
    pub fn new(event_loop: &EventLoop<()>) -> Application {
        let xres = 800;
        let yres = 800;
    
        let video = Video::new("FRACCERZ", xres as f64, yres as f64, event_loop);
        
        Application {
            video,
            root_scene: GUI::default(),
            t_last: Instant::now(),
            instant_mouse_pos: Vec2::zero(),
            current: FrameInputState::new(xres as f64 / yres as f64),                        
        }
    }

    pub fn handle_event(&mut self, event: Event<()>) {
        match event {
            Event::LoopDestroyed => self.exit(),
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => self.exit(),
            Event::WindowEvent {event, ..} => match event {
                KeyboardInput { 
                    input: glutin::event::KeyboardInput { 
                        virtual_keycode: Some(virtual_code), 
                        state, 
                    ..},
                ..} => {
                    if state == ElementState::Pressed {
                        if self.current.curr_keys.contains(&virtual_code) {
                            self.current.repeat_keys.insert(virtual_code);
                        } else {
                            self.current.curr_keys.insert(virtual_code);
                        }
                    } else {
                        self.current.curr_keys.remove(&virtual_code);
                    }
                },
                MouseInput { button: glutin::event::MouseButton::Left, state, ..} => {
                    if state == ElementState::Pressed {
                        self.current.lmb = KeyStatus::JustPressed;
                    } else {
                        self.current.lmb = KeyStatus::JustReleased;
                    }
                },
                MouseInput { button: glutin::event::MouseButton::Middle, state, ..} => {
                    if state == ElementState::Pressed {
                        self.current.mmb = KeyStatus::JustPressed;
                    } else {
                        self.current.mmb = KeyStatus::JustReleased;
                    }
                },
                MouseInput { button: glutin::event::MouseButton::Right, state, ..} => {
                    if state == ElementState::Pressed {
                        self.current.rmb = KeyStatus::JustPressed;
                    } else {
                        self.current.rmb = KeyStatus::JustReleased;
                    }
                },

                // Scroll
                glutin::event::WindowEvent::MouseWheel { delta, ..} => {
                    match delta {
                        glutin::event::MouseScrollDelta::LineDelta(_, y) => {
                            self.current.scroll_delta = y as f64;
                        },
                        glutin::event::MouseScrollDelta::PixelDelta(p) => {
                            self.current.scroll_delta = p.y;
                        },
                    }
                },


                // Mouse motion
                CursorMoved {
                    position: pos,
                    ..
                } => {
                    self.instant_mouse_pos = Vec2::new(pos.x as f64 / self.video.yres, pos.y as f64 / self.video.yres);
                },

                // Resize
                Resized(physical_size) => {
                    self.video.window.resize(physical_size);
                    self.video.xres = physical_size.width as f64;
                    self.video.yres = physical_size.height as f64;
                    self.current.xres = physical_size.width as i32;
                    self.current.yres = physical_size.height as i32;
                    unsafe {self.video.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32)};
                    self.current.screen_rect = Rect::new(0.0, 0.0, self.video.xres / self.video.yres, 1.0);
                },
                _ => {},
            },
            Event::MainEventsCleared => {
                let t_now = Instant::now();
                let dt = t_now.duration_since(self.t_last).as_secs_f64();
                self.current.dt = dt;
                self.current.t += dt;
                self.t_last = t_now;
                self.current.frame += 1;
                self.current.mouse_delta = self.instant_mouse_pos - self.current.mouse_pos;
                self.current.mouse_pos = self.instant_mouse_pos;
                let state = self.current.clone();
                self.current.prev_keys = self.current.curr_keys.clone();
                self.current.repeat_keys = HashSet::new();
                self.current.seed = khash(self.current.seed * 196513497);
                self.current.scroll_delta = 0.0;
                self.current.lmb = match self.current.lmb {KeyStatus::JustPressed | KeyStatus::Pressed => KeyStatus::Pressed, KeyStatus::JustReleased | KeyStatus::Released => KeyStatus::Released};
                self.current.mmb = match self.current.mmb {KeyStatus::JustPressed | KeyStatus::Pressed => KeyStatus::Pressed, KeyStatus::JustReleased | KeyStatus::Released => KeyStatus::Released};
                self.current.rmb = match self.current.rmb {KeyStatus::JustPressed | KeyStatus::Pressed => KeyStatus::Pressed, KeyStatus::JustReleased | KeyStatus::Released => KeyStatus::Released};

                let mut new_outputs = FrameOutputs::new(state.screen_rect.aspect());
                self.root_scene.frame(&state, &mut new_outputs);
                self.video.render(&new_outputs, state.screen_rect.aspect());
            },
            _ => {},
        }
    }

    pub fn exit(&mut self) {
        println!("exiting");
        std::process::exit(0);
    }
}