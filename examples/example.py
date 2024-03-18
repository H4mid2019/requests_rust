from http_requests import send_request
res = send_request('GET', 'http://127.0.0.1:3000/')
print(res)
