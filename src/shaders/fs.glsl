#version 330 core

uniform vec4 col;
uniform sampler2D texture0;

in vec2 st_fs;

out vec4 frag_col;

void main() {
    frag_col = texture(texture0, st_fs) * col;
}
