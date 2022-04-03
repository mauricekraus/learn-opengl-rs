#version 460 core
in vec3 Normal;
in vec3 FragPos;
out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;

void main() {
  float ambientStrength = 0.1f;
  vec3 ambient = ambientStrength * lightColor;

  // diffuse
  vec3 norm = normalize(Normal);
  vec3 lightDir = normalize(lightPos - FragPos);
  // if angle is greater than 90 the angle becomes negative.. we clip that here
  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = diff * lightColor;

  vec3 result = (ambient + diffuse) * objectColor;

  FragColor = vec4(result, 1.0);
}
