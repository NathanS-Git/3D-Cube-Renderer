import numpy as np
import pygame as pg

pg.init()
screen_width = 600
screen_height = 600
screen = pg.display.set_mode((screen_width, screen_height))
cube_center_x = screen_height//2
cube_center_y = screen_height//2
clock = pg.time.Clock()
ticks = pg.time.get_ticks()


# Modifiable parameters
initial_size = 150 # Each leg of the cube will be double this value
scale = 1 # How much the shape scales with each frame. 1 = Unchanged
shear = False # Must be true for shearing to function
drag = True # Whether drag is enabled or not. Functionally the same as drag_rate = 0 when disabled
drag_rate = 0.00 # 1 = Max friction, 0 = No friction at all
movement_multiplier = 0.01 # How fast we move the cube relative to cursor movement
x,y,z = 0,0,0 # The rate of change in the angle with respect to each dimension. (In degrees)
# Shear rates for every axis combination
sxy = 0.02
sxz = 0.001
syx = 0
syz = 0
szx = 0
szy = 0

# Eight points defining each vertex of the cube
#               X             Y             Z
points = [[-initial_size,-initial_size,initial_size], # Left top front
          [-initial_size,initial_size,initial_size], # Left bottom front
          [initial_size,initial_size,initial_size], # Right bottom front
          [initial_size,-initial_size,initial_size], # Right top front 
          [-initial_size,-initial_size,-initial_size], # Left top back
          [-initial_size,initial_size,-initial_size], # Left bottom back
          [initial_size,initial_size,-initial_size], # Right bottom back
          [initial_size,-initial_size,-initial_size]] # Right top back   

def center_point(point, center_x=cube_center_x, center_y=cube_center_y):
    return (point[0]+center_x,point[1]+center_y)

while True:
    clock.tick(60) # 60fps target framerate
    screen.fill((0,0,0)) # Black background

    for e in pg.event.get():
        if pg.mouse.get_pressed()[0]:
                prev_m_x,prev_m_y = m_x,m_y
                m_x,m_y = pg.mouse.get_pos()
                
                y += (m_x-prev_m_x)*movement_multiplier
                x += (m_y-prev_m_y)*movement_multiplier
        else:
            m_x,m_y = pg.mouse.get_pos()

    if drag: # Drag calculations
        x *= (1-drag_rate)
        y *= (1-drag_rate)
        z *= (1-drag_rate)

    for i in range(4): # Drawing lines connecting the vertices (Assuming it's a cube)
        pg.draw.line(screen, (255,255,255), (int(points[i][0])+cube_center_x, int(points[i][1])+cube_center_y) , (int(points[(i+1)%4][0])+cube_center_x, int(points[(i+1)%4][1])+cube_center_y) )
        pg.draw.line(screen, (255,255,255), (int(points[i+4][0])+cube_center_x, int(points[i+4][1])+cube_center_y) , (int(points[((i+1)%4)+4][0])+cube_center_x, int(points[((i+1)%4)+4][1])+cube_center_y) )
        pg.draw.line(screen, (255,255,255), (int(points[i][0])+cube_center_x, int(points[i][1])+cube_center_y) , (int(points[i+4][0])+cube_center_x, int(points[i+4][1])+cube_center_y) )

    # Obtaining normal vector from the front face of the cube to determine direction. 
    vec_a = np.array(points[0])-np.array(points[2])
    vec_b = np.array(points[0])-np.array(points[1])

    normal = ( vec_a[1]*vec_b[2]-vec_a[2]*vec_b[1] , vec_a[2]*vec_b[0]-vec_a[0]*vec_b[2] , vec_a[0]*vec_b[1]-vec_a[1]*vec_b[0] )

    face = ( (points[1][0]+points[3][0])/2 , (points[1][1]+points[3][1])/2 ) # Center of cube face 
    pg.draw.line(screen, (255,0,0),  center_point(face), center_point(np.array(normal)*0.0025), width=3)
    
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
        points[i] = np.matmul(points[i],transformation_matrix)
        # This is to pervent a bug that causes pygame to crash if a value goes to 'infinity'. We catch it before that can happen.
        if max(points[i]) >= 1e7 or min(points[i]) <= -1e7:
            print("Value going off to +\- infinity")
            exit()

    pg.display.update()
quit()