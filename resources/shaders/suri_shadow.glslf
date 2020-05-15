#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
in vec4 t_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

void main() {
    vec4 pos = gl_FragCoord;

    vec4 pixCol = texture(t_Texture, v_Uv) * v_Color;
    float tex_alpha = pixCol.a; // * 0.8 + 0.2 * (pixColTL.a + pixColBR.a) / 2.0;
    //total = total / 3.0;
    vec2 frame_coord = vec2(fract(v_Uv.x * 10.0), fract(v_Uv.y * 10.0));
    float cent_dist = distance(frame_coord,vec2(0.50,0.65));

    if (tex_alpha < 0.5) {
        float shad = max(0.0, 0.5 - 0.3 * pow(cent_dist * 4.0, 2.0));
        pixCol.r = 0.01;
        pixCol.g = 0.015;
        pixCol.b = 0.025;
        pixCol.a = shad;
    }


    // blend overlay onto 
    Target0 = pixCol;

}