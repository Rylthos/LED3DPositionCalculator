#ifndef SYSTEM_SOCKET_HPP
#define SYSTEM_SOCKET_HPP

#include "SocketBase.hpp"

#if defined(_WIN32)
    #include "Windows/Socket.hpp"
#elif defined(__linux__)
    #include "Linux/Socket.hpp"
#endif

#endif
