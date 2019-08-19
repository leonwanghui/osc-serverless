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
    instance = new OpenServiceCloudApi.CloudServerRequestFragmentPublicipEip();
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

  describe('CloudServerRequestFragmentPublicipEip', function() {
    it('should create an instance of CloudServerRequestFragmentPublicipEip', function() {
      // uncomment below and update the code to test CloudServerRequestFragmentPublicipEip
      //var instane = new OpenServiceCloudApi.CloudServerRequestFragmentPublicipEip();
      //expect(instance).to.be.a(OpenServiceCloudApi.CloudServerRequestFragmentPublicipEip);
    });

    it('should have the property ipType (base name: "ip_type")', function() {
      // uncomment below and update the code to test the property ipType
      //var instane = new OpenServiceCloudApi.CloudServerRequestFragmentPublicipEip();
      //expect(instance).to.be();
    });

    it('should have the property bandwidth (base name: "bandwidth")', function() {
      // uncomment below and update the code to test the property bandwidth
      //var instane = new OpenServiceCloudApi.CloudServerRequestFragmentPublicipEip();
      //expect(instance).to.be();
    });

  });

}));