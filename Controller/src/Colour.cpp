#include "Colour.hpp"

#include <cmath>
#include <sstream>

namespace Colours {
const cRGB RGB_RED = { 255, 0, 0 };
const cRGB RGB_ORANGE = { 171, 85, 0 };
const cRGB RGB_YELLOW = { 171, 177, 0 };
const cRGB RGB_GREEN = { 0, 255, 0 };
const cRGB RGB_AQUA = { 0, 171, 85 };
const cRGB RGB_BLUE = { 0, 0, 255 };
const cRGB RGB_PURPLE = { 85, 0, 85 };
const cRGB RGB_PINK = { 170, 0, 85 };

const uint8_t HUE_RED = 0;
const uint8_t HUE_ORANGE = 32;
const uint8_t HUE_YELLOW = 64;
const uint8_t HUE_GREEN = 96;
const uint8_t HUE_AQUA = 140;
const uint8_t HUE_BLUE = 160;
const uint8_t HUE_PURPLE = 192;
const uint8_t HUE_PINK = 224;
} // namespace Colours

void RGB2HSV(const cRGB& rgb, cHSV& hsv)
{
    float Rp = rgb.r / 255.0;
    float Gp = rgb.g / 255.0;
    float Bp = rgb.b / 255.0;

    float max = std::max(Rp, Gp);
    max = std::max(max, Bp);

    float min = std::min(Rp, Gp);
    min = std::min(min, Bp);

    float delta = max - min;
    float h;

    hsv.v = 255 * max;
    if (max < 0.0001)
        hsv.s = 0;
    else
        hsv.s = 255 * (delta / max);

    h = 0;
    if (max == Rp)
        h = 60 * std::fmod(((Gp - Bp) / delta), 6.0);
    else if (max == Gp)
        h = 60 * (((Bp - Rp) / delta) + 2);
    else if (max == Bp)
        h = 60 * (((Rp - Gp) / delta) + 4);

    hsv.h = (h / 360.0) * 255;
}

void HSV2RGB_rainbow(const cHSV& hsv, cRGB& rgb)
{
    // Rewritten from FastLED hsv2rgb.cpp hsv2rgb_rainbow

    // Scale yellow up
    const uint8_t Y1 = 1;
    // Scale yellow up more
    const uint8_t Y2 = 0;

    // Scale green down
    const uint8_t G2 = 0;
    // Scale green down by what
    const uint8_t Gscale = 0;

    uint8_t hue = hsv.h;
    uint8_t sat = hsv.s;
    uint8_t val = hsv.v;

    uint8_t offset = hue & 0x1F; // 0..31

    uint8_t offset8 = offset << 3;

#define K255 255
#define K171 171
#define K170 170
#define K85 85
#define scale8(i, sc) (((uint16_t)(i) * (uint16_t)(sc)) >> 8)
#define scale8_video(i, sc) ((((int)i * (int)sc) >> 8) + ((i && sc) ? 1 : 0))
    uint8_t third = scale8(offset8, (256 / 3));

    if (!(hue & 0x80))
    { // 0xx
        if (!(hue & 0x40))
        { // 00x
            if (!(hue & 0x20))
            { // 000
                rgb.r = K255 - third;
                rgb.g = third;
                rgb.b = 0;
            }
            else
            { // 001
                if (Y1)
                {
                    rgb.r = K171;
                    rgb.g = K85 + third;
                    rgb.b = 0;
                }
                if (Y2)
                {
                    rgb.r = K170 + third;
                    uint8_t twoThirds = scale8(offset8, ((256 * 2) / 3));
                    rgb.g = K85 + twoThirds;
                    rgb.b = 0;
                }
            }
        }
        else
        { // 01x
            if (!(hue & 0x20))
            { // 010
                if (Y1)
                {
                    uint8_t twoThirds = scale8(offset8, ((256 * 2) / 3));

                    rgb.r = K171 - twoThirds;
                    rgb.g = K170 + third;
                    rgb.b = 0;
                }
                if (Y2)
                {
                    rgb.r = K255 - offset8;
                    rgb.g = K170 + third;
                    rgb.b = 0;
                }
            }
            else
            { // 011
                rgb.r = 0;
                rgb.g = K255 - third;
                rgb.b = third;
            }
        }
    }
    else
    { // 1xx
        if (!(hue & 0x40))
        { // 10X
            if (!(hue & 0x20))
            { // 100
                rgb.r = 0;
                uint8_t twothirds = scale8(offset8, ((256 * 2) / 3)); // max=170
                rgb.g = K171 - twothirds;
                rgb.b = K85 + twothirds;
            }
            else
            { // 101
                rgb.r = third;
                rgb.g = 0;
                rgb.b = K255 - third;
            }
        }
        else
        {
            if (!(hue & 0x20))
            { // 110
                rgb.r = K85 + third;
                rgb.g = 0;
                rgb.b = K171 - third;
            }
            else
            { // 111
                rgb.r = K170 + third;
                rgb.g = 0;
                rgb.b = K85 - third;
            }
        }
    }

    if (G2) rgb.g = rgb.g >> 1;
    if (Gscale) rgb.g = scale8_video(rgb.g, Gscale);

    if (sat != 255)
    {
        if (sat == 0)
        {
            rgb.r = 255;
            rgb.g = 255;
            rgb.b = 255;
        }
        else
        {
            uint8_t desat = 255 - sat;
            desat = scale8_video(desat, desat);
            uint8_t satscale = 255 - desat;

            if (rgb.r) rgb.r = scale8(rgb.r, satscale) + 1;
            if (rgb.g) rgb.g = scale8(rgb.g, satscale) + 1;
            if (rgb.b) rgb.b = scale8(rgb.b, satscale) + 1;

            uint8_t brightnessFloor = desat;
            rgb.r += brightnessFloor;
            rgb.g += brightnessFloor;
            rgb.b += brightnessFloor;
        }
    }

    if (val != 255)
    {
        val = scale8_video(val, val);
        if (val == 0)
        {
            rgb.r = 0;
            rgb.g = 0;
            rgb.b = 0;
        }
        else
        {
            if (rgb.r) rgb.r = scale8(rgb.r, val) + 1;
            if (rgb.g) rgb.g = scale8(rgb.g, val) + 1;
            if (rgb.b) rgb.b = scale8(rgb.b, val) + 1;
        }
    }

#undef scale8_video
#undef scale8
#undef K85
#undef K170
#undef K171
#undef K255
}

