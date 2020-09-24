#version 330 core

layout (location = 0) in vec3 Position;

uniform mat4 trans_mat;

void main()
{
    gl_Position = trans_mat * vec4(Position, 1.0);
}