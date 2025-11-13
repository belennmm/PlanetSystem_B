use nalgebra_glm::{Vec2, Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragments::Fragments;
use crate::color::Color;
use std::f32::consts::PI;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  let screen_position = uniforms.viewport_matrix * ndc_position;

  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub enum ShaderType {
  Sun,
  Earth,
  GasPlanet,
  RingPlanet,
  RockyPlanet,
  IcyPlanet,
  VolcanicPlanet,
  Moon,
  Ring,
  Ship, 
}

pub fn fragment_shader(fragment: &Fragments, uniforms: &Uniforms, current_shader: &ShaderType) -> Color {
  match current_shader {
    ShaderType::Sun => sun_shader(fragment, uniforms),
    ShaderType::Earth => earth_shader(fragment, uniforms),
    ShaderType::GasPlanet => gas_planet_shader(fragment, uniforms),
    ShaderType::RingPlanet => ring_planet_shader(fragment, uniforms),
    ShaderType::RockyPlanet => rocky_planet_shader(fragment, uniforms),
    ShaderType::IcyPlanet => icy_planet_shader(fragment, uniforms),
    ShaderType::VolcanicPlanet => volcanic_planet_shader(fragment, uniforms),
    ShaderType::Moon => moon_shader(fragment, uniforms),
    ShaderType::Ring => ring_shader(fragment, uniforms),
    ShaderType::Ship => ship_shader(fragment, uniforms)
  }
}



// Planet de las nieves 
pub fn icy_planet_shader(fragment: &Fragments, uniforms: &Uniforms) -> Color {
  let base_color = Color::new(173, 216, 230); // celestito
  let fracture_color = Color::new(255, 255, 255); //white 

  // como grietas I guess 
  let stripe_width = 0.15;
  let combined_pos = fragment.vertex_pos.x * 0.7 + fragment.vertex_pos.y * 0.3;
  let stripe_factor = ((combined_pos / stripe_width) * PI).sin().abs();

  let fracture_factor = (1.0 - stripe_factor).powf(3.0);
  let fractured_surface = base_color.lerp(&fracture_color, fracture_factor);

  // el reflejo
  let normal = fragment.normal.normalize();
  let light_dir = Vec3::new(0.0, 0.0, -1.0);
  let view_dir = -fragment.vertex_pos.normalize();
  let reflect_dir = (2.0 * dot(&light_dir, &normal) * normal - light_dir).normalize();
  let specular_intensity = dot(&reflect_dir, &view_dir).max(0.0).powf(32.0);
  let specular_color = Color::new(255, 255, 255);
  let reflected_surface = fractured_surface.lerp(&specular_color, specular_intensity * 0.5);

  // dep
  match uniforms.debug_mode {
      1 => base_color * fragment.intensity,           
      2 => fracture_color * fracture_factor,           
      3 => specular_color * specular_intensity,       
      _ => reflected_surface * fragment.intensity,  
  }   
}

// planet de fuego 
pub fn volcanic_planet_shader(fragment: &Fragments, uniforms: &Uniforms) -> Color {
  let rock_color = Color::new(50, 50, 50);    // gris
  let lava_color = Color::new(255, 100, 0);    // naranjoso

  // la lava 
  let lava_scale = 15.0;
  let noise_x = fragment.vertex_pos.x * lava_scale + uniforms.time as f32 * 0.1;
  let noise_y = fragment.vertex_pos.y * lava_scale - uniforms.time as f32 * 0.1;
  let lava_noise = ((noise_x.sin() * noise_y.cos()).abs() * 1.5).fract();
  let lava_factor = (lava_noise - 0.7).max(0.0) / 0.3;
  let surface_color = rock_color.lerp(&lava_color, lava_factor);

  // luz o brillo 
  let glow_factor = (lava_factor.powf(2.0) * 0.8).clamp(0.0, 1.0);
  let glow_color = lava_color.lerp(&Color::new(255, 255, 50), glow_factor);
  let final_color = surface_color.lerp(&glow_color, glow_factor);

  // lava intense 
  let lava_emission_factor = 0.8;
  let lava_emitted_color = lava_color * lava_emission_factor;
  let emitted_color = final_color.lerp(&lava_emitted_color, lava_factor);

  
  match uniforms.debug_mode {
      1 => rock_color * fragment.intensity,             
      2 => lava_color * lava_factor,                    
      3 => glow_color * glow_factor,                    
      _ => emitted_color * fragment.intensity,          
  }
}

// SOOLL
pub fn sun_shader(fragment: &Fragments, uniforms: &Uniforms) -> Color {
  // para un degradé
  let color1 = Color::new(255, 255, 255); // como amarillo ligth  
  let color2 = Color::new(255, 230, 28); // amarillo baby shower 
  let color3 = Color::new(255, 178, 51); // amarillo intense
  let color4 = Color::new(204, 102, 0);  // orange medio oscuro


  let x = fragment.vertex_pos.x;
  let y = fragment.vertex_pos.y;

  // Centro
  let center = (0.0, 0.0);
  let radius = ((x - center.0).powi(2) + (y - center.1).powi(2)).sqrt();

  
  let t = radius.clamp(0.0, 1.0);

  // mezcalar 
  let blended_color = if t < 0.33 {
      color1.lerp(&color2, t / 0.33)
  } else if t < 0.66 {
      color2.lerp(&color3, (t - 0.33) / 0.33)
  } else {
      color3.lerp(&color4, (t - 0.66) / 0.34)
  };


  let emission_factor = 1.5;
  let emitted_color = blended_color * emission_factor;

  
  match uniforms.debug_mode {
      1 => blended_color * fragment.intensity,                     
      2 => blended_color,                                          
      3 => Color::new(255, 255, 255) * emission_factor,     
      _ => emitted_color * fragment.intensity,                    
  }
}

// planeta  gaseoso
pub fn gas_planet_shader(fragment: &Fragments, uniforms: &Uniforms) -> Color {
  let band_color1 = Color::new(139, 69, 19);  // café más oscuro
  let band_color2 = Color::new(205, 133, 63); // cagé claro
  let band_color3 = Color::new(222, 184, 135); // girs?


  // franjas
  let band_scale = 4.0;
  let flow_speed = 0.001;
  let flow_offset = uniforms.time as f32 * flow_speed;
  let y_position = fragment.vertex_pos.y + flow_offset;
  let band_factor = ((y_position * band_scale).sin() * 0.5 + 0.5).fract();


  let band_color = if band_factor < 0.33 {
      band_color1.lerp(&band_color2, band_factor / 0.33)
  } else if band_factor < 0.66 {
      band_color2.lerp(&band_color3, (band_factor - 0.33) / 0.33)
  } else {
      band_color3.lerp(&band_color1, (band_factor - 0.66) / 0.34)
  };

  // 
  let vortex_center = Vec2::new(-0.2, -0.2);
  let vortex_radius = 0.3;
  let distance_to_vortex = ((fragment.vertex_pos.x - vortex_center.x).powi(2)
      + (fragment.vertex_pos.y - vortex_center.y).powi(2))
      .sqrt();
  let vortex_intensity = ((vortex_radius - distance_to_vortex).max(0.0f32) / vortex_radius).powf(2.0);
  let vortex_color = Color::new(255, 69, 0);
  let final_color = band_color.lerp(&vortex_color, vortex_intensity);

  match uniforms.debug_mode {
      1 => band_color * fragment.intensity,       
      2 => vortex_color * vortex_intensity,      
      _ => final_color * fragment.intensity,      
  }
}

// planet del rocoso
pub fn rocky_planet_shader(fragment: &Fragments, _uniforms: &Uniforms) -> Color {

  let base_color = Color::new(139, 69, 19);    // café rojo
  let mid_color = Color::new(205, 92, 92);     // rojo rosado
  let highlight_color = Color::new(255, 160, 122); // como rosa de señoora

  // ruido 
  let rock_scale = 10.0; 
  let detail_scale = 0.3; 


  let x = fragment.vertex_pos.x;
  let y = fragment.vertex_pos.y;
  let randomness = (x * 12.9898 + y * 78.233).sin() * 43758.5453;
  let random_factor = randomness.fract() * detail_scale;

  
  let noise = (((x + random_factor) * rock_scale).sin() * ((y + random_factor) * rock_scale).cos()).abs();

  
  let rocky_surface = if noise < 0.4 {
      base_color.lerp(&mid_color, noise / 0.4)
  } else {
      mid_color.lerp(&highlight_color, (noise - 0.4) / 0.6)
  };

  
  rocky_surface * fragment.intensity
}

// Una luna para el rocoso - try 2
pub fn moon_shader(fragment: &Fragments, _uniforms: &Uniforms) -> Color {
  // base colors
  let base_color = Color::new(169, 169, 169);    // Gris
  let mid_color = Color::new(190, 190, 190);     // Gris medio
  let highlight_color = Color::new(211, 211, 211); // Gris claro

  // textura 
  let rock_scale = 12.0; 
  let detail_scale = 0.25;
  // coor
  let x = fragment.vertex_pos.x;
  let y = fragment.vertex_pos.y;
  let randomness = (x * 15.789 + y * 41.233).sin() * 43758.5453;
  let random_factor = randomness.fract() * detail_scale;

  let noise = (((x + random_factor) * rock_scale).sin() * ((y + random_factor) * rock_scale).cos()).abs();

  // interpolación
  let rocky_surface = if noise < 0.5 {
      base_color.lerp(&mid_color, noise / 0.5)
  } else {
      mid_color.lerp(&highlight_color, (noise - 0.5) / 0.5)
  };

  //  cráteres
  let crater_positions = [
      (0.1, 0.2, 0.50), 
      (-0.3, -0.1, 0.30),
      (0.4, -0.3, 0.2), 
      (-0.1, 0.5, 0.40),
      (-0.5, -0.4, 0.25),
      (0.3, 0.4, 0.35),
      (0.1, 0.5, 0.20),
      (0.2, -0.1, 0.25),
      (0.0, -0.6, 0.28), 
      (-0.4, 0.2, 0.22),
      (0.5, 0.0, 0.30),  
      (-0.2, -0.5, 0.18), 
      (0.35, 0.5, 0.24),
      (-0.45, -0.3, 0.20),
  ];

  let crater_color = Color::new(100, 100, 100); // Gris oscuro para los cráteres

  // intensity de los crat
  let mut combined_crater_intensity = 0.0;
  for &(cx, cy, radius) in crater_positions.iter() {
      let distance = ((fragment.vertex_pos.x - cx).powi(2)
          + (fragment.vertex_pos.y - cy).powi(2))
          .sqrt();
      let crater_intensity = ((radius - distance).max(0.0f32) / radius).powf(3.0);
      combined_crater_intensity += crater_intensity;
  }

  //  intensity
  let final_surface = rocky_surface.lerp(&crater_color, combined_crater_intensity);

  // multiplu
  final_surface * fragment.intensity
}

// mov de la luna
pub fn moon_position(time: f32, radius: f32) -> Vec3 {
  let angle = time * 0.01;
  Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin())
}


