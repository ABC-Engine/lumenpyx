Sprites have 4 components: an albedo map, a height map, a roughness map, and a normalmap.

# The Albedo Map
The albedo map is just a regular image. We do support transparency.

# The Heightmap
The height map is an image representation of the height of each pixel in the image. This will affect the shadows cast by your objects, so make them carefully. They can be solid colors for each object, or have more depth by using a texture.

# The Roughness Map
The roughness map is not necessary unless you want reflections like a lake. If you do, treat this texture like a mask, with white being the parts that will show the reflections. Keep in mind the reflections can only reflect things that are on the screen.

# The Normal Map
The normal map should 90% of the time just be set to ```Normal::AutoGenerate``` but sometimes, this autogenerate feature can produce defects, so there is an option to put this in manually as well.