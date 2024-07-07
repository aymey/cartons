const memory = new WebAssembly.Memory({initial: 1});

const importObject = {
    host: {
        log: (offset, len) => console.log(offset, len, new TextDecoder("utf8").decode(memory.buffer.slice(offset, offset + len)))
    },

    fromjsstuff: {
        memseg1: memory
    }
}

WebAssembly.instantiateStreaming(fetch("../test.wasm"), importObject).then(
    (obj) => obj.instance.exports.hello_world()
)
