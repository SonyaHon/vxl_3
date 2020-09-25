#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 UVs;

uniform mat4 trans_mat;
uniform mat4 proj_mat;
uniform mat4 view_mat;

out vec2 uv;

void main()
{
    gl_Position = proj_mat * view_mat * trans_mat * vec4(Position, 1.0);
    uv = UVs;
}