#version 330

in vec3 color_frag;

out vec4 out_color;

void main()
{
    out_color = vec4(color_frag, 1.0);
}