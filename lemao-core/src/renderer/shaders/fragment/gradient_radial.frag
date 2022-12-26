#version 330 core
in vec4 ourColor;
in vec2 TexCoord;
out vec4 FragColor;

uniform int gradientStepsCount;
uniform float gradientSteps[16];
uniform vec4 gradientColors[16];

uniform sampler2D ourTexture;

void main()
{
    float phase = length(TexCoord * 2 - 1);
    vec4 color = gradientColors[0];

    for (int i = 0; i < gradientStepsCount - 1; i++)
    {
        color = mix(color, gradientColors[i + 1], smoothstep(gradientSteps[i], gradientSteps[i + 1], phase));
    }
    
    FragColor = texture(ourTexture, TexCoord) * ourColor * color;
}