import demo
import demo.exports
import urllib3


class Worker(demo.exports.Worker):

    def http_fetch(self, url: str) -> str:
        print(f"wasm guest-python http-fetch: {url}")
        return urllib3.request("GET", url).data.decode()
