#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec2 Texture;


uniform mat4 camMatrix;

out VS_OUTPUT {
    vec3 Color;
    vec2 Texture;
} OUT;

void main()
{
    gl_Position = camMatrix * vec4(Position.x, Position.y, Position.z, 1.0);
    OUT.Color = Color;
    OUT.Texture = Texture;
}