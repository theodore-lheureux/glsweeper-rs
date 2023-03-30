#version 430 core

out vec4 FragColor;

in vec2 TexCoords;
in float Pos;

uniform sampler2D atlasTexture;

layout(std430, binding = 0) buffer TextureSSBO {
    float textureData[];
};

void main() 
{   
    float offsetX = textureData[int(Pos)] / 4.0;
    float offsetY = floor(textureData[int(Pos)] / 4.0) / 4.0;
    vec2 texCoords = TexCoords;

    texCoords.x = texCoords.x / 4.0 + offsetX;
    texCoords.y = texCoords.y / 4.0 + offsetY;

    vec4 color = texture(atlasTexture, texCoords);

    FragColor = color;
}