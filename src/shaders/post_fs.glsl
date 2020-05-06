#version 330 core

uniform sampler2D frame;

in vec2 st_fs;

out vec4 frag;

void main() {
	frag = texture(frame, st_fs);
}