void HSV2RGB_spectrum(const cHSV& hsv, cRGB& rgb)
{
    uint8_t val = hsv.val;
    uint8_t sat = hsv.sat;

    uint8_t invsat = 255 - sat;
    uint8_t brightnessFloor = (val * invsat) / 256;

    uint8_t colourAmplitude = val - brightnessFloor;

    uint8_t section = hsv.hue / 0x40;
    uint8_t offset = hsv.hue % 0x40;

    uint8_t rampup = offset;
    uint8_t rampdown = (0x40 - 1) - offset;

    uint8_t rampupAmpAdj = (rampup * colourAmplitude) / (256 / 4);
    uint8_t rampdownAmpAdj = (rampdown * colourAmplitude) / (256 / 4);

    uint8_t rampupAdjWithFloor = rampupAmpAdj + brightnessFloor;
    uint8_t rampdownAdjWithFloor = rampdownAmpAdj + brightnessFloor;

    if (section)
    {
        if (section == 1)
        {
            rgb.r = brightnessFloor;
            rgb.g = rampdownAdjWithFloor;
            rgb.b = rampupAdjWithFloor;
        }
        else
        {
            rgb.r = rampupAdjWithFloor;
            rgb.g = brightnessFloor;
            rgb.b = rampdownAdjWithFloor;
        }
    }
    else
    {
        rgb.r = rampdownAdjWithFloor;
        rgb.g = rampupAdjWithFloor;
        rgb.b = brightnessFloor;
    }
}

std::string RGBToString(const cRGB& rgb)
{
    std::stringstream ss;
    ss << (int)rgb.r << " : " << (int)rgb.g << " : " << (int)rgb.b;
    return ss.str();
    // return std::format("{:d} : {:d} : {:d}", rgb.r, rgb.g, rgb.b);
}

std::string HSVToString(const cHSV& hsv)
{
    std::stringstream ss;
    ss << (int)hsv.h << " : " << (int)(100 * (hsv.s / 255.0)) << " : "
       << (int)(100 * (hsv.v / 255.0));
    return ss.str();
}
