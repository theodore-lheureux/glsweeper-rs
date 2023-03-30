#version 430 core

layout (location = 0) in vec2 aCoords;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in float aPos;

out vec2 TexCoords;
out float Pos;

void main()
{
    gl_Position = vec4(aCoords, 0.0, 1.0);
    TexCoords = aTexCoord;
    Pos = aPos;
}