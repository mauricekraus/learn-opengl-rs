#version 460 core
in vec3 Normal;
in vec3 FragPos;
in vec3 LightPos;
out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 viewPos;

void main() {
  float ambientStrength = 0.1f;
  vec3 ambient = ambientStrength * lightColor;

  // diffuse
  vec3 norm = normalize(Normal);
  vec3 lightDir = normalize(LightPos - FragPos);
  // if angle is greater than 90 the angle becomes negative.. we clip that here
  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = diff * lightColor;

  // specular
  float specularStrength = 0.5;
  vec3 viewDir = normalize(-FragPos); // view is always 0.0.0
  vec3 reflectDir = reflect(-lightDir, norm);
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
  vec3 specular = specularStrength * spec * lightColor;

  vec3 result = (ambient + diffuse + specular) * objectColor;

  FragColor = vec4(result, 1.0);
}
