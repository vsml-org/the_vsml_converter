struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) base_width: u32,
    @location(1) base_height: u32,
    @location(2) x: f32,
    @location(3) y: f32,
    @location(4) width: f32,
    @location(5) height: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let x = 2 * ((in.x + (2 * in.width * f32(in.vertex_index & 1))) / f32(in.base_width)) - 1;
    let y = 2 * (1 - (in.y + in.height - (in.height * f32(in.vertex_index & 2))) / f32(in.base_height)) - 1;
    let u = f32(in.vertex_index & 1) * 2;
    let v = 1 - f32(in.vertex_index & 2);

    output.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    output.tex_coords = vec2<f32>(u, v);

    return output;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
