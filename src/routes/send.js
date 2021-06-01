function process(app) {
    app.post('/send', function (req, res) {
        common.log("🌐 Login Form", req.body.checklist.toLowerCase(), req.ip);
        res.cookie('checklist', common.makeCookie(req.body.checklist.toLowerCase()));
        res.redirect('/');
    });
}

module.exports = {
    name: 'send',
    disc: 'Send webhooks',
    process: process
}