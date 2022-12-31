#version 330 core
in vec4 ourColor;
in vec2 TexCoord;
out vec4 FragColor;

uniform int gradientPatternType;
uniform int gradientStepsCount;
uniform vec2 gradientOffset;
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
            phase = TexCoord.x + gradientOffset.x;
            break;
        }
        // Vertical
        case 1:
        {
            phase = TexCoord.y + gradientOffset.y;
            break;
        }
        // Radial
        case 2:
        {
            phase = length((TexCoord + gradientOffset) * 2 - 1);
            break;
        }
        // Rectangular
        case 3:
        {
            phase = max(abs((TexCoord.x + gradientOffset.x) * 2  - 1), abs((TexCoord.y + gradientOffset.y) * 2  - 1));
            break;
        }
    }

    for (int i = 0; i < gradientStepsCount - 1; i++)
    {
        color = mix(color, gradientColors[i + 1], smoothstep(gradientSteps[i], gradientSteps[i + 1], phase));
    }
    
    FragColor = texture(ourTexture, TexCoord) * ourColor * color;
}