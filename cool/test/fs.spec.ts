import { assert } from "chai";
import { promises as fsp } from "fs";
import path from "path";
import fs from "fs";

describe("fs", function () {
    const toReadTxtContent = "to_read_text\n";
    const toWriteTxtContent = "to_write_text\n";

    it("readFile", async function () {
        const filePath = path.resolve(__dirname, "../files/to_read.txt");
        const buffer = await fsp.readFile(filePath);
        // let arr = Uint8Array.from(buffer);
        // console.log(arr);

        assert.equal(buffer.toString("utf8"), toReadTxtContent);
    });

    it("open", async function () {
        const filePath = path.resolve(__dirname, "../files/open_r_plus.txt");
        const strA = "hello";
        const strB = "world";

        let fileHandle = await fsp.open(filePath, 'r+');
        await fileHandle.write(strA)
        await fileHandle.write(strB)
        await fileHandle.close();

        fileHandle = await fsp.open(filePath, 'r');
        const readResult = await fileHandle.readFile()
        // console.log(readResult.toString('utf8'))

        assert.equal(readResult.toString('utf8'), `${strA}${strB}`);
    });

    it("writeFileSync",function () {
        const filePath = path.resolve(__dirname, "../files/to_write.txt");
        fs.writeFileSync(filePath, toWriteTxtContent);

        assert.isTrue(fs.existsSync(filePath));
    });

    // it("writeFile", async function () {
    //     const filePath = path.resolve(__dirname, "../files/to_write.txt");
    //     await fsp.writeFile(filePath, toWriteTxtContent);
    //
    //     assert.isTrue(fs.existsSync(filePath));
    // });
});
