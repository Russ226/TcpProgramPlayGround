from typing import Dict
from datetime import datetime 
import os 
import sys

sys.path.append(os.getcwd())

from BasicHttpServer.Constants.HttpStatus import GetHttpStatusCode

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
            
    def createRawResponse(self) -> bytes:
        strrep = f'{self.version} {self.statusCode} {self.statusName}\\r\\n'
        
        for header in self.headers:
            key = [*header]
            strrep += f'{key[0]}: {header[key[0]]}\\r\\n'
        
        strrep += '\\r\\n'
        if self.body is not None:
            return strrep.encode() + self.body
        
        return strrep.encode()            
    
    @staticmethod        
    def createHttpResponse(code: int, headers: Dict[str, str] = {}, body: bytes = None, contentType: str = ''):
        retResponse = HttpResponse()
        retResponse.version = 'HTTP/1.1'
        
        statusCode = GetHttpStatusCode(code)
        retResponse.statusCode = statusCode.code
        retResponse.statusName = statusCode.message
        
        if 'Cache-Control' not in headers:
            retResponse.appendHeader({'Cache-Control': 'no-cache'})
        
        if 'Date' not in headers:
            retResponse.appendHeader({'Date': f'{datetime.now}'})
            
        if 'Content-Type' not in headers and len(contentType) > 0:
            retResponse.appendHeader({'Content-Type': contentType})
            
        if 'Content-Length' not in headers and body is not None:
            retResponse.appendHeader({'Content-Length': len(body)})
            
        for header in headers:
            retResponse.appendHeader(header)
            
        
        
        return retResponse