# Wasm fetch sample

This repository is a sample that fetches data within WebAssembly and returns response data to the JS side.

## usage

```
wasm-pack build --target web
```

```
python3 example/server.py
```

Access localhost:8888 with a browser.
You will see alert window and response data.