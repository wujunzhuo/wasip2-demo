package main

import (
	"fmt"
	"io"
	"net/http"

	"go.bytecodealliance.org/cm"

	"demo/internal/app/demo/worker"
)

func httpFetch(url string) (string, error) {
	fmt.Println("wasm guest-go http-fetch:", url)

	resp, err := http.Get(url)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	response, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}

	return string(response), nil
}

func init() {
	type Res = cm.Result[string, string, string]

	worker.Exports.HTTPFetch = func(url string) Res {
		response, err := httpFetch(url)
		if err != nil {
			return cm.Err[Res](err.Error())
		}

		return cm.OK[Res](response)
	}
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
