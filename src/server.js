const rateLimit = require("express-rate-limit");
const express = require('express');
const config = require('./../config/config.json')
const fs = require('fs');

const app = express();

function startServer() {
    app.listen(config.server.port, config.server.ip, function () {
        console.log(`🐍 Serving http://${config.server.ip}:${config.server.port}/`);
    })
}

module.exports = {
    startServer
}