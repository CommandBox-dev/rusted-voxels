#version 330 core

layout (location = 0) in vec2 v_position;
layout (location = 1) in vec2 v_uv;

uniform mat4 u_projection;

out vec2 uv;

void main()
{
    uv = v_uv;
    gl_Position = u_projection * vec4(v_position, 0.0, 1.0);
}