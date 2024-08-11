import os
import sys
sys.path.append(os.getcwd())

from BasicHttpServer.HttpParser.HttpResponseParser import parseFirstLine, parseHTTPResponse
from BasicHttpServer.Models.HttpResponse import HttpResponse



def test_parseResponseFirstLine():
    raw_response1: str = 'HTTP/1.1 200 OK\\r\\n'
    raw_response2: str = 'HTTP/1.1 401 Unauthorized\\r\\n'
    raw_response3: str = 'HTTP/1.1 502 Fiddler - Connection Failed\\r\\n'
    
    excepted_response1 = parseFirstLine(raw_response1)
    excepted_response2 = parseFirstLine(raw_response2)
    excepted_response3 = parseFirstLine(raw_response3)
    
    
    assert excepted_response1.version == 'HTTP/1.1'
    assert excepted_response1.statusCode == 200
    assert excepted_response1.statusName == 'OK'
    
    assert excepted_response2.version == 'HTTP/1.1'
    assert excepted_response2.statusCode == 401
    assert excepted_response2.statusName == 'Unauthorized'
    
    assert excepted_response3.version == 'HTTP/1.1'
    assert excepted_response3.statusCode == 502
    assert excepted_response3.statusName == 'Fiddler - Connection Failed'


def test_parseResponse():
    test_200_ok_json: bytes = None
    test_401_unauthorized_html: bytes = None
    test_502_connectionFailed: bytes = None
    
    actual_result1: HttpResponse = HttpResponse()
    actual_result1.version = 'HTTP/1.1'
    actual_result1.statusCode = 200
    actual_result1.statusName = 'OK'
    
    actual_result1.headers = {
        'Cache-Control': 'no-cache',
        'Pragma': 'no-cache',
        'Content-Type': 'application/json; charset=utf-8',
        'Expires': '-1',
        'Server': 'Microsoft-IIS/8.5',
        'X-AspNet-Version': '4.0.30319',
        'X-Powered-By': 'ASP.NET',
        'Date': 'Tue, 23 Jul 2024 00:22:08 GMT',
        'Content-Length': '136073'
    }
    
    actual_result2: HttpResponse = HttpResponse()
    actual_result2.version = 'HTTP/1.1'
    actual_result2.statusCode = 401
    actual_result2.statusName = 'Unauthorized'
    
    actual_result2.headers = {
        'X-Plex-Protocol': '1.0',
        'Content-Type': 'text/html',
        'Connection': 'close',
        'Content-Encoding': 'gzip',
        'X-Plex-Content-Original-Length': '193',
        'X-Plex-Content-Compressed-Length': '157',
        'Content-Length': '157',
        'Cache-Control': 'no-cache',
        'Date': 'Tue, 23 Jul 2024 00:25:19 GMT'
    }

    actual_result3: HttpResponse = HttpResponse()
    actual_result3.version = 'HTTP/1.1'
    actual_result3.statusCode = 502
    actual_result3.statusName = 'Fiddler - Connection Failed'
    
    actual_result3.headers = {
        'Date': 'Tue, 23 Jul 2024 00:25:25 GMT',
        'Content-Type': 'text/html; charset=UTF-8',
        'Connection': 'close',
        'Cache-Control': 'no-cache, must-revalidate',
        'Timestamp': '20:25:25.999',
    }
    
    with open(r'C:\Users\russ2\Desktop\TcpPrograms\BasicHttpServer\Tests\TestResponse\200_ok_json.txt', 'rb') as readfile1:
       test_200_ok_json = readfile1.read()
       
    with open(r'C:\Users\russ2\Desktop\TcpPrograms\BasicHttpServer\Tests\TestResponse\401_unauthorized_html.txt', 'rb') as readfile2:
       test_401_unauthorized_html = readfile2.read()
       
    with open(r'C:\Users\russ2\Desktop\TcpPrograms\BasicHttpServer\Tests\TestResponse\502_connectionFailed_html.txt', 'rb') as readfile3:
       test_502_connectionFailed = readfile3.read()  
       
    
       
     # test 1 test_200_ok_json
    
    expected_result1 = parseHTTPResponse(test_200_ok_json)
    
    assert actual_result1.statusName == expected_result1.statusName  
    assert actual_result1.statusCode == expected_result1.statusCode  
    assert actual_result1.version == expected_result1.version
    assert actual_result1.headers == expected_result1.headers
    #print(expected_result1.body)
    
    # test 2 test_401_unauthorized_html
    expected_result2 = parseHTTPResponse(test_401_unauthorized_html)
    
    assert actual_result2.statusName == expected_result2.statusName  
    assert actual_result2.statusCode == expected_result2.statusCode  
    assert actual_result2.version == expected_result2.version
    assert actual_result2.headers == expected_result2.headers
    #print(actual_result2.body)

    # test 3 test_502_connectionFailed
    expected_result3 = parseHTTPResponse(test_502_connectionFailed)
    
    assert actual_result3.statusName == expected_result3.statusName  
    assert actual_result3.statusCode == expected_result3.statusCode 
    assert actual_result3.version == expected_result3.version
    assert actual_result3.headers == expected_result3.headers
    #print(actual_result3.body)

if __name__ == "__main__":
    test_parseResponseFirstLine()
    test_parseResponse()