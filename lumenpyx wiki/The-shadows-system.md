The shadow system involves using a heightmap to see which parts of the map the light will be able to make it to. 

To do this, inside the shader we use a line tracing algorithm to trace a line between the light and the pixel. As we go along every pixel gets checked to see if the height of the pixel exceeds the height of the line. If it does, we exit out of the line tracing algorithm, the pixel is in a shadow. If we make it to the end of the line tracing algorithm without any intersections, the line is not in a shadow.

Here is a 2D example of what we are doing the lines that are in a shadow are red and those that aren't are green: 
![Image](https://github.com/ABC-Engine/lumenpyx/blob/main/Visual%20Aids/2D%20Shadow%20Visualizer.png)
Here is the same thing but for a 3D example:
![Image](https://github.com/ABC-Engine/lumenpyx/blob/main/Visual%20Aids/3D%20Shadow%20Visualizer.png)

If the line is in a shadow we color it using shadow strength * intensity * albedo, where intensity is $intensity / (1 + (dist * falloff)^2)$. If not we color it using intensity * albedo. 