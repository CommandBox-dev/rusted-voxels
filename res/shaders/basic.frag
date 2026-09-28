#version 330 core

uniform sampler2D baseTexture;

in vec2 uv;
in float light;

out vec4 FragColor;

void main()
{
    vec4 tex = texture(baseTexture, uv);
    tex.rgb *= light;
    FragColor = vec4(tex);
}