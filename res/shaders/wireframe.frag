#version 330 core

in vec2 uv;

uniform vec4 u_face_color;
uniform vec4 u_line_color;
uniform float u_line_width;

out vec4 FragColor;

const bool show_triangles = false;

void main() {
    vec2 edge_dist = min(uv, 1.0 - uv);
    float min_edge = min(edge_dist.x, edge_dist.y);

    bool quad_edge = min_edge < u_line_width;
    bool diagonal = abs(uv.x + uv.y - 1.0) < u_line_width;
    
    if (quad_edge || (diagonal && show_triangles)) {
        FragColor = u_line_color;
    } else {
        FragColor = u_face_color;
    }
}
