#version 330

in vec3 position;
in vec3 color;

out vec3 color_frag;

void main()
{
    gl_Position = vec4(position, 1.0);
    color_frag = color;
}