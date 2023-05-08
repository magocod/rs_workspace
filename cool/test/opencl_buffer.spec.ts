import { assert } from "chai";
// import { promises as fsp } from "fs";

import { DEFAULT_VECTOR_SIZE, DEFAULT_GLOBAL_ARRAY_COUNT, OclBlock } from '../index.js'
import { Buffer } from "buffer";

describe("opencl_buffer", function () {
    const data = "hello";

    let oclBlock = new OclBlock(DEFAULT_VECTOR_SIZE, DEFAULT_GLOBAL_ARRAY_COUNT);
    oclBlock.initialize();
    let index: number;

    it("initialize", function () {
        let block = new OclBlock(DEFAULT_VECTOR_SIZE, DEFAULT_GLOBAL_ARRAY_COUNT);
        block.initialize();
        let i = block.enqueueBuffer(Buffer.from("world"));
        let buffer = block.dequeueBuffer(i);
        assert.equal(buffer.toString("utf8"), "world");
    });

    it("enqueueBuffer", function () {
        const buffer = Buffer.from(data);
        index = oclBlock.enqueueBuffer(buffer);
        assert.equal(index, 0);
    });

    it("dequeueBuffer", function () {
        let buffer = oclBlock.dequeueBuffer(index);
        assert.equal(buffer.toString("utf8"), data);
    });

    // it("readFile", async function () {
    //     const readBuffer = await fsp.readFile('../cl/examples/files/package-lock.json');
    //     // const readBuffer = await fsp.readFile('../cl/examples/files/libaho.rmeta');
    //     console.log(readBuffer.length)
    //
    //     let i = oclBlock.enqueueBuffer(readBuffer);
    //     let buffer = oclBlock.dequeueBuffer(i);
    //     console.log(buffer.toString("utf8"))
    //
    //     // await fsp.writeFile("package-lock.buffer.json", buffer);
    //     // await fsp.writeFile("libaho_buffer.rmeta", buffer);
    //     assert.isNotNull(buffer);
    // });

    it("getGlobalArrays", function () {
        let globalArrays = oclBlock.getGlobalArrays();
        console.log(globalArrays);
        assert.isTrue(Array.isArray(globalArrays));
    });
});
