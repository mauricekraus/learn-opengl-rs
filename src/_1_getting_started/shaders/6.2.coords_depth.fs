#version 420 core
out vec4 FragColor;

in vec2 texCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main() 
{
  FragColor = mix(texture(texture1, texCoord), texture(texture2, vec2(texCoord.x * -1, texCoord.y)), 0.2);
}
