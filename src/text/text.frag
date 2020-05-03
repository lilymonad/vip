in vec4 fcolor;
in vec2 texcoord;

uniform sampler2D tex;
uniform mat3 view;

out vec4 diffuseColor;

void main()
{
  float v = texture(tex, texcoord).r;

  diffuseColor = v * fcolor;
}
