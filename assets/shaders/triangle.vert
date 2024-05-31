#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

uniform float scale;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = vec4(Position.x * scale, Position.y * scale, Position.z * scale, 1.0);
    OUT.Color = Color;
}