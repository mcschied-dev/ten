#version 100
precision lowp float;

varying vec2 uv;
varying vec4 color;

uniform sampler2D Texture;
uniform float time;

// C64-style color palette effect
vec3 c64_color_cycle(float t) {
    // Create a rainbow cycle similar to C64 demos
    float r = sin(t * 3.0) * 0.5 + 0.5;
    float g = sin(t * 3.0 + 2.094) * 0.5 + 0.5;  // 2.094 = 2*PI/3
    float b = sin(t * 3.0 + 4.189) * 0.5 + 0.5;  // 4.189 = 4*PI/3
    return vec3(r, g, b);
}

void main() {
    // Apply sine wave distortion to UV coordinates
    vec2 distorted_uv = uv;

    // Horizontal wave (affects Y position based on X)
    distorted_uv.y += sin(uv.x * 20.0 + time * 3.0) * 0.015;

    // Vertical wave (affects X position based on Y)
    distorted_uv.x += sin(uv.y * 15.0 + time * 2.0) * 0.01;

    // Sample texture with distorted coordinates
    vec4 tex_color = texture2D(Texture, distorted_uv);

    // Apply color cycling effect to the text
    vec3 cycled_color = c64_color_cycle(time + uv.x * 2.0);

    // Mix original color with cycling color
    vec3 final_color = mix(tex_color.rgb * color.rgb, cycled_color, 0.7);

    // Add scanline effect for C64 authenticity
    float scanline = sin(uv.y * 800.0) * 0.05 + 0.95;
    final_color *= scanline;

    // Chromatic aberration for extra retro feel
    float aberration = 0.002;
    float r = texture2D(Texture, distorted_uv + vec2(aberration, 0.0)).r;
    float g = texture2D(Texture, distorted_uv).g;
    float b = texture2D(Texture, distorted_uv - vec2(aberration, 0.0)).b;

    vec3 aberrated = vec3(r, g, b);
    final_color = mix(final_color, aberrated * cycled_color, 0.3);

    gl_FragColor = vec4(final_color, tex_color.a * color.a);
}
