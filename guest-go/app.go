package main

import (
	"fmt"
	"io"
	"net"

	"go.bytecodealliance.org/cm"

	"demo/internal/app/demo/worker"
)

func tcpChat(addr string, request []byte) ([]byte, error) {
	fmt.Println("wasm guest-go[tcp_chat]")

	conn, err := net.Dial("tcp", addr)
	if err != nil {
		return nil, err
	}

	_, err = conn.Write(request)
	if err != nil {
		return nil, err
	}

	fmt.Printf("wasm guest-go[tcp_chat]: sent %d bytes to %s\n", len(request), addr)

	response, err := io.ReadAll(conn)

	fmt.Printf("wasm guest-go[tcp_chat]: received %d bytes from %s\n", len(response), addr)

	return response, nil
}

type RES = cm.Result[cm.List[uint8], cm.List[uint8], string]

func init() {
	worker.Exports.TCPChat = func(addr string, request cm.List[uint8]) RES {
		response, err := tcpChat(addr, request.Slice())
		if err != nil {
			return cm.Err[RES](err.Error())
		}

		return cm.OK[RES](cm.ToList(response))
	}
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
