#version 330 core

uniform sampler2D baseTexture;

in vec2 uv;

out vec4 FragColor;

void main()
{
    vec4 tex = texture(baseTexture, uv);
    if (tex.a <= 0.1) discard;
    FragColor = tex;
}