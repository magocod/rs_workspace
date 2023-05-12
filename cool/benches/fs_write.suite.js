const b = require('benny')
const fs = require("fs");
const oclFs = require("../index");

module.exports = (buf, filePath) => b.suite(
    'fs',

    b.add('fs.writeFileSync', () => {
        fs.writeFileSync(filePath, buf);
    }),

    b.add('oclFs.writeFileSync', () => {
        oclFs.writeFileSync(filePath, buf)
    }),

    b.cycle(),
    b.complete(),
    b.save({ file: 'fs', version: '1.0.0' }),
    b.save({ file: 'fs', format: 'chart.html' }),
)
