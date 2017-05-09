#version 140

uniform int iFrame;
uniform uvec2 iResolution;

out vec4 color;

void main() {

    float xratio = gl_FragCoord.x / iResolution.x;
    float yratio = gl_FragCoord.y / iResolution.y;

    float c = (iFrame % 300) / 300.0;

    if (gl_FragCoord.x < 50 || (iResolution.x - gl_FragCoord.x) < 50) {
	color = vec4(c);
    } else if (gl_FragCoord.y < 50 || (iResolution.y - gl_FragCoord.y) < 50){
	color = vec4(c);
    } else {
	color = vec4(1.0 * yratio, 1.0 * xratio, 0.0, 1.0);
    }
}
