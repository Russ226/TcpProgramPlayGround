class HttpStatusCodeDetail:
    def __init__(self, code: int, message: str, desc: str) -> None:
        self.code = code
        self.message = message
        self.description = desc