#version 140

uniform uvec2 iResolution;
uniform vec4 iMouse;
out vec4 color;

void main() {

    vec2 mouse = iMouse.xy;

    int x_step = int(gl_FragCoord.x - mouse.x) / 80;
    int y_step = int(gl_FragCoord.y + mouse.y) / 80;

    if ((x_step + y_step) % 2 == 0) {
	color = vec4(0.7, 0.0, 0.0, 1.0);
    } else {
	color = vec4(0.7, 0.7, 0.7, 1.0);
    }
}
