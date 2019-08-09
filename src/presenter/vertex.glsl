#version 440

in dvec4 position;
in dvec3 normal;
in dvec3 color;
in dvec3 tex_coords;

out vec3 v_position;
out vec3 v_normal;
out vec4 v_color;
out vec3 v_tex_coords;

uniform dmat4 u_model_mat;
uniform dmat4 u_view_mat;
uniform dmat4 u_proj_mat;

void main() {
	gl_Position = vec4(u_proj_mat * u_view_mat * u_model_mat * position);
	v_position = vec3(u_model_mat * position);
	v_normal = vec3(normal);
	v_color = vec4(color, 1);
	v_tex_coords = vec3(tex_coords);
}
