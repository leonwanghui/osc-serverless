const request = require('supertest')

var url = 'http://127.0.0.1:6106'
var config = {
    user: 'username',
    password: 'password'
}

module.exports = {
    computeResourceCreate: function(body) {
        request(url).post('/v1alpha/compute_resources')
            .auth(config.user, config.password)
            .send(body)
            .expect(201)
            .end(function(err, res) {
                if (err) throw err;
                console.log(res.body)
            })
    },
    computeResourceDelete: function(cr_id, cloud_provider, opts) {
        request(url).post('/v1alpha/compute_resources/' + cr_id + '?cloud_provider=' + cloud_provider + '&delete_publicip=' + opts.delete_publicip + '&delete_volume=' + opts.delete_volume)
            .auth(config.user, config.password)
            .expect(200)
            .end(function(err, res) {
                if (err) throw err;
                console.log(res.body)
            })
    },
    storageResourceCreate: function(body) {
        request(url).post('/v1alpha/storage_resources')
            .auth(config.user, config.password)
            .send(body)
            .expect(201)
            .end(function(err, res) {
                if (err) throw err;
                console.log(res.body)
            })
    },
    storageResourceDelete: function(sr_id, cloud_provider) {
        request(url).post('/v1alpha/storage_resources/' + sr_id + '?cloud_provider=' + cloud_provider)
            .auth(config.user, config.password)
            .expect(200)
            .end(function(err, res) {
                if (err) throw err;
                console.log(res.body)
            })
    },
    networkResourceCreate: function(body) {
        request(url).post('/v1alpha/network_resources')
            .auth(config.user, config.password)
            .send(body)
            .expect(201)
            .end(function(err, res) {
                if (err) throw err;
                console.log(res.body)
            })
    },
    networkResourceDelete: function(nr_id, cloud_provider) {
        request(url).post('/v1alpha/network_resources/' + nr_id + '?cloud_provider=' + cloud_provider)
            .auth(config.user, config.password)
            .expect(200)
            .end(function(err, res) {
                if (err) throw err;
                console.log(res.body)
            })
    }
}