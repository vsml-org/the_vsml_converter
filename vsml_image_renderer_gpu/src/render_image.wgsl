struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var output: VertexOutput;

    let x = f32(in_vertex_index & 1) * 4 - 1;
    let y = f32(in_vertex_index & 2) * 2 - 1;
    let u = f32(in_vertex_index & 1) * 2;
    let v = 1 - f32(in_vertex_index & 2);

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
