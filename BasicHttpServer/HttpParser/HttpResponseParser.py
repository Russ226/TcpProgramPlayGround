from typing import Dict
import traceback
import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent.parent))

from BasicHttpServer.Models.HttpResponse import HttpResponse


            
def parseHTTPResponse(rawRequest: bytes) -> HttpResponse:
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
    
    retObj: HttpResponse = HttpResponse()
    parsedLine: str = ''  
    
    sizeLine = len(line)
    cursor: int = 0
    
    while not parsedLine.endswith('\\r\\n') and cursor < sizeLine:
        section: str = ''
        
        while not section.endswith(' ') and cursor < sizeLine:
            section += line[cursor]
            cursor += 1
        
        parsedLine += section
        section = section.strip()
        postStatusCode = isStatusCode(section)
        
        if section.lower().startswith('http'):
            retObj.version = section
        
        elif postStatusCode != -999:
            retObj.statusCode = postStatusCode
            retObj.statusName = line[cursor: len(line) - (len('\\r\\n') if line.endswith('\\r\\n') else len('\r\n'))]
            break

    return retObj
            
def isStatusCode(toParse: str) -> int:
    failCode = -999
    try:
        tempInt = int(toParse)
        
        if tempInt > 100 and tempInt < 600:
            return tempInt
        
        return failCode
    except Exception:
        return failCode
                       
            
           
            
def parseHeaders(headersToParse: str) -> dict[str, str]:
    headersToParse = headersToParse.strip()
    splitHeader = headersToParse.split(":", maxsplit = 1)
    if len(splitHeader) == 2:
        return {splitHeader[0].strip() : splitHeader[1].strip()}
    
    return None