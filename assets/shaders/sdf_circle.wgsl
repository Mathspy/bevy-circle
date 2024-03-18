#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> radius: f32;
@group(2) @binding(1) var<uniform> color: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let d = length(mesh.uv - 0.5) * radius * 2 - radius;
    let aaf = 0.71 * fwidth(d);
    let a = smoothstep(aaf, -aaf, d);

    return vec4<f32>(color.rgb, color.a * a);
}
