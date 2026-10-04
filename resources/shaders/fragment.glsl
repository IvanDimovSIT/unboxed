#version 100
#define MAX_LIGHTS 32

precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

uniform mediump float screenWidth;
uniform mediump float screenHeight;
uniform int lightsCount;
uniform vec2 lightPositions[MAX_LIGHTS];
uniform vec4 lightColors[MAX_LIGHTS];

const float distortionAmount = 0.025;

const float haloRadiusMultiplier = 1.49;
const float haloStrength = 0.12;

const float bloomStrength = 0.5;
const float bloomThreshold = 0.5;
const float bloomRadius = 4.0;

vec2 crtDistort(vec2 uv, float distortion) {
    vec2 center = uv - vec2(0.5);
    float dist = dot(center, center);

    return uv + center * dist * distortion;
}

vec3 sampleBloom(vec2 uv) {
    vec2 texel = vec2(1.0 / screenWidth, 1.0 / screenHeight);

    vec3 bloom = vec3(0.0);

    vec3 c = texture2D(Texture, uv).rgb;
    float brightness = dot(c, vec3(0.2126, 0.7152, 0.0722));
    float centerFactor = max(brightness - bloomThreshold, 0.0);

    bloom += c * centerFactor * 0.25;

    for (int i = 1; i <= 3; i++) {
        float offset = float(i) * bloomRadius;

        vec3 left = texture2D(Texture, uv - vec2(texel.x * offset, 0.0)).rgb;
        vec3 right = texture2D(Texture, uv + vec2(texel.x * offset, 0.0)).rgb;

        float leftBrightness = dot(left, vec3(0.2126, 0.7152, 0.0722));
        float rightBrightness = dot(right, vec3(0.2126, 0.7152, 0.0722));

        float leftFactor = max(leftBrightness - bloomThreshold, 0.0);
        float rightFactor = max(rightBrightness - bloomThreshold, 0.0);

        bloom += left * leftFactor * (0.12 / float(i));
        bloom += right * rightFactor * (0.12 / float(i));
    }

    for (int i = 1; i <= 3; i++) {
        float offset = float(i) * bloomRadius;

        vec3 up = texture2D(Texture, uv - vec2(0.0, texel.y * offset)).rgb;
        vec3 down = texture2D(Texture, uv + vec2(0.0, texel.y * offset)).rgb;

        float upBrightness = dot(up, vec3(0.2126, 0.7152, 0.0722));
        float downBrightness = dot(down, vec3(0.2126, 0.7152, 0.0722));

        float upFactor = max(upBrightness - bloomThreshold, 0.0);
        float downFactor = max(downBrightness - bloomThreshold, 0.0);

        bloom += up * upFactor * (0.12 / float(i));
        bloom += down * downFactor * (0.12 / float(i));
    }

    return bloom * bloomStrength;
}

void main() {
    vec2 crtUV = crtDistort(vec2(uv.x, 1.0 - uv.y), distortionAmount);

    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    vec4 baseColor = texture2D(Texture, crtUV) * color;

    vec3 totalLighting = vec3(0.90);
    vec3 totalHalo = vec3(0.0);

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
        float distance01 = dist / max(radius, 0.0001);

        float attenuation = 1.0 - clamp(distance01, 0.0, 1.0);
        attenuation *= attenuation;
        attenuation *= 0.75 + 0.25 * attenuation;

        totalLighting += lightColor * attenuation * 0.75;

        float haloDistance = distance01 / haloRadiusMultiplier;
        float halo = 1.0 - smoothstep(0.0, 1.0, haloDistance);
        halo *= halo;

        float centerFade = smoothstep(0.15, 0.65, distance01);

        halo *= mix(0.35, 1.0, centerFade);

        totalHalo += lightColor * halo * haloStrength;
    }

    vec3 litRGB = baseColor.rgb * totalLighting;

    vec3 haloRGB = totalHalo * (vec3(1.0) - baseColor.rgb);
    litRGB += haloRGB;

    // Keep brightness above 1.0 available for bloom.
    litRGB = min(litRGB, vec3(2.0));

    float scanlineWave = sin(crtUV.y * 600.0);
    float scanline = 0.5 + 0.5 * scanlineWave;

    float scanlineBrightness = mix(0.82, 1.0, scanline);

    float fineScanline = 0.5 + 0.5 * sin(crtUV.y * 1200.0);

    scanlineBrightness *= mix(0.96, 1.0, fineScanline);

    vec2 centeredUV = crtUV - vec2(0.5);

    float vignette = 1.0 - dot(centeredUV, centeredUV) * 0.22;

    vignette = clamp(vignette, 0.0, 1.0);

    vec3 finalRGB = litRGB;

    finalRGB += sampleBloom(crtUV);

    finalRGB *= scanlineBrightness;
    finalRGB *= vignette;

    finalRGB = clamp(finalRGB, 0.0, 1.0);

    gl_FragColor = vec4(finalRGB, baseColor.a);
}
