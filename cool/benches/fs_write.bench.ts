import { Bench } from 'tinybench';
import fs from "fs";
import path from "path";
import * as oclFs from "../index";

const bench = new Bench({ time: 500 });
const toWriteTxtContent = "to_write_text_bench\n";
const buf = Buffer.from(toWriteTxtContent);
const filePath = path.resolve(__dirname, "../files/to_write_bench.txt");

bench
    .add('fs.writeFileSync', () => {
        fs.writeFileSync(filePath, buf);
    })
    .add('fsOcl.writeFileSync', async () => {
        oclFs.writeFileSync(filePath, buf)
    })
    .todo('unimplemented bench')

async function start() {
    oclFs.initialize();
    await bench.run();
    console.table(bench.table());
    console.log(oclFs.cache());
    console.log(oclFs.oclCache());
}

start().catch((e) => {
    console.log(e);
})

