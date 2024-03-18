from http_requests import send_request
res = send_request('get', 'http://127.0.0.1:3000/', 0)
print(res)
