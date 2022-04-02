#version 420 core
out vec4 FragColor;

/* in vec3 ourColor; */

void main() {
  vec3 ourColor = vec3(0.3, 0.4, 0.4);
  FragColor = vec4(ourColor, 1.0);
}
