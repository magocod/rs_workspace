import { assert } from "chai";
// import { promises as fsp } from "fs";

import { OclBlock } from '../index.js'
import { Buffer } from "buffer";

describe("opencl_buffer", function () {
    const data = "hello";

    let oclBlock = new OclBlock();
    oclBlock.initialize();
    let index: number;

    it("initialize", function () {
        let block = new OclBlock();
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
});
