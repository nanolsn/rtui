#version 330 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 st;

out vec2 st_fs;

void main() {
    st_fs = st;
    gl_Position = vec4(pos, 0.0, 1.0);
}
