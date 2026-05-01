
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, 
    @location(0) color: vec3<f32>
}

// Hardcoded triangle - no vertex buffer required 
var<private> POSITIONS: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 0.5), 
    vec2<f32>(-0.5, -0.5), 
    vec2<f32>(0.5, -0.5), 
);

var<private> COLORS: array<vec3<f32>, 3> = array<vec3<f32>, 3> (
    vec3<f32>(1.0, 0.0, 0.0), 
    vec3<f32>(0.0, 1.0, 0.0), 
    vec3<f32>(0.0, 0.0, 1.0), 
);

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VertexOutput {
    var out: VertexOutput; 
    out.clip_position = vec4<f32>(POSITIONS[vi], 0.0, 1.0);
    out.color = COLORS[vi];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}