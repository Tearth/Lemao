#version 330 core
in vec4 ourColor;
in vec2 TexCoord;
out vec4 FragColor;

uniform vec4 gradientSteps;
uniform vec4 gradientStep0Color;
uniform vec4 gradientStep1Color;
uniform vec4 gradientStep2Color;
uniform vec4 gradientStep3Color;

uniform sampler2D ourTexture;

void main()
{
    vec2 pos_ndc = TexCoord * 2 - 1;
    float dist = length(pos_ndc);

    vec4 color = mix(gradientStep0Color, gradientStep1Color, smoothstep(gradientSteps.x, gradientSteps.y, dist));
    color = mix(color, gradientStep2Color, smoothstep(gradientSteps.y, gradientSteps.z, dist));
    color = mix(color, gradientStep3Color, smoothstep(gradientSteps.z, gradientSteps.w, dist));
    
    FragColor = texture(ourTexture, TexCoord) * ourColor * color;
}