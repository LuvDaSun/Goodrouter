import assert from "node:assert/strict";
import test from "node:test";
import { defaultRouterOptions } from "./router-options.js";
import { parseTemplatePairs, parseTemplateParts } from "./template.js";

test("parse-template-parts", () => {
    {
        const parts = [
            ...parseTemplateParts(
                "/a/{b}/{c}",
                defaultRouterOptions.parameterPlaceholderRE,
            ),
        ];

        assert.deepEqual(parts, ["/a/", "b", "/", "c", ""]);
    }

    {
        const parts = [
            ...parseTemplateParts(
                "/a/{b}/{c}/",
                defaultRouterOptions.parameterPlaceholderRE,
            ),
        ];

        assert.deepEqual(parts, ["/a/", "b", "/", "c", "/"]);
    }

    {
        const parts = [
            ...parseTemplateParts(
                "",
                defaultRouterOptions.parameterPlaceholderRE,
            ),
        ];

        assert.deepEqual(parts, [""]);
    }
});

test("parse-template-pairs", () => {
    {
        const parts = [
            ...parseTemplatePairs(
                "/a/{b}/{c}",
                defaultRouterOptions.parameterPlaceholderRE,
            ),
        ];

        assert.deepEqual(parts, [
            ["/a/", null],
            ["/", "b"],
            ["", "c"],
        ]);
    }

    {
        const parts = [
            ...parseTemplatePairs(
                "/a/{b}/{c}/",
                defaultRouterOptions.parameterPlaceholderRE,
            ),
        ];

        assert.deepEqual(parts, [
            ["/a/", null],
            ["/", "b"],
            ["/", "c"],
        ]);
    }

    {
        const parts = [
            ...parseTemplatePairs(
                "",
                defaultRouterOptions.parameterPlaceholderRE,
            ),
        ];

        assert.deepEqual(parts, [["", null]]);
    }
});
