#include <stdio.h>
#include <stdlib.h>

enum HttpMethod{
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
};


struct HttpRequest{
    enum HttpMethod method;
    char* version;
    

};