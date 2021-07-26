const config = require('./../config/config.json');
const rateLimit = require("express-rate-limit");
const express = require('express');
const fs = require('fs');

const app = express();
app.use(rateLimit(config.server.rateLimit));
app.use(express.json());

// Load routes
const routes = fs.readdirSync('./src/routes').filter(file => file.endsWith('.js'));

for (const file of routes) {
    const command = require(`./routes/${file}`);
    const cmdName = file.split('.js')[0];
    console.log(`📂 Loading '${cmdName}.js' - ${command.disc}`)
    command.process(app, config)
}

// Start server with no tls :'(
function startServer() {
    app.listen(config.server.port, config.server.ip, () => {
        console.log(`🐍 Serving http://${config.server.ip}:${config.server.port}/`);
    })
}

module.exports = {
    startServer
}