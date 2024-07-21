from typing import List, Dict
import traceback

class HTTPRequest:
   
    def __init__(self, route: str = '', method:str = '', version:str = '' , headers: List[Dict[str, str]]  = {}, body: bytes = None ) -> None:
        self.route = route
        self.method = method
        self.version = version
        self.headers =  headers
        self.body = body
        
    def appendHeader(self, header: Dict[str, str]):
        if header is not None:
            self.headers.append(header)
        
def parseHTTPRequest(rawRequest: bytes) -> HTTPRequest:
    retObj: HTTPRequest = None
    strRequest: str = ''
    #splitReq: List[bytes] = bytes.splitlines(rawRequest)
    
    requestLength: int = len(rawRequest)
    
    startByteIndex: int = 0
    endByteIndex: int = 2
    
    byteIncSize = 2
    
    counter: int = 0
    while endByteIndex < requestLength:
        
        curLine: str = ''
        
        while not curLine.endswith('\r\n'):
            curLine += rawRequest[startByteIndex : endByteIndex].decode('utf-8')
            
            startByteIndex += byteIncSize
            endByteIndex += byteIncSize
            
        if counter == 0:
            retObj = parseFirstLine(curLine)
            
        elif counter > 0 and not strRequest.endswith('\r\n\r\n'):
            header : Dict[str, str] = parseHeaders(curLine)
            if retObj is not None:
                retObj.appendHeader(header)
                
        elif counter > 0 and strRequest.endswith('\r\n\r\n'):
            if endByteIndex + 1 < requestLength:
                retObj.body = rawRequest[startByteIndex : requestLength - 1]
        else:
            print("I messed up")
            traceback.print_exc()
            break
            
               
        strRequest += curLine
        counter += 1
        
        
    return retObj
       
    
    


def parseFirstLine(line1: str) -> HTTPRequest:
    retItem: HTTPRequest = HTTPRequest()
    
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

