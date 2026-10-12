#version 330 core

uniform sampler2D u_texture;

in vec2 uv;

out vec4 FragColor;

void main()
{
    vec4 tex = texture(u_texture, uv);
    if (tex.a <= 0.1) discard;

    FragColor = tex;
    //FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}