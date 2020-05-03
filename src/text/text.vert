in vec2 pos;
in vec2 texPos;

uniform sampler2D tex;
uniform mat3 view;

out vec4 fcolor;
out vec2 texcoord;

void main()
{
    vec3 fpos = vec3(pos, 1) * view;
    gl_Position = vec4(fpos.x, fpos.y, 0, 1.0);

    fcolor = vec4(0,1,1,1);

    texcoord = texPos;
}
