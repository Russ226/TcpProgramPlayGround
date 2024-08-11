import os 
import sys
from pathlib import Path
sys.path.append(os.getcwd())

from BasicHttpServer.Models.HttpRequest import HttpRequest
from BasicHttpServer.Models.HttpResponse import HttpResponse

class HttpHandler:
    def __init__(self, req: HttpRequest) -> None:
        self.request: HttpRequest = req
        self.workingDir = os.getcwd()
    
    # handle get requets first
    # ignore queryparams
    def HandleRequest(self) -> HttpResponse:
        if self.request.route == '/':
            resp = HttpResponse()
            indexHtml = os.path.join(self.workingDir, 'index.html')
            
            if os.path.exists(indexHtml):
                pass
            