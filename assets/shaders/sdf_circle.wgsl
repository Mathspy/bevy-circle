#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let remapped_uv = (mesh.uv - 0.5) * 2;
    let d = length(remapped_uv) - 1;
    let aaf = 0.71 * fwidth(d);
    let a = smoothstep(aaf, -aaf, d);

    return vec4<f32>(color.rgb, color.a * a);
}
