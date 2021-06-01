const rateLimit = require("express-rate-limit");
const express = require('express');
const config = require('./../config/config.json')
const fs = require('fs');

const app = express();

const routes = fs.readdirSync('./routes').filter(file => file.endsWith('.js'));

for (const file of routes) {
    const command = require(`./routes/${file}`);
    const cmdName = file.split('.js')[0];
    console.log(`Loading Route '${cmdName}.js' - ${command.disc}`)
    command.process(app, config)
}

function startServer() {
    app.listen(config.server.port, config.server.ip, function () {
        console.log(`🐍 Serving http://${config.server.ip}:${config.server.port}/`);
    })
}

module.exports = {
    startServer
}