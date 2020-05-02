in vec2 pos;
in vec2 texPos;
in vec3 onColor;

uniform sampler2D tex;
uniform mat3 view;

out float luminance;
out vec2 texcoord;

void main()
{
    vec3 fpos = vec3(pos, 1.0) * view;
    gl_Position = vec4(fpos.x, fpos.y, 0.0, 1.0);

    luminance = 0.3 * onColor.r + 0.59 * onColor.g + 0.11 * onColor.b;
    texcoord = vec2(texPos.x, texPos.y);
}
