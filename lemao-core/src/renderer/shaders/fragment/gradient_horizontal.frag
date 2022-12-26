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
    float phase = TexCoord.x;
    vec4 color = mix(gradientStep0Color, gradientStep1Color, smoothstep(gradientSteps.x, gradientSteps.y, phase));
    color = mix(color, gradientStep2Color, smoothstep(gradientSteps.y, gradientSteps.z, phase));
    color = mix(color, gradientStep3Color, smoothstep(gradientSteps.z, gradientSteps.w, phase));
    
    FragColor = texture(ourTexture, TexCoord) * ourColor * color;
}