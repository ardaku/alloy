#version 140
uniform sampler2D tex;
uniform vec2 texture_size;
uniform float brighten;
uniform vec3 shadow_color;
uniform vec4 bg_color;
uniform float shadow_offset;
in vec2 v_tex_coords;
out vec4 f_color;

void main() {
    vec4 color = texture(tex, v_tex_coords);
    color = vec4(mix(color.rgb, vec3(1.0), max(0.0, brighten)), color.a);
    color = vec4(mix(color.rgb, vec3(0.0), -min(0.0, brighten)), color.a);
    color.rgb *= color.a;

    const float shadow_size = 8.0;
    float shadow_pixel_offset = shadow_size * shadow_offset;
    vec2 tex_cood_from_edge = vec2(0.5) - abs(v_tex_coords - vec2(0.5));
    vec2 shadow_along_axes = 
        max(vec2(0.0), vec2(1.0) - (tex_cood_from_edge * texture_size + shadow_pixel_offset) / shadow_size);

    color = mix(bg_color, color, color.a);
    float shadow = shadow_along_axes.x + shadow_along_axes.y;
    f_color = vec4(mix(color.rgb, shadow_color, min(1.0, 4.0*shadow)), mix(color.a, 1.0, shadow));
}
