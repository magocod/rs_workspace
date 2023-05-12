const asyncSuite1 = require('./fs_write.suite')
const oclFs = require("../index");
const path = require("path");
const main = async () => {
    oclFs.initialize();
    const toWriteTxtContent = "to_write_text_bench\n";
    const buf = Buffer.from(toWriteTxtContent);
    const filePath = path.resolve(__dirname, "../files/to_write_bench.txt");
    await asyncSuite1(buf, filePath)
    console.log(oclFs.cache());
    console.log(oclFs.oclCache());
}

main().catch((e) => {
    console.log(e)
})
