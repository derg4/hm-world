#version 140

in vec4 position;
in vec3 normal;
in vec3 color;
in vec3 tex_coords;

out vec3 v_position;
out vec3 v_normal;
out vec4 v_color;
out vec3 v_tex_coords;

uniform mat4 u_model_mat;
uniform mat4 u_view_mat;
uniform mat4 u_proj_mat;

void main() {
	gl_Position = u_proj_mat * u_view_mat * u_model_mat * position;
	v_position = vec3(u_model_mat * position);
	v_normal = normal;
	v_color = vec4(color, 1);
	v_tex_coords = tex_coords;
}
