#version 330 core

out vec4 FragColor;

in vec2 TexCoords;

uniform sampler2D texture0;

void main() 
{
    FragColor = texture2D(texture0, TexCoords);
    // FragColor = vec4(1.0, 1.0, 1.0, 1.0);

    // set color from texture coordinates
    // FragColor = vec4(TexCoords.x, TexCoords.y, 1.0, 1.0);
}