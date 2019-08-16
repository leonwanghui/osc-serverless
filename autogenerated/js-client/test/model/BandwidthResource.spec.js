/**
 * Open Service Cloud API
 * Open Service Cloud API to manage different backend cloud services.
 *
 * The version of the OpenAPI document: 0.0.3
 * Contact: wanghui71leon@gmail.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 *
 */

(function(root, factory) {
  if (typeof define === 'function' && define.amd) {
    // AMD.
    define(['expect.js', process.cwd()+'/src/index'], factory);
  } else if (typeof module === 'object' && module.exports) {
    // CommonJS-like environments that support module.exports, like Node.
    factory(require('expect.js'), require(process.cwd()+'/src/index'));
  } else {
    // Browser globals (root is window)
    factory(root.expect, root.OpenServiceCloudApi);
  }
}(this, function(expect, OpenServiceCloudApi) {
  'use strict';

  var instance;

  beforeEach(function() {
    instance = new OpenServiceCloudApi.BandwidthResource();
  });

  var getProperty = function(object, getter, property) {
    // Use getter method if present; otherwise, get the property directly.
    if (typeof object[getter] === 'function')
      return object[getter]();
    else
      return object[property];
  }

  var setProperty = function(object, setter, property, value) {
    // Use setter method if present; otherwise, set the property directly.
    if (typeof object[setter] === 'function')
      object[setter](value);
    else
      object[property] = value;
  }

  describe('BandwidthResource', function() {
    it('should create an instance of BandwidthResource', function() {
      // uncomment below and update the code to test BandwidthResource
      //var instane = new OpenServiceCloudApi.BandwidthResource();
      //expect(instance).to.be.a(OpenServiceCloudApi.BandwidthResource);
    });

    it('should have the property name (base name: "name")', function() {
      // uncomment below and update the code to test the property name
      //var instane = new OpenServiceCloudApi.BandwidthResource();
      //expect(instance).to.be();
    });

    it('should have the property sharetype (base name: "sharetype")', function() {
      // uncomment below and update the code to test the property sharetype
      //var instane = new OpenServiceCloudApi.BandwidthResource();
      //expect(instance).to.be();
    });

    it('should have the property size (base name: "size")', function() {
      // uncomment below and update the code to test the property size
      //var instane = new OpenServiceCloudApi.BandwidthResource();
      //expect(instance).to.be();
    });

  });

}));
