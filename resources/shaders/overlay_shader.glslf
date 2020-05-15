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

void main() {
    vec2 pos = v_Uv;
    float cent_dist = distance(pos, vec2(0.5,0.5));
    float x_dist = abs(0.5-pos.x);
    float y_dist = abs(0.5-pos.y);

    vec4 pixCol = texture(t_Texture, v_Uv) * v_Color;

    float edge_amt = 0.25 + (pixCol.r + pixCol.g + pixCol.b) * 0.5;
    float comb_edge_dist = min(1.0, 1.0 * max(min(1.0, 0.55 * x_dist) + min(1.0, 0.55 * y_dist), 0.6 * cent_dist)); //+ 1.5 * cent_dist));

    float siren_amt = 0.5 + 0.5 * sin(u_Rate * 2.0);
    vec4 ol_pix = vec4(0.0);
    float cent_strength = min(1.5, 0.8 * cent_dist);

    ol_pix.r = (siren_amt * pixCol.r * 0.5 + 0.5 * cos(15.0 * u_Rate) * sin((u_Rate + pos.y) * 52.0)) - 1.0 * (1.0 - 1.5* cent_strength);

    ol_pix.g = (pixCol.g * 0.4
        //+ 0.2 * cos(15.0 * (u_Rate*10.1 - pos.x*2.5))
        + (0.3 * abs(cos(25.5 * pos.y + u_Rate) + 0.3)
        * 0.4 * sin(u_Rate * 0.03))) - 1.5 * (1.0 - cent_strength);
    ol_pix.b = (pixCol.b * 0.4 + 0.4 * sin( (3.5*u_Rate-2.1) 
        + 15.0 * (5.1 * pos.x - 1.8*pos.y) )
        ) - 0.5 * (1.0 - 1.5 * cent_strength);

    pixCol.a = (pixCol.a * 0.5 + 0.5 * sin((0.7 * u_Rate - 14.2 * pos.x + 4.7 * pos.y) * 1.5)
        + 0.8 * cos((0.23 * u_Rate - 8.2 * pos.x + 0.1 * pos.y) * 1.33)
        + 0.8 * (cos((1.5 * u_Rate - 12.2 * pos.x + 2.5 * pos.y) * 0.9)+0.5)
        + 0.5 * (cos((-0.02 * u_Rate + 7.2 * pos.x - 10.7 * pos.y) * 1.2)+0.5)
        + 0.5 * (cos((-0.13 * u_Rate + 1.2 * pos.x - 3.7 * pos.y) * 1.2)+0.3)
        - 0.7 * abs( sin((-0.42 * u_Rate + 0.04 * pos.x - 3.1 * pos.y) * 1.0))
        ) - 1.5 * (1.0 - 1.5 * cent_strength);
    pixCol.a = pixCol.a * 1.0;

    //float left_edge = max(0.0, 80.0 - pos.x) / 80.0;
    //float top_edge = max(0.0, 80.0 - pos.y) / 80.0;
    float edge = siren_amt * comb_edge_dist - pow(min(1.0, 1.0 * (1.0 - comb_edge_dist) / comb_edge_dist), 2.0);             //(0.5*left_edge + 0.5*top_edge);

    //float mixPerc = (0.5 + 0.5 * cos(pos.x * 110.25) + 0.5 * sin(pos.y * 70.25)) * comb_edge_dist;
    //float baseAlpha = max(0.0, 0.2 + 0.5 * cos(pos.x * 0.05) + 0.5 * sin(pos.y * 0.05));

    pixCol.rgb = mix(ol_pix.rgb, pixCol.rgb, comb_edge_dist);
    //pixCol.r = max(pixCol.r, 0.25);
    pixCol.a = pixCol.a + (cent_strength * comb_edge_dist) + 0.1;

    Target0 = pixCol;

}