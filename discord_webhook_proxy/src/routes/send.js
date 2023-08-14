const axios = require('axios').default;

function process(app, config) {
    app.post('/send', function (req, res) {
        axios.post('/user', { })
            .then(function (r) {
                console.log(r);
            })
            .catch(function (e) {
                console.log(e);
            });

        res.send({ response: 'success' })
        //res.status(503).send({response: 'error', error: '???'})
    });

    app.post('/send/basic', (req, res) => {
        axios.post(config.hook.url, {
            content: req.body.content || 'NULL',
        })
        res.send({ response: 'success' })
        //res.status(503).send({response: 'error', error: '???'})
    });
}

module.exports = {
    disc: 'Send webhooks',
    process: process
}