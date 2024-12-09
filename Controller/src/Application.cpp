#include "Application.hpp"

#include <chrono>

void Application::init(const char* name, glm::vec2 windowSize, const char* ip, uint32_t port)
{
    m_Socket.resetIp(ip, port);

    m_Controller.setup(10);

    m_Controller.setBrightness(255);
}

void Application::start()
{
    auto previousTime = std::chrono::steady_clock::now();
    while (true)
    {
        std::chrono::time_point currentTime = std::chrono::steady_clock::now();

        auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(
            std::chrono::duration(currentTime - previousTime));

        if (duration.count() < (1000. / 60.))
        {
            continue;
        }

        // Change time
        m_Controller.fillRainbow({ 10 }, 5);
        m_Controller.upload(m_Socket);
    }
}

void Application::processKeys() {}
