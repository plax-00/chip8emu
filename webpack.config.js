const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
	mode: "production",
	entry: {
		index: "./js/bootstrap.js",
	},
	output: {
		path: dist,
		filename: "bootstrap.js",
	},
	devServer: {
		contentBase: dist,
	},
	plugins: [
		new CopyPlugin([
			path.resolve(__dirname, "static")
		]),
	],
};
