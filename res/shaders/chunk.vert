#version 330 core

layout (location = 0) in vec3 v_position;
layout (location = 1) in vec2 v_uv;
layout (location = 2) in float v_light;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

uniform float u_fade;

out vec2 uv;
out float light;
out float dist;
flat out float fade;

void main()
{
    uv = v_uv;
    light = v_light;
    fade = u_fade;

    vec3 chunk_pos = v_position;
    chunk_pos.y += (fade - 1.0) * 20.0;

    vec4 finalPos = u_view * u_model * vec4(chunk_pos, 1.0);

    dist = length(finalPos.xyz);

    gl_Position = u_projection * finalPos;
}