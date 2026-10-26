#version 330 core

uniform sampler2D u_texture;

in vec2 uv;
in float light;
in float dist;
flat in float fade;

out vec4 FragColor;

const float fog_start = 8.0 * 16.0; // 3.0 *
const float fog_end = 100.0 * 16.0; // 10.0 * // 20.0
const vec3 fog_color = vec3(0.8, 0.8, 0.8);

void main()
{
    vec4 tex = texture(u_texture, uv);
    tex.rgb *= light;
    tex.rgb -= (fade - 1.0) * 0.5;

    // fog
    float fog_factor = (fog_end - dist) / (fog_end - fog_start);
    fog_factor = clamp(fog_factor, 0.0, 1.0);

    FragColor = vec4(mix(fog_color, tex.rgb, fog_factor), 1.0);
}