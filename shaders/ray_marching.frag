#version 140

// originally based on:
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

    for (int i = 0; i < 64; ++i) {
	vec3 p = o + r * t;
	t += map(p) * 0.5;
    }

    return t;
}

struct light {
    vec3 position;
    float intensity;
};

void main()
{

    vec2 uv = gl_FragCoord.xy / iResolution.xy;

    uv = uv * 2.0 - 1.0;

    uv.x *= float(iResolution.x) / float(iResolution.y);

    vec3 r = normalize(vec3(uv, 1.0));

    // camera 4 units back along z
    vec3 o = vec3(0.0, 0.0, -4.0);

    float t = trace(o, r);

    vec3 normal = o + r * t;

    light lights[] = light[](
	light(vec3(-10.0, 0.0, -5.0), 0.5),
	light(vec3(10.0, 0.0, -5.0), 0.5)
    );

    float lighting = 0.0;


    for (int i = 0; i < 2; i++) {
	// diffuse
	vec3 l = normalize(lights[i].position - normal);

	float dif = max(dot(normal, l), 0.0);

	lighting += dif;

	// specular
	float nl = dot(normal, l);
	if (nl > 0.0) {
	    vec3 rr = 2.0 * nl * normal - l;

	    float spec = pow(max(dot(rr, -r), 0.0), 9.0);

	    lighting += spec;
	}
    }

    vec3 fc = vec3(0.5, 0.5, 0.5) * lighting;

    color = vec4(fc, 1.0);
}
