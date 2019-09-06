import * as request from 'superagent';

let config = {
  user: 'username',
  password: 'password'
};

/**
 * @param {any} body
 * @returns {none}
 */
export function compute_resource_create(body) {
  request.post('/v1alpha/compute_resources')
    .auth(config.user, config.password)
    .send(body)
    .set('Content-Type', 'application/json')
    .set('Accept', 'application/json')
    .end(function(err, res) {
      if (err || !res.ok) throw err;
      console.log(res.body)
    })
};

/**
 * @param {string} cr_id
 * @param {string} cloud_provider
 * @param {any} opts
 * @returns {none}
 */
export function compute_resource_delete(cr_id, cloud_provider, opts) {
  request.delete('/v1alpha/compute_resources/' + cr_id + '?cloud_provider=' + cloud_provider + '&delete_publicip=' + opts.delete_publicip + '&delete_volume=' + opts.delete_volume)
    .auth(config.user, config.password)
    .set('Accept', 'application/json')
    .end(function(err, res) {
      if (err) throw err;
      console.log(res.body)
    })
};

/**
 * @param {any} body
 * @returns {none}
 */
export function storage_resource_create(body) {
  request.post('/v1alpha/storage_resources')
    .auth(config.user, config.password)
    .send(body)
    .set('Content-Type', 'application/json')
    .set('Accept', 'application/json')
    .end(function(err, res) {
      if (err) throw err;
      console.log(res.body)
    })
};

/**
 * @param {string} sr_id
 * @param {string} cloud_provider
 * @returns {none}
 */
export function storage_resource_delete(sr_id, cloud_provider) {
  request.delete('/v1alpha/storage_resources/' + sr_id + '?cloud_provider=' + cloud_provider)
    .auth(config.user, config.password)
    .set('Accept', 'application/json')
    .end(function(err, res) {
      if (err) throw err;
      console.log(res.body)
    })
};

/**
 * @param {any} body
 * @returns {none}
 */
export function network_resource_create(body) {
  request.post('/v1alpha/network_resources')
    .auth(config.user, config.password)
    .send(body)
    .set('Content-Type', 'application/json')
    .set('Accept', 'application/json')
    .end(function(err, res) {
      if (err) throw err;
      console.log(res.body)
    })
};

/**
 * @param {string} nr_id
 * @param {string} cloud_provider
 * @returns {none}
 */
export function network_resource_delete(nr_id, cloud_provider) {
  request.post('/v1alpha/network_resources/' + nr_id + '?cloud_provider=' + cloud_provider)
    .auth(config.user, config.password)
    .set('Accept', 'application/json')
    .end(function(err, res) {
      if (err) throw err;
      console.log(res.body)
    })
};
