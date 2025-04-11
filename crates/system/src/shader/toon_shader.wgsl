#import bevy_pbr::forward_io::VertexOutput

struct ToonMaterial {
    base_color: vec4<f32>,
    light_direction: vec3<f32>,
    light_color: vec4<f32>,
    camera_position: vec3<f32>,
    ambient_color: vec4<f32>,
    rim_amount: f32,
    rim_color: vec4<f32>,
    rim_threshold: f32,
    band_count: u32
};

@group(2) @binding(0) var<uniform> material: ToonMaterial;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var sample: sampler;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {

    #ifdef VERTEX_UVS_A
        let uv = input.uv;
    #else
        let uv = vec2(1.0, 1.0);
    #endif

    let base_color = material.base_color * textureSample(texture, sample, uv);

    // shading the object
    let normal = normalize(input.world_normal); // make the world_normal of the input mesh have a length of one
    let n_dot_l = dot(material.light_direction, normal) ;
    var light_intensity = 0.0;

    // if we want bands, create bands, otherwise use smooth lighting
    if material.band_count > 0 {
        if n_dot_l > 0.0 {
            var x = round(n_dot_l * f32(material.band_count));
            light_intensity = x / f32(material.band_count);
        } else {
            light_intensity = 0.0;
        }
    } else {
        light_intensity = smoothstep(0.0, 0.01, n_dot_l);
    }


    let light = (light_intensity * material.light_color);

    // specular
    let view_direction = normalize(material.camera_position - input.world_position.xyz);
    let half_vector = normalize(material.light_direction + view_direction);
    let n_dot_h = dot(normal, half_vector);
    let spec_color = vec4<f32>(0.9, 0.9, 0.9, 1.0);
    let glossiness = 32.0;
    let spec_intensity = pow(n_dot_h * light_intensity, glossiness * glossiness);
    let spec_intensity_smooth = smoothstep(0.005, 0.01, spec_intensity);
    let specular = spec_intensity_smooth * spec_color;

    // rim lighting
    let rim_dot = 1 - dot(view_direction, normal);
    var rim_intensity = rim_dot * pow(n_dot_l, material.rim_threshold);
    rim_intensity = smoothstep(material.rim_amount - 0.01, material.rim_amount + 0.01, rim_intensity);
    let rim = rim_intensity * material.rim_color;

    return base_color * (material.ambient_color + light + specular + rim);
}