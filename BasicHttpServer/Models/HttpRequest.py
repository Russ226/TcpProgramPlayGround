from typing import Dict

class HTTPRequest:
   
    def __init__(self) -> None:
        self.route: str = ''
        self.method: str = ''
        self.version: str = ''
        self.headers: Dict[str, str] =  {}
        self.body: bytes = None
        
    def appendHeader(self, header: Dict[str, str]):
        keys =list(header.keys())
        if header is not None and keys[0] not in self.headers:
            self.headers[keys[0]] = header[keys[0]]
        