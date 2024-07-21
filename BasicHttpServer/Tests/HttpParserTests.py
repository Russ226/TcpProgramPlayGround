import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent))

from BasicHttpServer.HttpParser import HttpParser

def test_ParseFirstLine():
    test_line1 = 'GET / HTTP/1.1'
    test_line2 = 'POST /?test=true&user=bob HTTP/1.1'
    test_line3 = 'PATCH /update?test=true&userid=80&username=larry HTTP/1.1'
    
    ##test line 1
    actual_result = HttpParser.parseFirstLine(test_line1)
    
    assert actual_result.method.lower() == 'get'
    assert actual_result.version.lower() == 'http/1.1'
    assert actual_result.route == '/'
    
    ##test line 2
    actual_result = HttpParser.parseFirstLine(test_line2)
    
    assert actual_result.method.lower() == 'post'
    assert actual_result.version.lower() == 'http/1.1'
    assert actual_result.route == '/?test=true&user=bob'
    
     ##test line 3
    actual_result = HttpParser.parseFirstLine(test_line3)
    
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
    actual_result = HttpParser.parseHeaders(test_header1)
    
    assert actual_result['User-Agent'] == expected_result1['User-Agent']
    
     #test header 2
    actual_result = HttpParser.parseHeaders(test_header2)
    
    assert actual_result['User-Agent'] == expected_result2['User-Agent']
    
     #test header 13
    actual_result = HttpParser.parseHeaders(test_header3)
    
    assert actual_result['Accept-Language'] == expected_result3['Accept-Language']
    
if __name__ == "__main__":
    test_ParseFirstLine()
    test_parseHeaders()