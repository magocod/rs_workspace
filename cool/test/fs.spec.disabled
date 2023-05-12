import { assert } from "chai";
import path from "path";
import fs from "fs";
import { promises as fsp } from "fs";
import * as fsOcl from "../index";

describe("fs", function () {
    const toReadTxtContent = "to_read_text\n";
    const toWriteTxtContent = "to_write_text\n";

    const readFilePath = path.resolve(__dirname, "../files/to_read.txt");

    describe("read", function () {
        before(function () {
            fsOcl.writeFileSync(readFilePath, Buffer.from(toReadTxtContent));
        })

        it("readFileSync", function () {
            const buffer = fs.readFileSync(readFilePath);
            // let arr = Uint8Array.from(buffer);
            // console.log(arr);
            const bufferOcl = fsOcl.readFileSynchronous(readFilePath);

            assert.equal(buffer.toString("utf8"), toReadTxtContent);
            assert.equal(bufferOcl.toString("utf8"), toReadTxtContent);
        });

        it("readFileSync not exist", function () {
            let jsError  = "";
            let napiError = "";

            try {
                fs.readFileSync("/dir/not_exit");
                // @ts-ignore
            } catch (e: Error) {
                // console.log(e)
                jsError = e.message;
            }

            try {
                fsOcl.readFileSynchronous("/dir/not_exit");
                // @ts-ignore
            } catch (e: Error) {
                // console.log(e)
                napiError = e.message
            }

            assert.equal(jsError, napiError);
        });

        // it("readFile", async function () {
        //     const filePath = path.resolve(__dirname, "../files/to_read.txt");
        //     const buffer = await fsp.readFile(filePath);
        //     // let arr = Uint8Array.from(buffer);
        //     // console.log(arr);
        //
        //     assert.equal(buffer.toString("utf8"), toReadTxtContent);
        // });
    })

    describe("write", function () {
        it("writeFileSync",function () {
            const filePath = path.resolve(__dirname, "../files/to_write.txt");
            fs.writeFileSync(filePath, toWriteTxtContent);
            fsOcl.writeFileSync(filePath, Buffer.from(toWriteTxtContent));

            console.log(fsOcl.cache())

            assert.isTrue(fs.existsSync(filePath));
            assert.isTrue(fsOcl.existsSynchronous(filePath));
        });

        // it("writeFile", async function () {
        //     const filePath = path.resolve(__dirname, "../files/to_write.txt");
        //     await fsp.writeFile(filePath, toWriteTxtContent);
        //
        //     assert.isTrue(fs.existsSync(filePath));
        // });
    })

    describe("fileHandle", function () {
        const filePath = path.resolve(__dirname, "../files/open_r_plus.txt");

        before(function () {
            fsOcl.writeFileSync(filePath, Buffer.from(""));
        })

        it("open async", async function () {
            const strA = "hello";
            const strB = "world";

            // ocl

            const oclFileHandle = fsOcl.FileHandle.open(filePath);
            console.log(oclFileHandle.fd())

            // const f1 = fsOcl.FileHandle.open(filePath);
            // console.log(f.fd())
            await oclFileHandle.writeFile(Buffer.from(`${strA}${strB}`))

            const oclReadResult = await oclFileHandle.readFile()

            // node

            let fileHandle = await fsp.open(filePath, 'r+');
            // console.log(fileHandle.fd)

            // let fileHandle2 = await fsp.open(filePath, 'r+');
            // console.log(fileHandle2.fd)

            await fileHandle.write(strA)
            await fileHandle.write(strB)
            await fileHandle.close();

            fileHandle = await fsp.open(filePath, 'r');
            const readResult = await fileHandle.readFile()

            console.log(fsOcl.cache())
            console.log(fsOcl.oclCache())

            // console.log(readResult.toString('utf8'))
            // console.log(oclReadResult.toString('utf8'))

            assert.equal(readResult.toString('utf8'), `${strA}${strB}`);
            assert.equal(oclReadResult.toString('utf8'), `${strA}${strB}`);
        });
    })
});
