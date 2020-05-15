#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

void main() {
    vec4 pos = gl_FragCoord;
    //pos.x = fract(pos.x * 10.0);
    //pos.y = fract(pos.y * 10.0);
    vec2 frame_coord = vec2(fract(v_Uv.x * 10.0), fract(v_Uv.y * 10.0));
    float pix_scale = 0.1 * 0.01;
    vec4 pixColT = texture(t_Texture, v_Uv + pix_scale * vec2(0.0,1.0)) * v_Color;
    vec4 pixColR = texture(t_Texture, v_Uv + pix_scale * vec2(1.0,0.0)) * v_Color;
    vec4 pixColB = texture(t_Texture, v_Uv + pix_scale * vec2(0.0,-1.0)) * v_Color;
    vec4 pixColL = texture(t_Texture, v_Uv + pix_scale * vec2(-1.0,0.0)) * v_Color;
    float surrAlpha = pixColT.a + pixColR.a + pixColB.a + pixColL.a;
    vec4 pixCol = texture(t_Texture, v_Uv) * v_Color;
    float tex_colors = pixCol.r + pixCol.g + pixCol.b;
    float tex_alpha = pixCol.a; // * 0.8 + 0.2 * (pixColTL.a + pixColBR.a) / 2.0;
    //total = total / 3.0;
    float cent_dist = distance(frame_coord,vec2(0.53,0.59));

    vec4 overlay = vec4(0.0,0.0,0.0,0.0);
    if (pixCol.a > 0.9) {
        overlay.a = 0.0;
    }
    else if (pixCol.a > 0.0 || surrAlpha > 0.7) { //} && pixCol.a < 0.90)) { //  || tex_colors > 0.7
        float strength = 1.0-0.2*surrAlpha;
        overlay.rgba = vec4(vec3(1.0), 1.0);
        // overlay.r = 0.5 + 0.5 * cos(-frame_coord.x * 133.0) - 0.3 * sin(-frame_coord.y * 150.0) + cos((1.5-frame_coord.x)*(1.0-frame_coord.y) * 145.0);
        // overlay.g = 0.5 - 0.5 * cos(-frame_coord.y * 169.0) + 0.3 * sin(frame_coord.x * 138.0);
        // overlay.b = 0.3 + 0.3 * cos((1.5-frame_coord.x)*(1.0-frame_coord.y) * 172.0) - 0.3 * sin((0.05 - frame_coord.x) * 131.0);
        // overlay.r *= strength;
        // overlay.g *= strength;
        // overlay.b *= strength * 0.5;
        //overlay.a = strength * 0.25; // = vec4(0.1+strength,0.1+strength,0.1+strength,strength);
    }
    else {
        float overlay_str = max(0.0, 1.0 - pow(2.5*cent_dist+ 0.2*cos(10.0*pos.x-15.0*pos.y) + 0.2*sin(-17.0*pos.x+20.0*pos.y), 2.0) - 0.1 * pixCol.a);
        overlay.r = 0.5 + 0.5 * cos(frame_coord.x * 33.0) - 0.3 * sin(frame_coord.y * 50.0) + cos((1.5-frame_coord.x)*(1.0-frame_coord.y) * 45.0);
        overlay.g = 0.5 - 0.5 * cos(frame_coord.y * 69.0) + 0.3 * sin(-frame_coord.x * 38.0);
        overlay.b = 0.5 + 0.7 * cos((1.5-frame_coord.x)*(1.0-frame_coord.y) * 72.0) - 0.7 * sin((0.05 - frame_coord.x) * 31.0);
        overlay.a = overlay_str;
        //overlay = vec4(0.0, 0.5, 1.0, 0.5);
    }
    
    
    // overlay.r = 0.25 + 0.5 * sin(frame_coord.x * 3.14); //+ 0.5 * cos(pos.x * 1.0)
    // overlay.g = 0.25 - 0.5 * cos(frame_coord.y * 3.14); // - 0.5 * sin(pos.y * 1.0) 

    // blend overlay onto 
    Target0 = pixCol + overlay;// * max(1.5 - tex_colors, 0.0);
    // if (overlay.a > 0.0) {
    //     Target0 = pixCol + overlay;// * max(1.5 - tex_colors, 0.0);
    // }
    // else {
    //     Target0 = pixCol;
    // }

}