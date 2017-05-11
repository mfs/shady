#version 140

// https://www.youtube.com/watch?v=yxNnRSefK94

uniform int iFrame;
uniform uvec2 iResolution;

out vec4 color;

float map(vec3 p)
{
    // radius 1 sphere at origin
    return length(p) - 1.0;
}

float trace(vec3 o, vec3 r)
{
    float t = 0.0;

    for (int i = 0; i < 32; ++i) {
	vec3 p = o + r * t;
	t += map(p) * 0.5;
    }

    return t;
}

void main()
{

    vec2 uv = gl_FragCoord.xy / iResolution.xy;

    uv = uv * 2.0 - 1.0;

    uv.x *= float(iResolution.x) / float(iResolution.y);

    vec3 r = normalize(vec3(uv, 1.0));

    // camera 3 units back along z
    vec3 o = vec3(0.0, 0.0, -3.0);

    float t = trace(o, r);

    float fog = 1.0 / (1.0 + t * t * 0.1);

    vec3 fc = vec3(fog);

    color = vec4(fc, 1.0);
}
