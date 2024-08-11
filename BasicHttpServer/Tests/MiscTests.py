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
                                            + " the result of one or more instance-manipulations applied "
                                            + "to the current instance.")
    
    expected_result3 = HttpStatusCodeDetail(409, "Conflict", "The request could not be completed " 
                                            + "because of a conflict in the request.")