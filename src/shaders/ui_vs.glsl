#version 330 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 st;

uniform mat4 projection;

out vec2 st_fs;

void main() {
    st_fs = st;
    gl_Position = projection * vec4(pos, 0.0, 1.0);
}
