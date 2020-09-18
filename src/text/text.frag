in vec4 fcolor;
in vec2 texcoord;

uniform sampler2D tex;
uniform mat3 view;

out vec4 diffuseColor;

float median(float r, float g, float b) {
  return max(min(r, g), min(max(r, g), b));
}

void main()
{
  vec3 msdf = texture(tex, texcoord).rgb;
  float sigDist = median(msdf.r, msdf.g, msdf.b);
  float w = fwidth(sigDist);
  float opacity = smoothstep(0.5 - w, 0.5 + w, sigDist);

  
  diffuseColor = vec4(fcolor.rgb, opacity);
}
