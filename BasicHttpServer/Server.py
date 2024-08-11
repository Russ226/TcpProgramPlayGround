import socket
import traceback
import threading

from HttpParser.HttpRequestParser import parseHTTPRequest

class MyHTTPServer:
             
    def run(self):
        sock  = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(2)
        sock.bind(("192.168.1.146", 8080))
        sock.listen(1)
        self.counter: int = 0
        print("listening on 192.168.1.146:8080")
        # to do set time create thread with 15 sec timeout
        while True:
            conn = None
            
            try:
                conn , addr = sock.accept()
                conn.settimeout(2)
                
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
            

    def recieveRequest(self, conn: socket) -> None:
        req:bytes = bytes()
        curByteLength = 0
        prevByteLength = 0
        try:
            while True:
                more: bytes = conn.recv(1024)
            
                curByteLength = len(more)
                if more:
                    req += more
                else:
                    break
               
                if prevByteLength > curByteLength:
                    break
                
                prevByteLength = curByteLength
        except TimeoutError:
                pass
        except Exception:
            traceback.print_exc() 
        finally:
            # with open(f'C:\\Users\\russ2\\Desktop\\TcpPrograms\\BasicHttpServer\\Tests\\TestRequests\\testfile{self.counter}.txt', 'wb') as rightToFile:
            #     rightToFile.write(req) 
            print(req, '\n')
            parsedReq = parseHTTPRequest(req)
            print(parsedReq.__dict__)
            conn.close() 



if __name__ == "__main__":
    httpServer = MyHTTPServer()
    httpServer.run()