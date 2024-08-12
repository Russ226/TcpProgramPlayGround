import os 
import sys

sys.path.append(os.getcwd())

from BasicHttpServer.Constants.HttpStatus import GetHttpStatusCode
from BasicHttpServer.Models.HttpStatusCodeDetail import HttpStatusCodeDetail

def test_StatusCodesGetter():
    expected_result1 = HttpStatusCodeDetail(100, "Continue", "The server has received the request " 
                                           + "headers, and the client should proceed "
                                           + "to send the request body."
                        )
    
    expected_result2 = HttpStatusCodeDetail(226, "IM Used" ,"The server has fulfilled a request for " 
                                            + "the resource, and the response is a representation of "
                                            + "the result of one or more instance-manipulations applied "
                                            + "to the current instance.")
    
    expected_result3 = HttpStatusCodeDetail(409, "Conflict", "The request could not be completed " 
                                            + "because of a conflict in the request.")
    
    actual_result1 = GetHttpStatusCode(100)
    actual_result2 = GetHttpStatusCode(226)
    actual_result3 = GetHttpStatusCode(409)
    try:
        GetHttpStatusCode(44)
    except Exception as e:
        assert str(e) == 'code 44 is not a valid a http status code'
    finally:
        pass
        
    
    assert actual_result1.code == expected_result1.code
    assert actual_result1.message == expected_result1.message
    assert actual_result1.description == expected_result1.description
    
    
    assert actual_result2.code == expected_result2.code
    assert actual_result2.message == expected_result2.message
    assert actual_result2.description == expected_result2.description
    
    assert actual_result3.code == expected_result3.code
    assert actual_result3.message == expected_result3.message
    assert actual_result3.description == expected_result3.description
    
if __name__ == "__main__":
    test_StatusCodesGetter()