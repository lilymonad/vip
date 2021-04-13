in vec3 color_out;

out vec4 diffuseColor;

void main()
{
  diffuseColor = vec4(color_out.r, color_out.g, color_out.b, 1.0);
}
