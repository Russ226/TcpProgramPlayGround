import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent))

from BasicHttpServer.HttpParser import HttpRequestParser

def test_ParseFirstLine():
    test_line1 = 'GET / HTTP/1.1'
    test_line2 = 'POST /?test=true&user=bob HTTP/1.1'
    test_line3 = 'PATCH /update?test=true&userid=80&username=larry HTTP/1.1'
    
    ##test line 1
    actual_result = HttpRequestParser.parseFirstLine(test_line1)
    
    assert actual_result.method.lower() == 'get'
    assert actual_result.version.lower() == 'http/1.1'
    assert actual_result.route == '/'
    
    ##test line 2
    actual_result = HttpRequestParser.parseFirstLine(test_line2)
    
    assert actual_result.method.lower() == 'post'
    assert actual_result.version.lower() == 'http/1.1'
    assert actual_result.route == '/?test=true&user=bob'
    
     ##test line 3
    actual_result = HttpRequestParser.parseFirstLine(test_line3)
    
    assert actual_result.method.lower() == 'patch'
    assert actual_result.version.lower() == 'http/1.1'
    assert actual_result.route == '/update?test=true&userid=80&username=larry'
    
    
def test_parseHeaders():
    test_header1 = 'User-Agent: PostmanRuntime/7.26.8\r\n'
    test_header2 = 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36\r\n'
    test_header3 = 'Accept-Language: en-US,en;q=0.9\r\n\r\n'
    
    expected_result1 = {'User-Agent': 'PostmanRuntime/7.26.8'}
    expected_result2 = {'User-Agent' : 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36'}
    expected_result3 = {'Accept-Language' : 'en-US,en;q=0.9'}
    
    #test header 1
    actual_result = HttpRequestParser.parseHeaders(test_header1)
    
    assert actual_result['User-Agent'] == expected_result1['User-Agent']
    
     #test header 2
    actual_result = HttpRequestParser.parseHeaders(test_header2)
    
    assert actual_result['User-Agent'] == expected_result2['User-Agent']
    
     #test header 13
    actual_result = HttpRequestParser.parseHeaders(test_header3)
    
    assert actual_result['Accept-Language'] == expected_result3['Accept-Language']
    
def test_httpRquest():
    test_httpRquest_update_json_body: bytes = None
    test_httpRquest_multipart: bytes = None
    test_httpRquest_get: bytes = None
    test_httpRquest_delete_body: bytes = None
    
    
    expect_result1: HttpRequestParser.HTTPRequest = HttpRequestParser.HTTPRequest()
    
    expect_result1.method = 'PATCH'
    expect_result1.route = '/update?test=true&userid=80&username=larry'
    expect_result1.version = 'HTTP/1.1'
    
    expect_result1.headers = {
        'User-Agent': 'PostmanRuntime/7.26.8',
        'Content-Type': 'application/json',
        'Accept': '*/*',
        'Cache-Control': 'no-cache',
        'Postman-Token': '4139c842-fda7-48fb-8d2b-40d2ac079993',
        'Host': '192.168.1.146:8080',
        'Accept-Encoding': 'gzip, deflate, br',
        'Connection': 'keep-alive',
        'Content-Length': '60'
    }
    
    expect_result2: HttpRequestParser.HTTPRequest = HttpRequestParser.HTTPRequest()
    
    expect_result2.method = 'GET'
    expect_result2.route = '/update?test=true&userid=80&username=larry'
    expect_result2.version = 'HTTP/1.1'
    
    expect_result2.headers = {
        'User-Agent': 'PostmanRuntime/7.26.8',
        'Content-Type': 'multipart/form-data; boundary=--------------------------824477221437051630641179',
        'Accept': '*/*',
        'Cache-Control': 'no-cache',
        'Postman-Token': 'b542d5b1-6d56-4851-9395-1e2f9eb4bdc0',
        'Host': '192.168.1.146:8080',
        'Accept-Encoding': 'gzip, deflate, br',
        'Connection': 'keep-alive',
        'Content-Length': '39358'
    }
    
    expect_result3: HttpRequestParser.HTTPRequest = HttpRequestParser.HTTPRequest()
    
    expect_result3.method = 'GET'
    expect_result3.route = '/'
    expect_result3.version = 'HTTP/1.1'
    
    expect_result3.headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36',
        'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7',
        'Accept-Encoding': 'gzip, deflate',
        'Cache-Control': 'max-age=0',
        'Upgrade-Insecure-Requests': '1',
        'Host': '192.168.1.146:8080',
        'Connection': 'keep-alive',
        'Accept-Language': 'en-US,en;q=0.9'
    }
    
    expect_result4: HttpRequestParser.HTTPRequest = HttpRequestParser.HTTPRequest()
    
    expect_result4.method = 'DELETE'
    expect_result4.route = '/delete'
    expect_result4.version = 'HTTP/1.1'
    
    expect_result4.headers = {
        'User-Agent': 'PostmanRuntime/7.26.8',
        'Content-Type': 'application/json',
        'Accept': '*/*',
        'Cache-Control': 'no-cache',
        'Postman-Token': 'e6fa2b02-d6ba-4fe0-960e-2d1708d620ae',
        'Host': '192.168.1.146:8080',
        'Accept-Encoding': 'gzip, deflate, br',
        'Connection': 'keep-alive',
        'Content-Length': '61'
    }
    
    with open(r'C:\Users\russ2\Desktop\TcpPrograms\BasicHttpServer\Tests\TestRequests\testfile2.txt', 'rb') as readfile1:
       test_httpRquest_update_json_body = readfile1.read()
       
    with open(r'C:\Users\russ2\Desktop\TcpPrograms\BasicHttpServer\Tests\TestRequests\testfile4.txt', 'rb') as readfile2:
       test_httpRquest_multipart = readfile2.read()
       
    with open(r'C:\Users\russ2\Desktop\TcpPrograms\BasicHttpServer\Tests\TestRequests\testfile15.txt', 'rb') as readfile3:
       test_httpRquest_get = readfile3.read()  
       
    with open(r'C:\Users\russ2\Desktop\TcpPrograms\BasicHttpServer\Tests\TestRequests\testfile27.txt', 'rb') as readfile4:
       test_httpRquest_delete_body = readfile4.read()
       
    
    # test 1 test_httpRquest_update_json_body
    actual_result1 = HttpRequestParser.parseHTTPRequest(test_httpRquest_update_json_body)
    
    assert actual_result1.method == expect_result1.method  
    assert actual_result1.route == expect_result1.route  
    assert actual_result1.version == expect_result1.version
    assert actual_result1.headers == expect_result1.headers
    #print(actual_result1.body)
    
    # test 2 test_httpRquest_multipart
    actual_result2 = HttpRequestParser.parseHTTPRequest(test_httpRquest_multipart)
    
    assert actual_result2.method == expect_result2.method  
    assert actual_result2.route == expect_result2.route  
    assert actual_result2.version == expect_result2.version
    assert actual_result2.headers == expect_result2.headers
    #print(actual_result2.body)

    # test 3 test_httpRquest_multipart
    actual_result3 = HttpRequestParser.parseHTTPRequest(test_httpRquest_get)
    
    assert actual_result3.method == expect_result3.method  
    assert actual_result3.route == expect_result3.route  
    assert actual_result3.version == expect_result3.version
    assert actual_result3.headers == expect_result3.headers
    #print(actual_result3.body)

    
    # test 4 test_httpRquest_multipart
    actual_result4 = HttpRequestParser.parseHTTPRequest(test_httpRquest_delete_body)
    
    assert actual_result4.method == expect_result4.method  
    assert actual_result4.route == expect_result4.route  
    assert actual_result4.version == expect_result4.version
    assert actual_result4.headers == expect_result4.headers
    #print(actual_result4.body)

      
       
    
        
    
if __name__ == "__main__":
    test_ParseFirstLine()
    test_parseHeaders()
    test_httpRquest()