#include "LEDController.hpp"

#include <algorithm>
#include <cmath>

#include <cassert>
#include <cstdlib>
#include <cstring>

LEDController::LEDController() {}

void LEDController::setup(unsigned int LEDCount)
{
    m_LEDs.resize(LEDCount);
    memset(&m_LEDs[0], 0x10, LEDCount * sizeof(cHSV));

    m_TotalPackets = std::ceil((LEDCount * sizeof(cHSV)) / (MAX_BYTES - 7.0));

    setDataBounds();
}

void LEDController::upload(SocketBase& socket)
{
    int currentByte = 0;
    uint32_t currentIndex = 0;
    constexpr int offset = 6;
    float brightness = getBrightnessFactor();

    cHSV colourHSV;
    cRGB colour;

    for (uint32_t i = 0; i < m_TotalPackets; i++)
    {
        m_DataBuffer[4] = i + 1; // Current Packet
        while (currentByte <= (MAX_BYTES - 7 - 3) || currentIndex == m_LEDs.size())
        {
            colourHSV = m_LEDs[currentIndex];
            // colourHSV.h = getHueFromPalette(m_CurrentPalette, m_LEDs[currentIndex]);
            colour = colourHSV;

            m_DataBuffer[offset + currentByte++] = colour.r * brightness;
            m_DataBuffer[offset + currentByte++] = colour.g * brightness;
            m_DataBuffer[offset + currentByte++] = colour.b * brightness;

            currentIndex++;
        }
        setDataSize(currentByte);
        currentByte = 0;
        socket.sendData(m_DataBuffer, MAX_BYTES);
    }
}

void LEDController::setLED(int index, const cHSV& led)
{
    if (index >= 0 && index < m_LEDs.size()) m_LEDs[index] = led;
}

cHSV& LEDController::getLED(int index)
{
    assert(index >= 0 && index < m_LEDs.size());

    return m_LEDs[index];
}

cHSV LEDController::getLEDWBrightness(int index)
{
    cHSV c = m_LEDs[index];
    c.v *= getBrightnessFactor();
    return c;
    // return m_LEDs[index] / getBrightnessFactor();
}

float LEDController::getBrightnessFactor() { return (m_Brightness / 255.0f); }

void LEDController::fillSolid(cHSV colour) { std::fill(m_LEDs.begin(), m_LEDs.end(), colour); }

void LEDController::fillSolid(cHSV colour, int start, int length)
{
    auto end = std::min(m_LEDs.begin() + start + length, m_LEDs.end());
    std::fill(m_LEDs.begin() + start, end, colour);
}

void LEDController::fillRainbow(cHSV hsv, uint8_t deltaHue)
{
    for (uint32_t i = 0; i < m_LEDs.size(); i++)
    {
        uint8_t hue = hsv.H + (deltaHue * i);
        m_LEDs[i] = cHSV(hue, hsv.S, hsv.V);
    }
}

void LEDController::changeSize(uint32_t size) { m_LEDs.resize(size); }

void LEDController::setDataBounds()
{
    m_DataBuffer[0] = 0x9C;             // Start Byte
    m_DataBuffer[1] = 0xDA;             // Data
    m_DataBuffer[MAX_BYTES - 1] = 0x36; // End Byte

    m_DataBuffer[5] = m_TotalPackets; // Total Packets
}

void LEDController::setDataSize(int size)
{
    m_DataBuffer[2] = size >> 8;
    m_DataBuffer[3] = size;
}
