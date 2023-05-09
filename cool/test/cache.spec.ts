import { assert } from "chai";
// import fs from "node:fs";

describe("cache", function () {
    it("require.cache", function () {
        let v = require.cache;
        // console.log(v);

        assert.equal(3, 3);
    });
});
