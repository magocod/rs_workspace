import { assert } from "chai";
import { promises as fsp } from "fs";

describe("read_file", function () {
    it("readFile", async function () {
        const data = await fsp.readFile('../cl/examples/files/info.txt');
        let buf = Uint8Array.from(data);
        // console.log(buf);

        assert.isNotNull(buf);
    });
});
