async function httpFetch(url) {
    console.log("wasm guest-js http-fetch:", url)

    var response = await fetch(url)
    if (!response.ok) {
        throw new Error('response error')
    }

    return await response.text()
}

export const worker = {
    httpFetch,
};
