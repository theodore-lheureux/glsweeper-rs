#version 430 core

out vec4 FragColor;

in vec2 TexCoords;
in float Pos;

uniform sampler2D atlasTexture;

layout(std430, binding = 2) buffer TextureSSBO {
    float textureData[];
};

void main() 
{   
    vec2 texCoords = TexCoords;
    texCoords.x = texCoords.x / 4.0 + textureData[int(Pos)];
    texCoords.y = texCoords.y / 4.0 + textureData[int(Pos)];

    vec4 color = texture(atlasTexture, texCoords);

    FragColor = color;
}