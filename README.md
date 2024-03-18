# requests_rust 
**__HTTP/s requests for python with rust__**

## Description

it's a POC for a library in Python that sends requests and gets JSON responses written in Rust.

## Features

- written in rust
- only works for sending and receiving JSON requests
- always returns a JSON, no need to convert by using the JSON method
- it's a POC

## Installation

1. by running `pip install requests-rust`
2. download from actions artifacts latest one
3. clone it and build using maturin (cargo)

## Usage

By default, it returns JSON, if the server doesn't respond the status code will be 0, it only throws an error if the delete method doesn't work.

```
from http_requests import send_request
res = send_request('GET', 'http://127.0.0.1:3000/')
print(res)
```

it supports these methods:  `GET`, `POST`, `PATCH`, `PUT`, `DELETE`

## Contributing

Contributions are welcome! Please follow the guidelines in [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

For any questions or feedback, please reach out to [hamid@darabi.website](mailto:hamid@darabi.website).