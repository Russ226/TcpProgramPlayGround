import os 
import sys
from pathlib import Path
sys.path.append(os.getcwd())

from BasicHttpServer.Models.HttpRequest import HttpRequest
from BasicHttpServer.Models.HttpResponse import HttpResponse

CONTENTTYPES = {
    'html': 'text/html',
    'css': 'text/css',
    'js': 'text/js',
    'png': 'image/png',
    'jpeg': 'image/jpeg',
    'tiff': 'image/tiff'
    
}

class HttpHandler:
    def __init__(self, req: HttpRequest) -> None:
        self.request: HttpRequest = req
        self.workingDir = os.getcwd()
    
    # handle get requets first
    # ignore queryparams
    def HandleRequest(self) -> HttpResponse:
        if self.request.route == '/':
            indexHtml = os.path.join(self.workingDir, 'index.html')
            
            if os.path.exists(indexHtml):
                with open(indexHtml, 'rb') as readFile:
                    retBody = readFile.read()
                    
                    return HttpResponse.createHttpResponse(code = 200, body = retBody, contentType = 'text/html')
                
        if os.path.isdir(os.path.join(self.workingDir, self.request.route)):
            indexHtml = os.path.join(self.workingDir, self.request.route, 'index.html')
            
            if os.path.exists(indexHtml):
                with open(indexHtml, 'rb') as readFile:
                    retBody = readFile.read()
                    
                    return HttpResponse.createHttpResponse(code = 200, body = retBody, contentType = 'text/html')
                
        filepath = self.workingDir + self.request.route.replace('/', '\\')  
        if os.path.isfile(filepath) and os.path.exists(filepath):
            
            with open(filepath, 'rb') as readFile:
                retBody = readFile.read()
                filetype = filepath[-3:]
                
                if filetype in CONTENTTYPES:
                    return HttpResponse.createHttpResponse(code = 200, body = retBody, contentType = CONTENTTYPES[filetype])
                
                return HttpResponse.createHttpResponse(code = 200, body = retBody)
            
        return HttpResponse.createHttpResponse(code = 404)
            