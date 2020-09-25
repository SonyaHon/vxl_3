#version 330 core

in vec2 uv;
out vec4 Color;

void main()
{
    Color = vec4(uv.x, uv.y, 1.0, 1.0);
}