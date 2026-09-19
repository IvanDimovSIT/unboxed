#version 100
#define MAX_LIGHTS 32

precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

uniform float screenWidth;
uniform float screenHeight;
uniform int lightsCount;
uniform vec2 lightPositions[MAX_LIGHTS];
uniform vec4 lightColors[MAX_LIGHTS];

const float distortionAmount = 0.09;

const float blurCenterWeight = 0.6;
const float blurCardinalWeight = 0.075;
const float blurDiagonalWeight = 0.025;

vec2 crtDistort(vec2 uv, float distortion) {
    vec2 center = uv - vec2(0.5);
    float dist = dot(center, center);

    return uv + center * dist * distortion;
}

vec4 blurTexture(vec2 uv, vec2 texelSize) {
    vec4 result = vec4(0.0);

    result += texture2D(Texture, uv) * blurCenterWeight;

    result += texture2D(Texture, uv + vec2(texelSize.x, 0.0)) * blurCardinalWeight;
    result += texture2D(Texture, uv + vec2(-texelSize.x, 0.0)) * blurCardinalWeight;
    result += texture2D(Texture, uv + vec2(0.0, texelSize.y)) * blurCardinalWeight;
    result += texture2D(Texture, uv + vec2(0.0, -texelSize.y)) * blurCardinalWeight;

    result += texture2D(Texture, uv + texelSize) * blurDiagonalWeight;
    result += texture2D(Texture, uv - texelSize) * blurDiagonalWeight;
    result += texture2D(Texture, uv + texelSize) * blurDiagonalWeight;
    result += texture2D(Texture, uv + texelSize) * blurDiagonalWeight;

    return result;
}

void main() {
    vec2 crtUV = crtDistort(vec2(uv.x, 1.0 - uv.y), distortionAmount);

    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    vec2 texelSize = vec2(1.0 / screenWidth, 1.0 / screenHeight);
    vec4 baseColor = blurTexture(crtUV, texelSize) * color;

    vec3 totalLighting = vec3(0.75);

    float aspectRatio = screenWidth / screenHeight;

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
