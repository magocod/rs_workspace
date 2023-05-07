import { assert } from "chai";

import { sum } from '../index.js'

describe("index", function () {
    it("sum from native", function () {
        assert.equal(sum(1, 2), 3);
    });
});
