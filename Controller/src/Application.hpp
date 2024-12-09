#ifndef APPLICATION_HPP
#define APPLICATION_HPP

#include <glm/glm.hpp>

#include "System/Socket.hpp"

#include "LED/LEDController.hpp"

class Application
{
  public:
    inline static uint8_t updateFPS = 60;

    inline static Socket m_Socket;
    inline static LEDController m_Controller;

  private:
  public:
    static void init(const char* name, glm::vec2 windowSize, const char* ip, uint32_t port);
    static void start();

  private:
    Application() = default;
    ~Application() = default;

    static void processKeys();
};

#endif
