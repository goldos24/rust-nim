proc rustFunc() {.importc.}

proc nimFunc() {.exportc.} =
    echo "nimFunc called"
    rustFunc()

echo "NimMain called"
