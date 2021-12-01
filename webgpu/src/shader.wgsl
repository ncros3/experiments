// Vertex shader

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
};

struct VertexOutputColor {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] position: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

[[stage(vertex)]]
fn vs_main_color(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutputColor {
    var out: VertexOutputColor;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.position = vec2<f32>(x, y);
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var out: vec4<f32>;
    let color_1 = vec4<f32>(0.3, 0.2, 0.1, 1.0);
    let color_2 = vec4<f32>(0.1, 0.2, 0.5, 1.0);
    if ((in.clip_position[0] > 400.0) && (in.clip_position[1] > 300.0)) {
        out = color_1;
    } else {
        out = color_2;
    }
    return out;
}

[[stage(fragment)]]
fn fs_main_color(in: VertexOutputColor) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.position, 0.1, 1.0);
}

