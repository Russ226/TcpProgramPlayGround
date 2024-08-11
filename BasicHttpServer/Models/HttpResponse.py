from typing import Dict

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