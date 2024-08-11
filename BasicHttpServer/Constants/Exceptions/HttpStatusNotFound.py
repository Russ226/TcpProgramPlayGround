class HttpStatusNotFound(Exception):
    def __init__(self, code):
        super().__init__(f'code {code} is not a valid a http status code')