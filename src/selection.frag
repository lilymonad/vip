in vec3 fcolor;
in vec2 texcoord;

uniform sampler2D tex;

out vec4 diffuseColor;

void main()
{
    vec4 fullcolor = texture(tex, texcoord);
    diffuseColor = vec4(fcolor, 1) * fullcolor;
}
