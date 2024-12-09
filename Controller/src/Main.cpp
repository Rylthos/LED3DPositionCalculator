// #define STB_IMAGE_IMPLEMENTATION
#include "Application.hpp"
static const unsigned int SCREEN_WIDTH = 1280;
static const unsigned int SCREEN_HEIGHT = 720;

int main()
{
    // srand(1);

    Application::init("LED Controller", { SCREEN_WIDTH, SCREEN_HEIGHT }, "192.168.0.99", 65506);
    Application::start();
}
