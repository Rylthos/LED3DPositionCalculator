#ifndef SOCKET_BASE_HPP
#define SOCKET_BASE_HPP

#include <cstdint>

class SocketBase
{
public:
    SocketBase() {}
    SocketBase(const char* ip, int port) {}
    virtual ~SocketBase() {}

    virtual void resetIp(const char* ip, int port) {}
    virtual void sendData(uint8_t* buffer, int size) {}
};

#endif
