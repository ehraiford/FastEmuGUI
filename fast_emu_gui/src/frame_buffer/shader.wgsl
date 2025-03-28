@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 6>(
        vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0),
        vec2(-1.0, 1.0), vec2(1.0, -1.0), vec2(1.0, 1.0)
    );
    return vec4(positions[vertex_index], 0.0, 1.0);
}

@group(0) @binding(0) var texture_sampler: sampler;
@group(0) @binding(1) var texture_data: texture_2d<f32>;

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_coord.xy / vec2<f32>(160.0, 144.0);
    return textureSample(texture_data, texture_sampler, uv);
}
