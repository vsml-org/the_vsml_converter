struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) base_width: u32,
    @location(1) base_height: u32,
    @location(2) x: f32,
    @location(3) y: f32,
    @location(4) width: f32,
    @location(5) height: f32,
    @location(6) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let x = 2.0 * ((in.x + (2.0 * in.width * f32(in.vertex_index & 1))) / f32(in.base_width)) - 1.0;
    let y = 2.0 * (1.0 - (in.y + in.height - (in.height * f32(in.vertex_index & 2))) / f32(in.base_height)) - 1.0;

    output.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    output.color = in.color;

    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
