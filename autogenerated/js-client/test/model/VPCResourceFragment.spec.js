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
    instance = new OpenServiceCloudApi.VPCResourceFragment();
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

  describe('VPCResourceFragment', function() {
    it('should create an instance of VPCResourceFragment', function() {
      // uncomment below and update the code to test VPCResourceFragment
      //var instane = new OpenServiceCloudApi.VPCResourceFragment();
      //expect(instance).to.be.a(OpenServiceCloudApi.VPCResourceFragment);
    });

    it('should have the property cidr (base name: "cidr")', function() {
      // uncomment below and update the code to test the property cidr
      //var instane = new OpenServiceCloudApi.VPCResourceFragment();
      //expect(instance).to.be();
    });

    it('should have the property routes (base name: "routes")', function() {
      // uncomment below and update the code to test the property routes
      //var instane = new OpenServiceCloudApi.VPCResourceFragment();
      //expect(instance).to.be();
    });

  });

}));
