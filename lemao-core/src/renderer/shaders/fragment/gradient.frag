#version 330 core
in vec4 ourColor;
in vec2 TexCoord;
out vec4 FragColor;

uniform int gradientPatternType;
uniform int gradientStepsCount;
uniform float gradientSteps[16];
uniform vec4 gradientColors[16];

uniform sampler2D ourTexture;

void main()
{
    float phase = 0.0;
    vec4 color = gradientColors[0];

    switch (gradientPatternType)
    {
        // Horizontal
        case 0:
        {
            phase = TexCoord.x;
            break;
        }
        // Vertical
        case 1:
        {
            phase = TexCoord.y;
            break;
        }
        // Radial
        case 2:
        {
            phase = length(TexCoord * 2 - 1);
            break;
        }
        // Rectangular
        case 3:
        {
            phase = max(abs(TexCoord.x * 2  - 1), abs(TexCoord.y * 2  - 1));
            break;
        }
    }

    for (int i = 0; i < gradientStepsCount - 1; i++)
    {
        color = mix(color, gradientColors[i + 1], smoothstep(gradientSteps[i], gradientSteps[i + 1], phase));
    }
    
    FragColor = texture(ourTexture, TexCoord) * ourColor * color;
}