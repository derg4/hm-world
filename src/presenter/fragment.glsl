#version 140

in vec3 v_position;
in vec3 v_normal;
//in vec4 v_color;
in vec3 v_tex_coords;

out vec4 color;

uniform vec3 u_light_pos;
uniform vec4 u_light_color;
uniform vec4 u_light_ambient;
uniform sampler2DArray u_tex;

void main() {
	vec4 v_color = texture(u_tex, v_tex_coords);
	vec3 vert_to_light = normalize(u_light_pos - v_position);
	vec4 i_diffuse = v_color * u_light_color * max(dot(v_normal, vert_to_light), 0.0);
	vec4 i_ambient = v_color * u_light_ambient;

	color = clamp(i_diffuse + i_ambient, 0.0, 1.0);
}
