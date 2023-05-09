import {
    addGlobalVec,
    // showGlobalVec,
    getGlobalVec
} from "../index";
import { assert} from "chai";

describe("global_vec rust", function () {
    it("call from js", async function () {
        addGlobalVec(10);
        addGlobalVec(2)
        addGlobalVec(1)

        // showGlobalVec()

        assert.equal(getGlobalVec().toString(), [10, 2, 1].toString());
    });
});
