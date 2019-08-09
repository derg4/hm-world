#version 440

in vec3 v_position;
in vec3 v_normal;
//in vec4 v_color;
in vec3 v_tex_coords;

out vec4 color;

uniform dvec3 u_light_pos;
uniform dvec4 u_light_color;
uniform dvec4 u_light_ambient;
uniform sampler2DArray u_tex;

void main() {
	dvec4 v_color = texture(u_tex, v_tex_coords);
	dvec3 vert_to_light = normalize(u_light_pos - v_position);
	dvec4 i_diffuse = v_color * u_light_color * max(dot(v_normal, vert_to_light), 0.0);
	dvec4 i_ambient = v_color * u_light_ambient;

	color = vec4(clamp(i_diffuse + i_ambient, 0.0, 1.0));
}