// saturno 
pub fn ring_planet_shader(fragment: &Fragments, uniforms: &Uniforms) -> Color {
  let band_color1 = Color::new(189, 155, 107); // café claro
  let band_color2 = Color::new(210, 180, 140); // girs
  let band_color3 = Color::new(255, 222, 173); // blancoso

  
  let band_scale = 3.5; //
  let flow_speed = 0.0008; //
  let flow_offset = uniforms.time as f32 * flow_speed;
  let y_position = fragment.vertex_pos.y + flow_offset;
  let band_factor = ((y_position * band_scale).sin() * 0.5 + 0.5).fract();

  
  let band_color = if band_factor < 0.33 {
      band_color1.lerp(&band_color2, band_factor / 0.33)
  } else if band_factor < 0.66 {
      band_color2.lerp(&band_color3, (band_factor - 0.33) / 0.33)
  } else {
      band_color3.lerp(&band_color1, (band_factor - 0.66) / 0.34)
  };

  match uniforms.debug_mode {
      1 => band_color * fragment.intensity, 
      _ => band_color * fragment.intensity, 
  }
}

// los anillos de saturno viven
fn ring_shader(fragment: &Fragments, uniforms: &Uniforms) -> Color {
  // Colores base para el anillo
  let base_color = Color::new(255, 220, 80); // yellowstone
  let shadow_color = Color::new(150, 120, 60); // 

  let surface_color = base_color;

 
  let light_direction = Vec3::new(1.0, 1.0, 1.0).normalize(); 
  let normal = fragment.vertex_pos.normalize(); 
  let light_intensity = (normal.dot(&light_direction)).clamp(0.2, 1.0); 

  
  let final_color = match uniforms.debug_mode {
      1 => base_color * fragment.intensity,                                                 
      _ => surface_color * light_intensity + shadow_color * (1.0 - light_intensity),    
  };

  final_color
}


