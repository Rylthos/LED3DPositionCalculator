#if defined(__linux__)
#ifndef LINUX_SOCKET_HPP
#define LINUX_SOCKET_HPP

#include "../SocketBase.hpp"

class Socket : public SocketBase
{
public:
    Socket();
    Socket(const char* ip, int port);
    ~Socket() override;

    void resetIp(const char* ip, int port) override;
    void sendData(uint8_t* buffer, int size) override;
};

#endif
#endif
