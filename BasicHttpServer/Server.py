import socket
import traceback
import threading

from HttpParser.HttpRequestParser import parseHTTPRequest
from HttpHandler.HttpHandler import HttpHandler

class MyHTTPServer:
             
    def run(self):
        sock  = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        sock.bind(("192.168.1.146", 8080))
        sock.listen(1)
        self.counter: int = 0
        print("listening on 192.168.1.146:8080")
        # to do set time create thread with 15 sec timeout
        while True:
            conn = None
            
            try:
                conn , addr = sock.accept()
                
                print("\n")
                print(f'connection recieved from {addr}:\n', )
                
                t: threading.Thread = threading.Thread(target=self.recieveRequest, args=(conn,))
                t.daemon = True
                t.start()
                
            except TimeoutError:
                pass
            except Exception:
                traceback.print_exc() 
            finally:
                self.counter += 1
            

    def recieveRequest(self, conn: socket.socket) -> None:
        req:bytes = bytes()
        
        try:
            req += conn.recv(1024)
            
            print(req, '\n')
            parsedReq = parseHTTPRequest(req)
            
            if 'Content-Length' in parsedReq.headers:
                if parsedReq.body is not None:
                    parsedReq.body += conn.recv(int(parsedReq.headers["Content-Length"]))
                    
                else:
                    parsedReq.body = conn.recv(int(parsedReq.headers["Content-Length"]))
        
            handler = HttpHandler(parsedReq)
            resp = handler.HandleRequest()
            
            conn.send(resp.createRawResponse())
            if resp.body is not None:
                conn.send(resp.body)
           
        except TimeoutError:
                pass
        except Exception:
            traceback.print_exc() 
        finally:
            # with open(f'C:\\Users\\russ2\\Desktop\\TcpPrograms\\BasicHttpServer\\Tests\\TestRequests\\testfile{self.counter}.txt', 'wb') as rightToFile:
            #     rightToFile.write(req) 
            
            conn.close() 



if __name__ == "__main__":
    httpServer = MyHTTPServer()
    httpServer.run()