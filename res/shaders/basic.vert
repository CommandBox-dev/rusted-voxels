#version 330 core

layout (location = 0) in vec3 v_position;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

out vec3 color;

void main()
{
    //color = v_position * 0.5 + 0.5;
    color = mod(v_position, 1.2);

    gl_Position =
        u_projection *
        u_view *
        u_model *
        vec4(v_position, 1.0);
}