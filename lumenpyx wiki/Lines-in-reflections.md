Sometimes there are lines in the reflections. This will often occur in an angled plane where the height map is a gradient at some point. 

# Explanation
The problem here is with the auto-generated normals functionality and with color spaces. Since color is stored in 8bit channels there is often a point in your gradient where the color is constant for 2 pixels. The auto-generate function uses the difference between these two pixels to calculate the normal, so the difference in this region will be half of what it should be causing a defect in the normal map. This will cause lines in the reflected image because the reflected rays will land in unexpected places.

# Solution
The solution for this is simple (but can be a bit annoying). Set the debug of your program to Debug::Normal:
```rust
lumenpyx_program.set_debug(Debug::Normal);
```
Screenshot part of the normal map that doesn't have the lines (I recommend using Windows + Shift + S). Paste this into a tool like [this](https://redketchup.io/color-picker) (I'm not affiliated with this it just seems to work fine). Now use this color value to change your normals from ```Normal::AutoGenerated``` to ```[r, g, b].into()```. Don't forget to change the color space if necessary from 0-255 to 0-1!