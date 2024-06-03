#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec2 Texture;
} IN;

uniform sampler2D tex0;

out vec4 Color;

void main()
{
//    Color = vec4(IN.Color, 1.0f);
    Color = texture(tex0, IN.Texture);
}