use crate::kapp::*;
use crate::kmath::*;
use crate::texture_buffer::TextureBuffer;
use crossbeam_channel::*;

const ITER_MAX_MAX: i32 = 6400;
const ITER_INITIAL: i32 = 1600;

pub struct Job {
    x_px: i32,
    y_px: i32,
    x: f64,
    y: f64,
    max_iters: i32,
    gen: u32,
}

pub struct Result {
    iters: i32,
    x_px: i32,
    y_px: i32,
    gen: u32,
    div: bool,
}

impl Default for GUI {
    fn default() -> Self {
        let n_workers = 6;
        
        let (job_sender, job_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();

        for _ in 0..n_workers {
            let job_receiver: Receiver<Job> =  job_receiver.clone();
            let result_sender: Sender<Result> = result_sender.clone();
            std::thread::spawn(move || {
                loop {
                    let job = job_receiver.recv().unwrap();

                    let c = Vec2::new(job.x, job.y);
                    let mut z = Vec2::zero();

                    let mut iters = 0;
                    let mut div = false;
                    
                    while iters < job.max_iters {
                        if z.mag2() > 100.0 {
                            div = true;
                            break;
                        }
                        // fracting x makes eyelashes
                        // z = Vec2::new(z.x.fract().abs(), z.y.fract().abs());
                        // z = z.complex_mul(z) + c;
                        // this is the sick triangle one, its really cool
                        // z^2 / (z + 0.01i) (z-0.01i) + c
                        // z = z.complex_mul(z).complex_div(z.plus(Vec2::new(0.00, 0.01)).complex_mul(z.plus(Vec2::new(0.00, -0.01)))) + c;

                        // related
                        // z = z.complex_mul(z).complex_div(z.plus(Vec2::new(0.00, PHI)).complex_mul(z.plus(Vec2::new(0.00, -PHI)))) + c;
                        // original
                        //z = z.complex_mul(z).complex_div(z.plus(Vec2::new(0.00, ROOT2INV)).complex_mul(z.plus(Vec2::new(0.00, -ROOT2INV)))) + c;
                        // z = z.complex_mul(z).complex_div(z.plus(Vec2::new(0.00, PI)).complex_mul(z.plus(Vec2::new(0.05, 0.5))).complex_mul(z.plus(Vec2::new(0.05, ROOT2)))) + c;

                        // hyper giga laser
                        // z^2 / (z+0.01)^2  + c
                        // z = z.complex_mul(z).complex_div(z.plus(Vec2::new(0.01, 0.00)).complex_mul(z.plus(Vec2::new(0.01, 0.00)))) + c;

                        // good found the singularity one, I shall sleep soundly tonight
                        // z = z.complex_mul(z).complex_div(z.plus(Vec2::new(0.00, 0.01)).complex_mul(z.plus(Vec2::new(0.00, -0.02)))) + c;
                        
                        // lanterns
                        z = z.complex_mul(z).complex_div(z.plus(Vec2::new(0.00, 0.01))) + c;

                        //z = z.complex_mul(z).plus(z).complex_mul(z).plus(z).complex_mul(z).plus(z).complex_mul(z).plus(z).complex_mul(z).plus(z) + c;


                        // infinite pascals triangle
                        // let zp = z + Vec2::new(1.0, 0.0);
                        // let zp = zp.complex_mul(zp);
                        // let zp = zp.complex_mul(zp);
                        // let zp = zp.complex_mul(zp);
                        // let zp = zp.complex_mul(zp);
                        // let zp = zp.complex_mul(zp);
                        // let zp = zp.complex_mul(zp);
                        // let zp = zp.complex_mul(z);
                        // z = zp + c;

                        // oh fuck its the shiftlebrot set
                        // let t = 0.1;
                        // z = z.minus(Vec2::new(t, 0.0)).complex_mul(z.plus(Vec2::new(t, 0.0))) + c;
                        // z = z.plus(Vec2::new(0.0, t)).complex_mul(z.plus(Vec2::new(0.0, t))) + c;

                        // let o = Vec2::new(t, 0.0);
                        // z = z.complex_mul(z).plus(o).complex_div(z.minus(o)) + c;

                        // miniverse one
                        // z = z.complex_div(
                        //     z.plus(Vec2::new(0.0, 0.5)).complex_mul(
                        //         z.plus(Vec2::new(0.0, -0.5)).complex_mul(
                        //             z.plus(Vec2::new(-2.0, 0.0))
                        //         )
                        //     )
                        // ) + c;

                        // z = c.complex_mul(z.plus(Vec2::new(0.0, 1.0))).complex_mul(c).plus(c).complex_div(z.plus(Vec2::new(0.33, 0.33)).complex_mul(z.plus(Vec2::new(0.0, -2.0)))) + c;

                        // z = z.plus(c).complex_mul(z.plus(c));
                        // z = z.complex_mul(z).plus(c);

                        // z = Vec2::new(z.x.abs(), z.y.abs());
                        // z = z.complex_mul(z) + c;

                        // z = z.complex_div(
                        //     z.plus(Vec2::new(1.0, 0.5)).complex_mul(
                        //         z.plus(Vec2::new(0.0, -0.5)).complex_mul(
                        //             z.plus(Vec2::new(-2.0, 0.0))
                        //         )
                        //     )
                        // ) + c;

                        iters += 1;
                    }
                    result_sender.send(Result {
                        x_px: job.x_px,
                        y_px: job.y_px,
                        iters: iters,
                        gen: job.gen,
                        div,
                    }).unwrap();
                }
            });
        }

        let mut palette = [Vec4::new(0.0, 0.0, 0.0, 0.0); ITER_MAX_MAX as usize];
        let mut theta = 0.0;
        let mut theta_mul = 2.0;
        for i in 0..ITER_MAX_MAX {
            palette[i as usize] = Vec4::new(theta, 1.0, 1.0, 1.0).hsv_to_rgb();
            theta_mul *= 0.999;
            theta += theta_mul;
        }
        /*
        let mut period = 16;
        let mut pc = 0;
        let mut j = 0;
        for i in 0..ITER_MAX_MAX {
            let colour_start = Vec4::new(137.5 * j as f64, 1.0, 1.0, 1.0).hsv_to_rgb();
            let colour_end = Vec4::new(137.5 * (j+1) as f64, 1.0, 1.0, 1.0).hsv_to_rgb();
            palette[i as usize] = colour_start.lerp(colour_end, pc as f64 / period as f64);
            pc += 1;
            if pc == period {
                period *= 2;
                pc = 0;
                j += 1;
            }
        }
        */

        GUI {
            sender: job_sender,
            receiver: result_receiver,
            job_receiver,
            iter_buf: Vec::new(),
            div: Vec::new(),
            gen: 0,
            view_center: Vec2::zero(),
            view_h: 4.0,
            stale: true,
            palette,
            path_c: None,
        }
    }
}

pub struct GUI {
    sender: Sender<Job>,
    job_receiver: Receiver<Job>,
    receiver: Receiver<Result>,
    iter_buf: Vec<i32>,
    div: Vec<bool>,
    gen: u32,
    view_center: Vec2,
    view_h: f64,
    stale: bool,
    palette: [Vec4; ITER_MAX_MAX as usize],

