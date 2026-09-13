#version 100
#define MAX_LIGHTS 32

precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

uniform float aspectRatio;
uniform int lightsCount;
uniform vec2 lightPositions[MAX_LIGHTS];
uniform vec4 lightColors[MAX_LIGHTS];

const float distortionAmount = 0.09;

vec2 crtDistort(vec2 uv, float distortion) {
    vec2 center = uv - vec2(0.5);
    float dist = dot(center, center);

    return uv + center * dist * distortion;
}

void main() {
    vec2 crtUV = crtDistort(vec2(uv.x, 1.0 - uv.y), distortionAmount);

    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    vec4 baseColor = texture2D(Texture, crtUV) * color;

    vec3 totalLighting = vec3(0.75);

    for (int i = 0; i < MAX_LIGHTS; i++) {
        if (i >= lightsCount)
            break;

        vec2 lightPos = lightPositions[i];

        vec3 lightColor = lightColors[i].rgb;
        float radius = lightColors[i].a;

        vec2 lightDelta = crtUV - lightPos;

        lightDelta.x *= aspectRatio;

        float dist = length(lightDelta);

        float attenuation = clamp(1.0 - dist / radius, 0.0, 1.0);

        attenuation = pow(attenuation, 2.0);
        totalLighting += lightColor * attenuation * 0.85;
    }

    float scanline = sin(crtUV.y * 600.0) * 0.012;

    vec2 centeredUV = crtUV - vec2(0.5);

    float vignette = 1.0 - dot(centeredUV, centeredUV) * 0.35;

    vignette = clamp(vignette, 0.0, 1.0);

    vec3 finalRGB =
    baseColor.rgb * totalLighting;

    finalRGB -= scanline;
    finalRGB *= vignette;

    finalRGB = min(finalRGB, baseColor.rgb);

    gl_FragColor = vec4(finalRGB, baseColor.a);
}
