#version 330 core

layout (location = 0) in vec3 v_position;
layout (location = 1) in vec2 v_uv;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

out vec2 uv;

void main()
{
    uv = v_uv;

    gl_Position =
        u_projection *
        u_view *
        u_model *
        vec4(v_position, 1.0);
}