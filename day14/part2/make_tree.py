width = 101
robots = [(width//2, 0)]

for i in range(width//2 + 1):
    robots.append((width//2 - i, i))
    robots.append((width//2 + i, i))

for robot in robots:
    print(f'p={robot[0]},{robot[1]} v=0,0')
