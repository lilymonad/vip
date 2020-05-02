in float luminance;
in vec2 texcoord;

uniform sampler2D tex;

out vec4 diffuseColor;

void main()
{
    vec4 fullcolor = texture(tex, texcoord);

    if (luminance < 0.5) {
        diffuseColor = fullcolor;
    } else {
        diffuseColor = vec4(0, 0, 0, 1) * fullcolor;
    }
}
