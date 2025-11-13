use nalgebra_glm::{Vec3, Vec4};
use rand::prelude::*;
use std::f32::consts::PI;
use crate::{Framebuffer, Uniforms};

pub struct Skybox {
    stars: Vec<Star>,
}

struct Star {
    position: Vec3,
    brightness: f32,
    size: u8,
}

impl Star {
    /// estrella con propiedades aleatorias
    pub fn new(radius: f32) -> Self {
        let mut rng = rand::thread_rng();
        let theta = rng.gen::<f32>() * 2.0 * PI; 
        let phi = rng.gen::<f32>() * PI ;       

        // coord cartesianas
        let x = radius * phi.sin() * theta.cos();
        let y = radius * phi.cos() ;
        let z = radius * phi.sin() * theta.sin();

        Star {
            position: Vec3::new(x, y, z),
            // brillo
            brightness: rng.gen::<f32>(), 
            size: rng.gen_range(1..=3),  
        }
    }
}

impl Skybox {
    ///  sky con un número específico de estrellas
    pub fn new(star_count: usize, radius: f32) -> Self {
        let stars = (0..star_count)
            .map(|_| Star::new(radius))
            .collect();
        Skybox { stars }
    }

    /// sky a frame
    pub fn render_sb(&self, framebuffer: &mut Framebuffer, uniforms: &Uniforms, camera_position: Vec3) {
    #[inline]
    fn put(framebuffer: &mut Framebuffer, x: i32, y: i32, depth: f32) {
        if x >= 0 && y >= 0
            && (x as usize) < framebuffer.width
            && (y as usize) < framebuffer.height
        {
            framebuffer.point(x as usize, y as usize, depth);
        }
    }

    for star in &self.stars {
        
        let position = star.position + camera_position;

        // poroy
        let pos_vec4 = Vec4::new(position.x, position.y, position.z, 1.0);
        let projected = uniforms.projection_matrix * uniforms.view_matrix * pos_vec4;

        if projected.w <= 0.0 { continue; }
        let ndc = projected / projected.w;

        let screen_pos = uniforms.viewport_matrix * Vec4::new(ndc.x, ndc.y, ndc.z, 1.0);

        
        if screen_pos.z < 0.0 { continue; }

        
        let x = screen_pos.x.round() as i32;
        let y = screen_pos.y.round() as i32;

        // color por brillo
        let intensity = (star.brightness * 255.0).clamp(0.0, 255.0) as u8;
        let color = (intensity as u32) << 16 | (intensity as u32) << 8 | intensity as u32;
        framebuffer.set_current_color(color);


        match star.size {
            1 => put(framebuffer, x, y, 1000.0),
            2 => {
                put(framebuffer, x,     y,     1000.0);
                put(framebuffer, x + 1, y,     1000.0);
                put(framebuffer, x,     y + 1, 1000.0);
                put(framebuffer, x + 1, y + 1, 1000.0);
            }
            3 => {
                put(framebuffer, x,     y,     1000.0);
                put(framebuffer, x - 1, y,     1000.0);
                put(framebuffer, x + 1, y,     1000.0);
                put(framebuffer, x,     y - 1, 1000.0);
                put(framebuffer, x,     y + 1, 1000.0);
            }
            _ => {}
        }
    }
}

}