    path_c: Option<Vec2>,
}



impl GUI {
    pub fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {
        let a = inputs.xres as f64 / inputs.yres as f64;
        let r = self.view_center.rect_centered(self.view_h * inputs.screen_rect.aspect(), self.view_h);

        let mut any_zoom = false;
        if inputs.key_held(VirtualKeyCode::LShift) && inputs.lmb == KeyStatus::JustPressed {
            any_zoom = true;
            let r = self.view_center.rect_centered(a * self.view_h, self.view_h);
            let rp = inputs.mouse_pos.transform(inputs.screen_rect, r);
            self.view_center = rp;
            self.view_h *= 0.5;
        }
        if inputs.key_held(VirtualKeyCode::LControl) && inputs.lmb == KeyStatus::JustPressed {
            any_zoom = true;
            let r = self.view_center.rect_centered(a * self.view_h, self.view_h);
            let rp = inputs.mouse_pos.transform(inputs.screen_rect, r);
            self.view_center = rp;
            self.view_h *= 2.0;
        } else if (inputs.lmb == KeyStatus::Pressed && !inputs.key_held(VirtualKeyCode::LShift) && !inputs.key_held(VirtualKeyCode::LControl)) {
            self.path_c = Some(inputs.mouse_pos.transform(inputs.screen_rect, r));
        }

        
        if let Some(path_c) = self.path_c {
            let mut zold = Vec2::new(0.0, 0.0);
            let mut z = Vec2::new(0.0, 0.0);
            for _ in 0..1000 {

                z = Vec2::new(z.x.abs(), z.y.abs());
                z = z.complex_mul(z) + path_c;

                let start = zold.transform(r, inputs.screen_rect);
                let end = z.transform(r, inputs.screen_rect);
    
    
                outputs.canvas.put_line(start, end, 0.002, 2.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
                zold = z;
            }
        }

        // handle zoom or res switch
        if any_zoom || self.iter_buf.len() as i32 != inputs.xres * inputs.yres {
            // drain the queue
            while let Ok(_) = self.job_receiver.try_recv() {};

            self.iter_buf = vec![0; (inputs.xres * inputs.yres) as usize];
            self.div = vec![false; (inputs.xres * inputs.yres) as usize];
            self.gen += 1;

            // issue initial jobs
            for i in 0..self.iter_buf.len() {
                if self.iter_buf[i] < ITER_MAX_MAX && self.div[i] == false {
                    let x_px = i as i32 % inputs.xres;
                    let y_px = i as i32 / inputs.xres;
                    let r = self.view_center.rect_centered(a * self.view_h, self.view_h);
                    let x = r.left() as f64 + (x_px as f64 + 0.5) * r.w as f64 / inputs.xres as f64;
                    let y = -r.bot() as f64 + (y_px as f64 + 0.5) * r.h as f64 / inputs.yres as f64;
                    let y = -y;
                    self.sender.send(Job {
                        x_px,
                        y_px,
                        x,
                        y,
                        gen: self.gen,
                        max_iters: ITER_INITIAL,
                    }).unwrap();
                }
            }
        }

        // receive results
        while let Some(res) = self.receiver.try_recv().ok() {
            if res.gen == self.gen {
                self.stale = true;
                let idx = (res.y_px * inputs.xres + res.x_px) as usize;
                self.iter_buf[idx] = res.iters;
                self.div[idx] = res.div;
                if !res.div && res.iters < ITER_MAX_MAX {
                    let x_px = res.x_px;
                    let y_px = res.y_px;
                    let r = self.view_center.rect_centered(a * self.view_h, self.view_h);
                    let x = r.left() as f64 + (x_px as f64 + 0.5) * r.w as f64 / inputs.xres as f64;
                    let y = -r.bot() as f64 + (y_px as f64 + 0.5) * r.h as f64 / inputs.yres as f64;
                    let y = -y;
                    self.sender.send(Job {
                        x_px,
                        y_px,
                        x,
                        y,
                        gen: self.gen,
                        max_iters: res.iters * 2,
                    }).unwrap();
                }
            }
        }

        // update texture
        if self.stale {
            let mut tb = TextureBuffer::new(inputs.xres as usize, inputs.yres as usize);
            for i in 0..inputs.xres {
                for j in 0..inputs.yres {
                    let it = self.iter_buf[(j * inputs.xres + i) as usize];
                    let div = self.div[(j * inputs.xres + i) as usize];

                    if div {
                        tb.set(i, j, self.palette[it as usize]);
                    } else {
                        tb.set(i, j, Vec4::new(0.0, 0.0, 0.0, 1.0));
                    }
                }
            }

            outputs.set_texture = vec![(tb, 0)];
            self.stale = false;
        }

        // draw
        outputs.draw_texture = vec![(inputs.screen_rect, 0)];
    }
}