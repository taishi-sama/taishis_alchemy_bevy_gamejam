//! Draws a progress bar with properties defined in CustomUiMaterial
import bevy_ui_render::ui_vertex_output::UiVertexOutput;


@group(1) @binding(0) var<uniform> color: vec4<f32>;
@group(1) @binding(1) var<uniform> point1: vec2<f32>;
@group(1) @binding(2) var<uniform> point2: vec2<f32>;
@group(1) @binding(3) var<uniform> point3: vec2<f32>;
@group(1) @binding(4) var<uniform> point4: vec2<f32>;
@group(1) @binding(5) var<uniform> width: f32;

fn sd_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    return length(pa - ba * h);
}

fn bezier(p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, t: f32) -> vec2<f32> {
    let u = 1.0 - t;
    return u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3;
}

fn sd_cubic(p: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>) -> f32 {
    var distance = 1e20;
    var previous = p0;

    // A small fixed number of segments is plenty for UI links and avoids
    // a costly analytic cubic-distance solve.
    for (var i = 1u; i <= 16u; i += 1u) {
        let current = bezier(p0, p1, p2, p3, f32(i) / 16.0);
        distance = min(distance, sd_segment(p, previous, current));
        previous = current;
    }

    return distance;
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // half size of the UI node
    let half_size = 0.5 * in.size;

    // position relative to the center of the UI node
    let p = in.uv * in.size - half_size;

    let half_width = 0.5 * width;

    // Cheap rejection for the large transparent parts of the full-size node.
    let low = min(min(point1, point4), min(point2, point3)) - vec2<f32>(half_width + 2.0);
    let high = max(max(point1, point4), max(point2, point3)) + vec2<f32>(half_width + 2.0);
    if (any(p < low) || any(p > high)) {
        return vec4<f32>(0.0);
    }

    let distance = sd_cubic(p, point1, point2, point3, point4) - half_width;
    let alpha = clamp(0.5 - distance, 0.0, 1.0);
    return vec4<f32>(color.rgb, color.a * alpha);
    return color;
}
