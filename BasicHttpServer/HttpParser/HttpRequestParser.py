from typing import Dict
import traceback
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent))

from BasicHttpServer.Models.HttpRequest import HttpRequest

def parseHTTPRequest(rawRequest: bytes) -> HttpRequest:
    retObj: HttpRequest = None
    strRequest: str = ''
    #splitReq: List[bytes] = bytes.splitlines(rawRequest)
    
    requestLength: int = len(rawRequest)
    
    startByteIndex: int = 0
    endByteIndex: int = 1
    
    byteIncSize = 1
    
    counter: int = 0
    while endByteIndex < requestLength:
        
        curLine: str = ''
        
        while not curLine.endswith('\r\n'):
            curLine += rawRequest[startByteIndex : endByteIndex].decode('utf-8')
            
            startByteIndex += byteIncSize
            endByteIndex += byteIncSize
            
            if curLine.endswith('\r\n') or curLine.endswith('\r\n\r\n'):
                break
            
        strRequest += curLine
        if counter == 0:
            retObj = parseFirstLine(curLine)
            
        elif counter > 0 and not strRequest.endswith('\r\n\r\n'):
            header : Dict[str, str] = parseHeaders(curLine)
            if retObj is not None:
                retObj.appendHeader(header)
                
        elif counter > 0 and strRequest.endswith('\r\n\r\n'):
            if endByteIndex + 1 < requestLength:
                retObj.body = rawRequest[startByteIndex : requestLength]
                break
        else:
            print("I messed up")
            print(f'parsed Http request {retObj.__dict__}')
            print(f'requestLength: {requestLength} \ncounter: {counter} \nstartByteIndex: {startByteIndex} \nendByteIndex:{endByteIndex}')
            traceback.print_exc()
            break
            
               
        counter += 1
        
        
    return retObj
       
    
    


def parseFirstLine(line1: str) -> HttpRequest:
    retItem: HttpRequest = HttpRequest()
      
    line1 = line1.strip()
    line1Split = line1.split(" ")
    for linePart in line1Split:
        if linePart.lower() in ['get',  'head', 'post', 'put', 'delete', 'connect', 'options', 'trace', 'patch']:
            retItem.method = linePart
        
        elif linePart.startswith('/'):
            retItem.route = linePart
        
        elif linePart.startswith('HTTP'):
            retItem.version = linePart
        
        else:
            print("invalid part of request {}", linePart)

    return retItem
            
def parseHeaders(headersToParse: str) -> dict[str, str]:
    headersToParse = headersToParse.strip()
    splitHeader = headersToParse.split(":", maxsplit = 1)
    if len(splitHeader) == 2:
        return {splitHeader[0].strip() : splitHeader[1].strip()}
    
    return None

