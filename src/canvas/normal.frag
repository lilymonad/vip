in vec2 texcoord;

uniform sampler2D tex;

out vec4 diffuseColor;

void main()
{
    diffuseColor = texture(tex, texcoord);
}
