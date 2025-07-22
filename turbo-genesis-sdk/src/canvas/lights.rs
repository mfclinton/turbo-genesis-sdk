pub fn set_light(index: u32, position: [f32; 3], intensity: f32, color: [f32; 3], radius: f32) {
    turbo_genesis_ffi::canvas::set_light(index, position[0], position[1], position[2], intensity, color[0], color[1], color[2], radius);
}

pub fn set_num_lights(count: u32) {
    turbo_genesis_ffi::canvas::set_num_lights(count);
}

pub fn set_ambient_light(color: [f32; 3]) {
    turbo_genesis_ffi::canvas::set_ambient_light(color[0], color[1], color[2]);
}
