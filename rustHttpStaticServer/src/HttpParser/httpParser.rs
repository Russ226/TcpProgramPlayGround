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
}

struct HttpRequest{
    method: HttpMethod,
    version:  &str,
    route:  &str,
    headers: HashMap<&str, &str>
}

fn getHttpMethod(m: &str) -> HttpMethod {
    return match m{
        "GET" => HttpMethod.GET,
        "HEAD" => HttpMethod.HEAD,
        "POST" => HttpMethod.POST,
        "PUT" => HttpMethod.PUT,
        "DELETE" => HttpMethod.DELETE,
        "OPTIONS" => HttpMethod.OPTIONS,
        "TRACE" => HttpMethod.TRACE,
        "PATCH" => HttpMethod.PATCH
    }
}

fn parseFirstLine(line: &str) -> HttpRequest{
    let method: HttpMethod;
    let version: &str;
    let route:  &str;

}