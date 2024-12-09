#if defined(_WIN32)
#ifndef WINDOWS_SOCKET_HPP
#define WINDOWS_SOCKET_HPP

#include "../SocketBase.hpp"

#include <winsock2.h>
#include <ws2tcpip.h>
#include <iostream>

#pragma comment(lib, "Ws2_32.lib")

class Socket : public SocketBase
{
public:
    sockaddr_in m_Server;
    SOCKET m_Socket;
    WSADATA m_Data;
private:
public:
    Socket();
    Socket(const char* ip, int port);
    ~Socket() override;

    void resetIp(const char* ip, int port) override;

    void sendData(uint8_t* buffer, int size) override;
private:
    void createSocket(const char* ip, int port);
    void setupWSA();
};

#endif
#endif
