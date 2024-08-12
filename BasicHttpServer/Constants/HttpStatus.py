import os
import sys
import json
sys.path.append(os.getcwd())

from BasicHttpServer.Models.HttpStatusCodeDetail import HttpStatusCodeDetail
from BasicHttpServer.Constants.Exceptions.HttpStatusNotFound import HttpStatusNotFound
httpStatusJsonPath: str = 'BasicHttpServer\Constants\Httpstatuses.json'

def GetHttpStatusCode(code) -> HttpStatusCodeDetail:
    rootdir = os.getcwd()
    
    str_code = str(code)
    
    with open(os.path.join(rootdir, httpStatusJsonPath), 'r') as fileToRead:
        rawCodes = fileToRead.read()
        t = json.loads(rawCodes)
        
        if str(str_code) in t:
            return HttpStatusCodeDetail(t[str_code]["code"], t[str_code]["message"], t[str_code]["description"])
    
    raise HttpStatusNotFound(code)
        
        