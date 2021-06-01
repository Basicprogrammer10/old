const axios = require('axios').default;

function process(app, config) {
    app.post('/send', function (req, res) {
        axios.post('/user', {
            firstName: 'Fred',
            lastName: 'Flintstone'
          })
          .then(function (response) {
            console.log(response);
          })
          .catch(function (error) {
            console.log(error);
          });
        
        res.send({response: 'success'})
        //res.status(503).send({response: 'error', error: '???'})
    });
}

module.exports = {
    name: 'send',
    disc: 'Send webhooks',
    process: process
}