import demo
import demo.exports
import socket


class Worker(demo.exports.Worker):

    def tcp_chat(self, addr: str, request: bytes) -> bytes:
        print("wasm guest-python[tcp_chat]")

        strs = addr.split(":", maxsplit=1)
        if len(strs) != 2:
            return b"invalid tcp address format"

        host = strs[0]
        port = int(strs[1])

        with socket.socket() as client:
            client.connect((host, port))

            client.sendall(request)
            print(f"wasm guest-python[tcp_chat]: sent {len(request)} bytes to {addr}")  # noqa: E501

            response = client.recv(1024)
            print(f"wasm guest-python[tcp_chat]: received {len(response)} bytes from {addr}")  # noqa: E501

            return response
