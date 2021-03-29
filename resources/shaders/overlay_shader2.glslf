#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

layout (std140) uniform overlay {
    float u_Rate;
};

float pixel_waves(in float base_r, in float orig_a, float x_dist, float y_dist, 
        in vec2 pos, in float siren_amt, in float cent_strength) {
    return (base_r * orig_a + 0.5 * cos(1.8 * u_Rate) * sin((u_Rate + (pos.y-0.5)) * 17.0
        + 4.0 * (floor(9.02 * u_Rate + 11.1 * (pos.x - 0.5 ) * y_dist - 37.0*(pos.y-0.5) * x_dist))
            * (floor(2.32 * u_Rate + 35.1 * (pos.x - 0.5) * x_dist - 17.0*(pos.y-0.5)*y_dist))
    )) 
        + siren_amt * 0.25 * ( 1.5* cent_strength);
}

// float pixel_waves(in float base_r, in float orig_a, in vec2 pos, in float siren_amt, in float cent_strength) {
//     return (base_r * orig_a + 0.5 * cos(1.8 * u_Rate) * sin((u_Rate + (pos.y-0.5)) * 17.0
//         + 4.0 * (floor(9.02 * u_Rate + 11.1 * (pos.x - 0.5)*y_dist - 37.0*(pos.y-0.5) * x_dist))
//             * (floor(2.32 * u_Rate + 35.1 * (pos.x - 0.5) * x_dist - 17.0*(pos.y-0.5)*y_dist))
//     )) 
//         + siren_amt * 0.25 * ( 1.5* cent_strength);
// }

void main() {
    vec2 pos = v_Uv;
    float cent_dist = distance(pos, vec2(0.5,0.5));
    float x_dist = abs(0.5-pos.x);
    float y_dist = abs(0.5-pos.y);

    vec4 pixCol = texture(t_Texture, v_Uv) * v_Color;

    float edge_amt = 0.25 + (pixCol.r + pixCol.g + pixCol.b) * 0.5;
    float comb_edge_dist = min(1.0, 1.0 * max(min(1.0, 0.55 * x_dist) + min(1.0, 0.55 * y_dist), 0.6 * cent_dist)); //+ 1.5 * cent_dist));

    float siren_amt = 0.5 + 0.35 * sin(u_Rate * 2.25) + 0.5 * cos(u_Rate * 0.09) + 0.2 * cos(u_Rate * 2.09)
         + 0.4 * sin(u_Rate * 4.8) 
         + 0.5 * sin(pos.x * 4.25) + 0.5 * cos(-pos.y * 7.09);
    vec4 ol_pix = vec4(0.0);
    float cent_strength = min(1.5, 1.2 * cent_dist);

    float orig_a = pixCol.a;

    // ol_pix.r = (pixCol.r * orig_a + 0.5 * cos(1.8 * u_Rate) * sin((u_Rate + (pos.y-0.5)) * 17.0
    //     + 4.0 * (floor(9.02 * u_Rate + 11.1 * (pos.x - 0.5)*y_dist - 37.0*(pos.y-0.5) * x_dist))
    //         * (floor(2.32 * u_Rate + 35.1 * (pos.x - 0.5) * x_dist - 17.0*(pos.y-0.5)*y_dist))
    // )) 
    //     + siren_amt * 0.25 * ( 1.5* cent_strength);
    ol_pix.r = pixel_waves(pixCol.r, orig_a, x_dist, y_dist, pos, siren_amt, cent_strength);
    ol_pix.g = pixel_waves(pixCol.g, orig_a, y_dist*1.1,1.2*x_dist,  pos, siren_amt, cent_strength);
    ol_pix.b = pixel_waves(pixCol.b, orig_a, x_dist*0.8, y_dist*0.9, pos, siren_amt, cent_strength);


    // ol_pix.g = (pixCol.g * orig_a
    //     + 0.2 * cos(15.0 * (u_Rate*10.1 - pos.x*2.5))
    //     + (0.3 * abs(cos(25.5 * pos.y + u_Rate) + 0.3)
    //     * 0.4 * sin(u_Rate * 0.03))) - 0.5 * (1.0 - 1.5*cent_strength);
    // ol_pix.b = (pixCol.b * orig_a + 0.4 * sin( (0.25*u_Rate-2.1) 
    //     + 15.0 * (floor(3.02 * u_Rate + 4.1 * pos.x * x_dist - 21.0*pos.y*y_dist))
    //         * (floor(1.02 * u_Rate + 5.1 * pos.x * x_dist - 12.0*pos.y*y_dist)) )
    //     ) - 0.5 * (1.0 - 1.5 * cent_strength);

    // pixCol.a = (pixCol.a * orig_a + 0.5 * sin((0.7 * u_Rate - 14.2 * pos.x + 4.7 * pos.y) * 1.5)
    //     + 0.5 * cos((0.23 * u_Rate - 8.2 * pos.x + 0.1 * pos.y) * 1.33)
    //     + 0.5 * (cos((1.5 * u_Rate - 12.2 * pos.x + 2.5 * pos.y) * 0.9)+0.5)
    //     + 0.5 * (cos((-0.02 * u_Rate + 7.2 * pos.x - 10.7 * pos.y) * 1.2)+0.5)
    //     + 0.5 * (cos((-0.13 * u_Rate + 1.2 * pos.x - 3.7 * pos.y) * 1.2)+0.3)
    //     - 0.5 * abs( sin((-0.42 * u_Rate + 0.04 * pos.x - 3.1 * pos.y) * 1.0))
    //     ) - 1.5 * (1.0 - 1.5 * cent_strength);
    //pixCol.a = max(0.0, pixCol.a - orig_a);

    //float left_edge = max(0.0, 80.0 - pos.x) / 80.0;
    //float top_edge = max(0.0, 80.0 - pos.y) / 80.0;
    float edge = comb_edge_dist - pow(min(1.0, 1.0 * (1.0 - comb_edge_dist) / comb_edge_dist), 2.0);             //(0.5*left_edge + 0.5*top_edge);

    //float mixPerc = (0.5 + 0.5 * cos(pos.x * 110.25) + 0.5 * sin(pos.y * 70.25)) * comb_edge_dist;
    //float baseAlpha = max(0.0, 0.2 + 0.5 * cos(pos.x * 0.05) + 0.5 * sin(pos.y * 0.05));

    pixCol.rgb = mix(ol_pix.rgb, pixCol.rgb, edge);
    //pixCol.r = max(pixCol.r, 0.25);
    pixCol.a = pixCol.a + 0.1;

    Target0 = pixCol;

}