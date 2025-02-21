let subpath = require('./subpath');
let port = require('./port');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const path = require('path');

module.exports = {
    mode: "production",
    // devtool: 'inline-source-map',
    entry: {
        app: "./src/index.ts"
    }/* ,
    devServer: {
        static: './dist',
        open: {
            app: {
                name: 'chrome'
            }
        },
        compress: true,
        port: port,
        hot: true
    } */,
    module: {
        exprContextCritical: false, // Disables the critical dependency warning
        rules: [
            {
                test: /\.s[ac]ss$/i,
                use: [
                    // Creates `style` nodes from JS strings
                    MiniCssExtractPlugin.loader,
                    // Translates CSS into CommonJS
                    "css-loader",
                    {
                        // Run postcss actions
                        loader: 'postcss-loader',
                        options: {
                            // `postcssOptions` is needed for postcss 8.x;
                            // if you use postcss 7.x skip the key
                            postcssOptions: {
                                // postcss plugins, can be exported to postcss.config.js
                                plugins: function () {
                                    return [
                                        require('autoprefixer')
                                    ];
                                }
                            }
                        }
                    },
                    // Compiles Sass to CSS
                    {
                        loader: "sass-loader",
                        options: {
                            implementation: require.resolve("sass"),
                            sourceMap: true
                        }
                    },
                ],
            },
            {
                test: /\.html$/i,
                loader: "html-loader",
                options: {
                    esModule: false,
                    minimize: true
                },
            },
            {
                test: /\.tsx?$/,
                exclude: /(node_modules|\.webpack)/,
                use: {
                    loader: 'ts-loader',
                    options: {
                        transpileOnly: true
                    }
                }
            }
        ],
    },
    plugins: [
        new ForkTsCheckerWebpackPlugin(),
        new HtmlWebpackPlugin({
            title: "Электрондук күндөлүк",
            template: path.resolve(__dirname + '/' + subpath + '/src', 'index.html')
        }),
        new MiniCssExtractPlugin()
    ],
    resolve: {
        extensions: ['.js', '.ts', '.jsx', '.tsx', '.css']
    },
    output: {
        path: path.resolve(__dirname + '/' + subpath + '/', 'dist'),
        filename: '[name].bundle.js',
        clean: true
    }
}