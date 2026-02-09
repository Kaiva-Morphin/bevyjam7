#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> size: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var uv = mesh.uv;
    var px = (uv - 0.5) * size.xy;
    px /= 7.0;
    px = floor(px);
    var c = vec3(0.0);
    let factor = 
    f32(i32(
        px.y
        + sin(globals.time * 1.5 + px.x / (3.0 - (cos(globals.time + px.x * 0.02 + px.y * 0.03) * 0.9 - 0.3))) * 5.0
        - px.x * 0.5
        + 500.0
    ) % 10) * 0.1;
    let v = 0.6;
    let x = px.x * 0.02;
    let y = px.y * 0.02;
    let mx = cos(globals.time * 0.5 + x * v + cos(y * 0.1 + x * 0.03) * 8) * 0.5 + 0.5;
    let a = vec3( 0.4, 0.2, 1.0 );
    let b = vec3( 1.0, 0.1, 0.6 );
    let accent = mix(a, b, mx);
    if factor < 0.2 {
        c = mix(accent, vec3(1.0), 0.1);
    } else if factor < 0.4 {
        c = accent * 0.2;
    } else if factor < 0.6 {
        c = accent * factor * 2.0;
    } else if factor < 0.8 {
        c = accent * factor * 0.4;
    } else {
        c = accent * 0.1;
    }
    c *= 0.1;
    return vec4(c, 1.0);
}
