
pub const DEFAULT_FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 140

    out vec4 color;

    void main() {
        int x_step = int(gl_FragCoord.x) / 40;
        int y_step = int(gl_FragCoord.y) / 40;

        if ((x_step + y_step) % 2 == 0) {
            color = vec4(0.4, 0.4, 0.4, 1.0);
        } else {
            color = vec4(0.7, 0.7, 0.7, 1.0);
        }
    }
"#;

pub const VERTEX_SHADER_SRC: &'static str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;
