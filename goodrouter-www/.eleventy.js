const path = require("path");

module.exports = function (eleventyConfig) {
  eleventyConfig.addPassthroughCopy({
    "../assets/": "/",
    "./src/styles": "/",
  });

  return {
    dir: {
      input: "./src/content",
      includes: "../includes",
      data: "../data",
      output: "./out",
    },
  };
};
