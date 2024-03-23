from requests_rust import send_request






if __name__ == '__main__':
    res = send_request('get', 'http://127.0.0.1:3000/', 0)
    print(res)
