"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var express_1 = require("express");
var app;
var PORT = 5007;
app = (0, express_1.default)();
// start the server
app.listen(PORT, function () {
    console.log("Knight marching on path 0.0.0.0:{PORT}");
});