// movement
pub fn planet_orbit(time: f32, radius: f32, speed: f32) -> Vec3 {
  let angle = time * speed; // vel angular
  Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin())
}

// intento de nuestro planeta 
pub fn earth_shader(fragment: &Fragments, uniforms: &Uniforms) -> Color {
  let x = fragment.vertex_pos.x;
  let y = fragment.vertex_pos.y;
  let z = fragment.vertex_pos.z;

  // emm coor
  let theta = (y / 0.5).asin(); 
  let phi = z.atan2(x);        
  let u = (phi / (2.0 * PI)) + 0.5; 
  let v = (theta / PI) + 0.5;      

  let scale = 7.2;
  let noise = ((u * scale).sin() * (v * scale).cos()).abs();
  let continent_threshold = 0.55;

  let land_color = Color::new(34, 139, 34);
  let ocean_color = Color::new(0, 105, 148); 
  let base_color = if noise > continent_threshold { land_color } else { ocean_color };

  // esto es un intento de nubecitas 
  let time = uniforms.time as f32 * 0.02;
  let cloud_scale = 8.0;                 
  let cloud_intensity = ((u * cloud_scale + time).sin() * (v * cloud_scale + time).cos()).abs();
  let cloud_intensity = (cloud_intensity - 0.5).clamp(0.0, 1.0) * 0.6; 

  let cloud_color = Color::new(255, 255, 255); 

  // círculo de nubes 
  let cloud_radius = 0.8; 
  let distance_from_center = Vec2::new(u, v).norm(); 
  let is_in_atmosphere = distance_from_center < cloud_radius;

  
  let num_clouds = 4; // para que no opaque 
  let mut cloud_positions = Vec::new();

  for i in 0..num_clouds {
      let angle = (i as f32 / num_clouds as f32) * 2.0 * PI + time * 0.2; 
      let radius = 0.2 + (i as f32 * 0.05); 
      let x_pos = (angle.cos() * radius + 0.5) % 1.0; 
      let y_pos = (angle.sin() * radius + 0.5) % 1.0  ;
      cloud_positions.push(Vec2::new(x_pos, y_pos));
  }

  // draw  las nubes en círculos
  let mut cloud_color_final = Color::new(0, 0, 0); 
  for cloud_pos in cloud_positions.iter() {
      let frag_position = Vec2::new(u, v);
      let distance_to_cloud = (frag_position - *cloud_pos).norm(); 
      let cloud_radius = 0.075; 
      let is_in_cloud = distance_to_cloud < cloud_radius;

      
      if is_in_cloud {
          cloud_color_final = cloud_color_final.lerp(&cloud_color, 0.7);
      }
  }

  // final color 
  let final_color = if is_in_atmosphere {
      
      base_color * (1.0 - cloud_intensity) + cloud_color_final
  } else {
      base_color
  };

  final_color
}

pub fn ship_shader(fragment: &Fragments, _uniforms: &Uniforms) -> Color {
    let scalar = fragment.intensity;
    Color {
        r: (255.0 * scalar).clamp(51.0, 123.0) as u8,
        g: (255.0 * scalar).clamp(29.0, 70.0) as u8,
        b: (255.0 * scalar).clamp(64.0, 155.0) as u8,
    }
}

