#version 330 core

uniform vec4 col;
uniform sampler2D texture0;

in vec2 st_fs;

out vec4 frag;

void main() {
    frag = vec4(1.0, 1.0, 1.0, texture(texture0, st_fs).r) * col;
}
