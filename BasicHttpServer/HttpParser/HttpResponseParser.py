from typing import Dict
import traceback


class HttpResponse:
    def __init__(self):
        self.version:str = ''
        self.statusCode:int = 0
        self.statusName: str = ''
        self.headers: Dict[str, str] = {}
        self.body: bytes =  None
        
    def appendHeader(self, header: Dict[str, str]):
        keys =list(header.keys())
        if header is not None and keys[0] not in self.headers:
            self.headers[keys[0]] = header[keys[0]]
            
def parseHTTPRequest(rawRequest: bytes) -> HttpResponse:
    retObj: HttpResponse = None
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

def parseFirstLine(line: str) -> HttpResponse:
    pass      
            
def parseHeaders(headersToParse: str) -> dict[str, str]:
    headersToParse = headersToParse.strip()
    splitHeader = headersToParse.split(":", maxsplit = 1)
    if len(splitHeader) == 2:
        return {splitHeader[0].strip() : splitHeader[1].strip()}
    
    return None