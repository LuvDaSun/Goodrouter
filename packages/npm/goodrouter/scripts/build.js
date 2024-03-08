#!/usr/bin/env node

import cp from "child_process";
import path from "path";

cp.execSync(["tsc"].join(" "));

cp.execSync(
  [
    "rollup",
    "--input",
    path.resolve("transpiled", "main.js"),
    "--file",
    path.resolve("bundled", "main.js"),
    "--sourcemap",
    "--format",
    "es",
  ].join(" "),
  {},
);

cp.execSync(
  [
    "rollup",
    "--input",
    path.resolve("transpiled", "main.js"),
    "--file",
    path.resolve("bundled", "main.cjs"),
    "--sourcemap",
    "--format",
    "cjs",
  ].join(" "),
);
