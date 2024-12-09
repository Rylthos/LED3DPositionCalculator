#if defined(__linux__)
#include "Socket.hpp"

#include <cstdio>

Socket::Socket() {}
Socket::Socket(const char* ip, int port) {}
Socket::~Socket() {}

void Socket::resetIp(const char* ip, int port) {}
void Socket::sendData(uint8_t* buffer, int size)
{
    for (int i = 0; i < size; i++)
    {
        printf("%x", buffer[i]);
    }
    printf("\n");
}
#endif
