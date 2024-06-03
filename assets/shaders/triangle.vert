#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec2 Texture;


uniform float scale;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

out VS_OUTPUT {
    vec3 Color;
    vec2 Texture;
} OUT;

void main()
{
    gl_Position = proj * view * model * vec4(Position.x * scale, Position.y * scale, Position.z * scale, 1.0);
    OUT.Color = Color;
    OUT.Texture = Texture;
}