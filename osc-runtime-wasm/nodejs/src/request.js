const request = require('superagent')

var url = 'http://127.0.0.1:6106'
var config = {
  user: 'username',
  password: 'password'
}

module.exports = {
  computeResourceCreate: function(body) {
    request.post(url + '/v1alpha/compute_resources')
      .auth(config.user, config.password)
      .send(body)
      .end(function(err, res) {
        if (err) throw err;
        console.log(res.body)
      })
  },
  computeResourceDelete: function(cr_id, cloud_provider, opts) {
    request.delete(url + '/v1alpha/compute_resources/' + cr_id + '?cloud_provider=' + cloud_provider + '&delete_publicip=' + opts.delete_publicip + '&delete_volume=' + opts.delete_volume)
      .auth(config.user, config.password)
      .end(function(err, res) {
        if (err) throw err;
        console.log(res.body)
      })
  },
  storageResourceCreate: function(body) {
    request.post(url + '/v1alpha/storage_resources')
      .auth(config.user, config.password)
      .send(body)
      .end(function(err, res) {
        if (err) throw err;
        console.log(res.body)
      })
  },
  storageResourceDelete: function(sr_id, cloud_provider) {
    request.delete(url + '/v1alpha/storage_resources/' + sr_id + '?cloud_provider=' + cloud_provider)
      .auth(config.user, config.password)
      .end(function(err, res) {
        if (err) throw err;
        console.log(res.body)
      })
  },
  networkResourceCreate: function(body) {
    request.post(url + '/v1alpha/network_resources')
      .auth(config.user, config.password)
      .send(body)
      .end(function(err, res) {
        if (err) throw err;
        console.log(res.body)
      })
  },
  networkResourceDelete: function(nr_id, cloud_provider) {
    request.delete(url + '/v1alpha/network_resources/' + nr_id + '?cloud_provider=' + cloud_provider)
      .auth(config.user, config.password)
      .end(function(err, res) {
        if (err) throw err;
        console.log(res.body)
      })
  }
}
