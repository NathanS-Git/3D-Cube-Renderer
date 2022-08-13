import numpy as np
import pygame as pg


# Modifiable parameters
initial_size = 150.0 # Each leg of the cube will be double this value
scale = 0 # How much the shape scales with each frame. 1 = Unchanged
shear = False # Must be true for shearing to function
drag = True # Whether drag is enabled or not. Functionally the same as drag_rate = 0 when disabled
drag_rate = 0.01 # 1 = Max friction, 0 = No friction at all
movement_multiplier = 0.01 # How fast we move the cube relative to cursor movement
x,y,z = 0,0,0 # The rate of change in the angle with respect to each dimension. (In degrees)
# Shear rates for every axis combination
sxy = 0
sxz = 0
syx = 0
syz = 0
szx = 0
szy = 0

# Eight points defining each vertex of the cube
#                        X             Y             Z
points = np.array([[-initial_size,-initial_size, initial_size], # Left top back
                   [-initial_size, initial_size, initial_size], # Left bottom back
                   [ initial_size, initial_size, initial_size], # Right bottom back
                   [ initial_size,-initial_size, initial_size], # Right top back
                   [-initial_size,-initial_size,-initial_size], # Left top front
                   [-initial_size, initial_size,-initial_size], # Left bottom front
                   [ initial_size, initial_size,-initial_size], # Right bottom front
                   [ initial_size,-initial_size,-initial_size]]) # Right top front


def normalize(vec):
    # Assumes numpy array
    total = 0
    for i in range(len(vec)):
        total += vec[i]
    return vec/total

def render_wireframe():
    for i in range(4): # Drawing lines connecting the vertices (Assuming it's a cube)
        pg.draw.line(screen, (255,255,255), points[i][:2]+cube_center, points[(i+1)%4][:2]+cube_center)
        pg.draw.line(screen, (255,255,255), points[i+4][:2]+cube_center, points[((i+1)%4)+4][:2]+cube_center)
        pg.draw.line(screen, (255,255,255), points[i][:2]+cube_center, points[i+4][:2]+cube_center)

def render_points():
    for i in range(len(points)):
        pg.draw.circle(screen, (np.cos([0.1+i,0.8+i,0.3+i])*0.25+0.75)*255, points[i][:2]+cube_center, 5)

def render_face(v_i):
    """ Renders face of cube given vertex indices that make-up that face. """

    light_direction = np.array([1,1,-1])

    # Obtaining normal vector from the front face of the cube to determine direction. 
    vec_a = points[v_i[0]]-points[v_i[1]]
    vec_b = points[v_i[0]]-points[v_i[2]]
    normal = np.cross(vec_a, vec_b)

    if normal[2] <= 0: # If the z coordinate of the face normal points outside the screen, render the face
        shading = ( np.dot(normal, light_direction)/(np.sqrt(normal.dot(normal))*np.sqrt(light_direction.dot(light_direction))) + 1 ) / 2
        pg.draw.polygon(screen, [shading*200+50]*3, [points[v_i[0]][:2]+cube_center,points[v_i[1]][:2]+cube_center,points[v_i[2]][:2]+cube_center,points[v_i[3]][:2]+cube_center])

def render_faces():
    render_face([3,2,1,0]) # back
    render_face([0,4,7,3]) # top
    render_face([4,5,6,7]) # front
    render_face([0,1,5,4]) # left-side
    render_face([2,3,7,6]) # right-side
    render_face([1,2,6,5]) # bottom


if (__name__ == "__main__"):

    pg.init()
    screen_size = np.array((600,600))
    screen = pg.display.set_mode(screen_size)
    cube_center = np.array(screen_size//2)
    clock = pg.time.Clock()
    ticks = pg.time.get_ticks()

    running = True
    while running:
        clock.tick(60) # 60fps target framerate
        screen.fill((0,0,0)) # Black background

        for e in pg.event.get():
            if pg.mouse.get_pressed()[0]:
                    prev_m_x,prev_m_y = m_x,m_y
                    m_x,m_y = pg.mouse.get_pos()
                    
                    y += (m_x-prev_m_x)*movement_multiplier
                    x += (m_y-prev_m_y)*movement_multiplier
            elif e.type == pg.KEYDOWN and e.key == pg.K_ESCAPE:
                running = False
            else:
                m_x,m_y = pg.mouse.get_pos()

        if drag: # Drag calculations
            x *= (1-drag_rate)
            y *= (1-drag_rate)
            z *= (1-drag_rate)

        #render_faces()
        #render_wireframe()
        render_points()

        # Building the transformation matrix
        transformation_matrix = np.array( ((1,0,0),(0,1,0),(0,0,1)) )
        if x:
            theta = np.radians(x)
            c, s = np.cos(theta), np.sin(theta)
            rotation_matrix_x = np.array( ((1,0,0),(0,c,-s),(0,s,c)) )
            transformation_matrix = np.matmul(transformation_matrix,rotation_matrix_x)
        if y:
            theta = np.radians(y)
            c, s = np.cos(-theta), np.sin(-theta)
            rotation_matrix_y = np.array( ((c,0,s),(0,1,0),(-s,0,c)) )
            transformation_matrix = np.matmul(transformation_matrix,rotation_matrix_y)
        if z:
            theta = np.radians(z)
            c, s = np.cos(theta), np.sin(theta)
            rotation_matrix_z = np.array( ((c,-s,0),(s,c,0),(0,0,1)) )
            transformation_matrix = np.matmul(transformation_matrix,rotation_matrix_z)
        if scale:
            scale_matrix = np.array( ((scale,0,0),(0,scale,0),(0,0,scale)) )
            transformation_matrix = np.matmul(transformation_matrix,scale_matrix)
        if shear:
            shear_matrix = np.array( ((1,sxy,sxz),(syx,1,syz),(szx,szy,1)) )
            transformation_matrix = np.matmul(transformation_matrix,shear_matrix)

        # Applying the transformation matrix to each vertex
        for i in range(len(points)):
            points[i] = np.matmul(transformation_matrix,points[i])
            # This is to prevent a bug that causes pygame to crash if a value goes to 'infinity'. We catch it before that can happen.
            if max(points[i]) >= 1e7 or min(points[i]) <= -1e7:
                print("Value going off to +\- infinity")
                exit()

        pg.display.update()
    quit()