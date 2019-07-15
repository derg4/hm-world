#!/usr/bin/python

from __future__ import print_function
import math, sys
from math import pi
from obj_writer import format_obj

def sphere_gen(lat_divs, lon_divs):
    lat_divs = int(lat_divs)
    lon_divs = int(lon_divs)

    if lat_divs < 2 or lon_divs < 3:
        return None
    lat_inc = pi / lat_divs
    lon_inc = 2 * pi / lon_divs
    verts = [] # Stores vert coords in (lat-angle, lon-angle) pairs
    faces  = [] # Stores faces by vert indices (i1, i2, i3) CCW triplets

    for lat_div in range(lat_divs + 1):
        verts.append((lat_div * lat_inc, 0.0))

        for lon_div in range(1, lon_divs + 1):
            if lat_div == 0:
                # Top of sphere, only add top vert
                break
            elif lat_div == 1:
                # First ring, add one vert and one tri for each lon_div
                if lon_div != lon_divs:
                    verts.append((lat_div * lat_inc, lon_div * lon_inc))
                    faces.append((0,
                                 len(verts) - 2,
                                 len(verts) - 1))
                else:
                    faces.append((0,
                                 len(verts) - 1,
                                 len(verts) - lon_divs))
            elif lat_div == lat_divs:
                # Last ring, add one tri for each lon div (and no verts)
                if lon_div != lon_divs:
                    faces.append((len(verts) - 1 - lon_divs + lon_div,
                                 len(verts) - 2 - lon_divs + lon_div,
                                 len(verts) - 1))
                else:
                    faces.append((len(verts) - 1 - lon_divs,
                                 len(verts) - 2,
                                 len(verts) - 1))
            else:
                # Middle rings, add one vert and two faces for each lon div
                if lon_div != lon_divs:
                    verts.append((lat_div * lat_inc, lon_div * lon_inc))
                    faces.append((len(verts) - 1 - lon_divs,
                                 len(verts) - 2 - lon_divs,
                                 len(verts) - 2))
                    faces.append((len(verts) - 1 - lon_divs,
                                 len(verts) - 2,
                                 len(verts) - 1))
                else:
                    faces.append((len(verts) - 2*lon_divs,
                                 len(verts) - 1 - lon_divs,
                                 len(verts) - 1))
                    faces.append((len(verts) - 2*lon_divs,
                                 len(verts) - 1,
                                 len(verts) - lon_divs))

    #num_v = 0
    #for vert in verts:
    #    print('vert {:3d}: ({:3.2f}, {:3.2f})'.format(num_v, vert[0], vert[1]))
    #    num_v += 1
    #num_t = 0
    #for tri in faces:
    #    print('tri  {:3d}: ({:3d},{:3d},{:3d})'.format(num_t, tri[0], tri[1], tri[2]))
    #    num_t += 1
    cartesian_verts = spherical_to_cartesian(verts, 1.0)
    return (cartesian_verts, cartesian_verts, faces)

def spherical_to_cartesian(sph_verts, radius):
    cart_verts = []
    for (lat, lon) in sph_verts:
        x = math.cos(lon) * math.sin(lat) * radius
        y = math.cos(lat) * radius
        z = -math.sin(lon) * math.sin(lat) * radius
        cart_verts.append((x, y, z))
    return cart_verts

def sphere_print(sphere):
    (verts, faces) = sphere
    for (i1, i2, i3) in faces:
        v1 = verts[i1]
        v2 = verts[i2]
        v3 = verts[i3]
        print('({:3.2f}, {:3.2f}), ({:3.2f}, {:3.2f}), ({:3.2f}, {:3.2f})'.format(
            v1[0], v1[1], v2[0], v2[1], v3[0], v3[1]))

def cart_print(verts, faces):
    for (i1, i2, i3) in faces:
        v1 = verts[i1]
        v2 = verts[i2]
        v3 = verts[i3]
        print('({:3.2f}, {:3.2f}, {:3.2f}),({:3.2f}, {:3.2f}, {:3.2f}),({:3.2f}, {:3.2f}, {:3.2f})'.format(
            v1[0], v1[1], v1[2], v2[0], v2[1], v2[2], v3[0], v3[1], v3[2]))

if __name__ == '__main__':
    #resolution = 45
    #(verts, normals, faces) = sphere_gen(180 / resolution, 360 / resolution)
    ##cart_print(spherical_to_cartesian(verts, 1.0), faces)
    ##print()
    #print(format_obj("Sphere with %s degree resolution" % resolution, verts, normals, faces))

    for resolution in [1, 2, 3, 4, 5, 6, 10, 15, 30, 45, 60, 90]:
        (verts, normals, faces) = sphere_gen(180 / resolution, 360 / resolution)
        f = open('models/sphere-%02d.obj' % resolution, 'w')
        f.write(format_obj("Sphere with %s degree resolution" % resolution, verts, normals, faces))
        f.close()
