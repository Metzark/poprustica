struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
};

struct SpriteUniform {
    position: vec2<f32>,
    scale: vec2<f32>,
    rotation: f32,
    // padding is handled automatically in wgsl
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(1) @binding(0)  // New binding group for transform
var<uniform> sprite: SpriteUniform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Apply scale
    let scaled = input.position.xy * sprite.scale;
    
    // Apply rotation
    let angle = sprite.rotation;
    let rotated = vec2<f32>(
        scaled.x * cos(angle) - scaled.y * sin(angle),
        scaled.x * sin(angle) + scaled.y * cos(angle)
    );
    
    // Apply translation
    let final_pos = rotated + sprite.position;
    
    output.position = vec4<f32>(final_pos, input.position.z, 1.0);
    output.tex_coords = input.tex_coords;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, input.tex_coords);
}