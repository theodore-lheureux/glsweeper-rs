#version 430 core

out vec4 FragColor;

in vec2 TexCoords;
in float Pos;

uniform sampler3D atlasTexture;

layout(std430, binding = 0) buffer TextureSSBO {
    float textureData[];
};

void main() 
{  
    vec3 texCoords = vec3(TexCoords.x, TexCoords.y, 0);
    vec4 color = texture(atlasTexture, texCoords);

    FragColor = color;
}