#version 330 core

uniform sampler2D baseTexture;

in vec2 uv;

out vec4 FragColor;

void main()
{
    vec4 tex = texture(baseTexture, uv);
    FragColor = vec4(tex);
}