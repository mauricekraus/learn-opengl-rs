#version 460 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;
out vec3 Normal;
out vec3 FragPos;
out vec3 LightPos;
/* layout(location = 1) in vec3 aColor; */
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 lightPos;
/* out vec3 ourColor; */
void main() {
  Normal = mat3(transpose(inverse(view * model))) * aNormal;
  FragPos = vec3(view * model * vec4(aPos, 1.0));
  gl_Position = projection * view * model * vec4(aPos, 1.0);
  LightPos = vec3(view * vec4(lightPos, 1.0));

  // compute everything in view space
}
