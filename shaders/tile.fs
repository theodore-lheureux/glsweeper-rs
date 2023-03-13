#version 330 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D tileTexture;

void main() 
{
    FragColor = texture(tileTexture, TexCoords);
}