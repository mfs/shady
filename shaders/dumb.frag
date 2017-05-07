#version 140

uniform uvec2 dimensions;

out vec4 color;

void main() {

    float xratio = gl_FragCoord.x / dimensions.x;
    float yratio = gl_FragCoord.y / dimensions.y;

    if (gl_FragCoord.x < 50 || (dimensions.x - gl_FragCoord.x) < 50) {
	color = vec4(0);
    } else if (gl_FragCoord.y < 50 || (dimensions.y - gl_FragCoord.y) < 50){
	color = vec4(0);
    } else {
	color = vec4(1.0 * yratio, 1.0 * xratio, 0.0, 1.0);
    }
}
