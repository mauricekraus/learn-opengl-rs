#version 460 core
in vec3 Normal;
in vec3 FragPos;
in vec2 texCoord;
out vec4 FragColor;

uniform vec3 viewPos;

struct Material {
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
  float shininess;
};

uniform Material material;

struct Light {
  vec3 position;
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};
uniform Light light;
uniform sampler2D tex;

void main() {
  vec3 ambient = light.ambient * material.ambient;

  // diffuse
  vec3 norm = normalize(Normal);
  vec3 lightDir = normalize(light.position - FragPos);
  // if angle is greater than 90 the angle becomes negative.. we clip that here
  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = light.diffuse * (diff * material.diffuse);

  // specular
  vec3 viewDir = normalize(viewPos - FragPos); // view is always 0.0.0
  vec3 reflectDir = reflect(-lightDir, norm);
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
  vec3 specular = light.specular * (spec * material.specular);

  vec3 result = ambient + diffuse + specular;
  /* FraColor = vec4(result, 1.0); */
  FragColor = vec4(texture(tex, texCoord).rgb, 1.0);
}
