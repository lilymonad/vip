in vec2 pos;
in vec3 color;

out vec3 color_out;

void main()
{
  gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
  color_out = color;
}
