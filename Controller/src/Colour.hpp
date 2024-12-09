#ifndef COLOUR_HPP
#define COLOUR_HPP

#include <cstdint>
#include <cstdlib>
#include <string>

struct cRGB;
struct cHSV;

void RGB2HSV(const cRGB& rgb, cHSV& hsv);
void HSV2RGB_rainbow(const cHSV& hsv, cRGB& rgb);
void HSV2RGB_spectrum(const cHSV& hsv, cRGB& rgb);

std::string RGBToString(const cRGB& rgb);
std::string HSVToString(const cHSV& hsv);

struct cHSV {
    union {
        struct {
            union {
                uint8_t hue;
                uint8_t H;
                uint8_t h;
            };
            union {
                uint8_t sat;
                uint8_t S;
                uint8_t s;
            };
            union {
                uint8_t val;
                uint8_t V;
                uint8_t v;
            };
        };
        uint8_t raw[3];
    };

    inline uint8_t& operator[](uint8_t x) { return raw[x]; }
    inline const uint8_t& operator[](uint8_t x) const { return raw[x]; }

    inline cHSV() = default;
    inline cHSV(uint8_t hue, uint8_t sat, uint8_t val) : h(hue), s(sat), v(val) {}
    inline cHSV(uint8_t hue) : h(hue), s(0x00), v(0x00) {}

    // Copy Constructors
    inline cHSV(const cHSV& rhs) = default;
    inline cHSV& operator=(const cHSV& rhs) = default;
    inline cHSV(const cRGB& rhs) { RGB2HSV(rhs, *this); }
    inline cHSV& operator=(const cRGB& rhs)
    {
        RGB2HSV(rhs, *this);
        return *this;
    }

    inline cHSV& setHSV(uint8_t hue, uint8_t sat, uint8_t val)
    {
        h = hue;
        s = sat;
        v = val;
        return *this;
    }

    inline cHSV& operator+=(const uint8_t hue)
    {
        h += hue;
        return *this;
    }

    inline cHSV operator+(const uint8_t hue)
    {
        cHSV result = *this;
        result += hue;
        return result;
    }
};

struct cRGB {
    union {
        struct {
            union {
                uint8_t red;
                uint8_t R;
                uint8_t r;
            };
            union {
                uint8_t green;
                uint8_t G;
                uint8_t g;
            };
            union {
                uint8_t blue;
                uint8_t B;
                uint8_t b;
            };
        };

        uint8_t raw[3];
    };

    inline uint8_t getHue()
    {
        cHSV hsv;
        RGB2HSV(*this, hsv);
        return hsv.hue;
    }

    inline uint8_t& operator[](uint8_t x) { return raw[x]; }
    inline const uint8_t& operator[](uint8_t x) const { return raw[x]; }

    inline cRGB() = default;
    inline cRGB(uint8_t red, uint8_t green, uint8_t blue) : r(red), g(green), b(blue) {}
    inline cRGB(uint8_t value) : r(value), g(value), b(value) {}

    inline cRGB(const cRGB& rhs) = default;
    inline cRGB& operator=(const cRGB& rhs) = default;

    inline cRGB(const cHSV& rhs) { HSV2RGB_rainbow(rhs, *this); }
    inline cRGB& operator=(const cHSV& rhs)
    {
        HSV2RGB_rainbow(rhs, *this);
        return *this;
    }

    inline cRGB(const uint32_t& code) : r(code >> 16), g(code >> 8), b(code >> 0) {}
    inline cRGB& operator=(const uint32_t& code)
    {
        r = code >> 16;
        g = code >> 8;
        b = code >> 0;
        return *this;
    }

    inline cRGB& setRGB(uint8_t red, uint8_t green, uint8_t blue)
    {
        r = red;
        g = green;
        b = blue;
        return *this;
    }

    inline cRGB& setHSV(uint8_t hue, uint8_t sat, uint8_t val)
    {
        HSV2RGB_rainbow(cHSV(hue, sat, val), *this);
        return *this;
    }

    inline cRGB& setCode(uint32_t code)
    {
        r = code >> 16;
        g = code >> 8;
        b = code >> 0;
        return *this;
    }
};

inline cRGB operator*(cRGB& rgb, float mul)
{
    cRGB rgb2;
    rgb2.r = rgb.r * mul;
    rgb2.g = rgb.g * mul;
    rgb2.b = rgb.b * mul;
    return rgb2;
}

inline cRGB operator/(cRGB& rgb, float div)
{
    cRGB rgb2;
    rgb2.r = rgb.r / div;
    rgb2.g = rgb.g / div;
    rgb2.b = rgb.b / div;
    return rgb2;
}

namespace Colours {
extern const cRGB RGB_RED;
extern const cRGB RGB_ORANGE;
extern const cRGB RGB_YELLOW;
extern const cRGB RGB_GREEN;
extern const cRGB RGB_AQUA;
extern const cRGB RGB_BLUE;
extern const cRGB RGB_PURPLE;
extern const cRGB RGB_PINK;

extern const uint8_t HUE_RED;
extern const uint8_t HUE_ORANGE;
extern const uint8_t HUE_YELLOW;
extern const uint8_t HUE_GREEN;
extern const uint8_t HUE_AQUA;
extern const uint8_t HUE_BLUE;
extern const uint8_t HUE_PURPLE;
extern const uint8_t HUE_PINK;
} // namespace Colours

#endif
