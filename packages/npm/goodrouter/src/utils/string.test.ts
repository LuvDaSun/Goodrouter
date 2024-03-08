import assert from "node:assert/strict";
import test from "node:test";
import { findCommonPrefixLength } from "./string.js";

test("find-common-prefix-length", () => {
  assert.equal(findCommonPrefixLength("ab", "abc"), 2);

  assert.equal(findCommonPrefixLength("abc", "abc"), 3);

  assert.equal(findCommonPrefixLength("bc", "abc"), 0);
});
