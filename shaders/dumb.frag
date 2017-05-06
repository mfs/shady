#version 140

out vec4 color;

void main() {
    if (gl_FragCoord.x < 100 || gl_FragCoord.y < 100) {
	color = vec4(0.0, 1.0, 0.5, 1.0);
    } else {
	color = vec4(1.0, 0.0, 0.0, 1.0);
    }
}
