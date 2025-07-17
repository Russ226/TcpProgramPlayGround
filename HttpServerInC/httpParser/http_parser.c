#include <stdio.h>
#include <stdlib.h>

struct HttpHeader{
    char *name;
    char *value;
};


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
    struct HttpHeader *httpHeaders;
    size_t httpHeadersSize;
    unsigned char *body 
};

struct HttpRequest* parse_first_line(){

}