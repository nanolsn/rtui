#version 330 core

uniform vec4 col;
uniform sampler2D texture0;
uniform bool draw_texture;

in vec2 st_fs;

out vec4 frag;

void main() {
    if (draw_texture) {
        frag = texture(texture0, st_fs) * col;
    } else {
        frag = col;
    }
}
