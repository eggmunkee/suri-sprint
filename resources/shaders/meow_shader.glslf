#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

void main() {

    Target0 = texture(t_Texture, v_Uv) * v_Color;
    /*
    vec4 texColor = texture(t_Texture, v_Uv) * v_Color;
    float a = texColor.a;
    float rev_a = min(1.0, 1.5 - a);
    float r = texColor.r;
    texColor.rgb = vec3(rev_a,rev_a,rev_a) + 2.0 * texColor.rgb;
    texColor.a = 1.0;
    //texColor.a = r;
    Target0 = texColor; */

}