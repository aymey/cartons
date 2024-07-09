window.onload = async function a() {
    /** @type HTMLCanvasElement **/
    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");

    let ent = await (await fetch("/entity")).json();
    ent.pos.x *= canvas.width;
    ent.pos.y = canvas.height*(1 - ent.pos.y);
    ent.rad *= canvas.width;

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.strokeRect(1, 1, canvas.width - 1, canvas.height - 1);

    ctx.beginPath();
    ctx.arc(ent.pos.x, ent.pos.y, ent.rad * 10, 0, 2*Math.PI, true);
    ctx.closePath();
    ctx.fill();

    setTimeout(a, 2);
}

const memory = new WebAssembly.Memory({initial: 1});

const importObject = {
    host: {
        log: (offset, len) => console.log(offset, len, new TextDecoder("utf8").decode(memory.buffer.slice(offset, offset + len)))
    },

    fromjsstuff: {
        memseg1: memory
    }
}

WebAssembly.instantiateStreaming(fetch("test.wasm"), importObject).then(
    (obj) => obj.instance.exports.hello_world()
)
