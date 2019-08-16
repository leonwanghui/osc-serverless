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


import ApiClient from "../ApiClient";
import ComputeResourceCreateRequest from '../model/ComputeResourceCreateRequest';
import ComputeResourceSpec from '../model/ComputeResourceSpec';
import ErrorSpec from '../model/ErrorSpec';

/**
* Compute service.
* @module api/ComputeApi
* @version 0.0.3
*/
export default class ComputeApi {

    /**
    * Constructs a new ComputeApi. 
    * @alias module:api/ComputeApi
    * @class
    * @param {module:ApiClient} [apiClient] Optional API client implementation to use,
    * default to {@link module:ApiClient#instance} if unspecified.
    */
    constructor(apiClient) {
        this.apiClient = apiClient || ApiClient.instance;
    }


    /**
     * Callback function to receive the result of the computeResourceCreate operation.
     * @callback module:api/ComputeApi~computeResourceCreateCallback
     * @param {String} error Error message, if any.
     * @param {module:model/ComputeResourceSpec} data The data returned by the service call.
     * @param {String} response The complete HTTP response.
     */

    /**
     * create a new compute resource.
     * @param {module:model/ComputeResourceCreateRequest} computeResourceCreateRequest parameters for the requested compute resource.
     * @param {module:api/ComputeApi~computeResourceCreateCallback} callback The callback function, accepting three arguments: error, data, response
     * data is of type: {@link module:model/ComputeResourceSpec}
     */
    computeResourceCreate(computeResourceCreateRequest, callback) {
      let postBody = computeResourceCreateRequest;
      // verify the required parameter 'computeResourceCreateRequest' is set
      if (computeResourceCreateRequest === undefined || computeResourceCreateRequest === null) {
        throw new Error("Missing the required parameter 'computeResourceCreateRequest' when calling computeResourceCreate");
      }

      let pathParams = {
      };
      let queryParams = {
      };
      let headerParams = {
      };
      let formParams = {
      };

      let authNames = ['basicAuth'];
      let contentTypes = ['application/json'];
      let accepts = ['application/json'];
      let returnType = ComputeResourceSpec;
      return this.apiClient.callApi(
        '/v1alpha/compute_resources', 'POST',
        pathParams, queryParams, headerParams, formParams, postBody,
        authNames, contentTypes, accepts, returnType, null, callback
      );
    }

    /**
     * Callback function to receive the result of the computeResourceDelete operation.
     * @callback module:api/ComputeApi~computeResourceDeleteCallback
     * @param {String} error Error message, if any.
     * @param {Object} data The data returned by the service call.
     * @param {String} response The complete HTTP response.
     */

    /**
     * remove specified compute resource.
     * @param {String} crId uuid of compute resource created.
     * @param {module:model/String} cloudProvider cloud provider name
     * @param {Object} opts Optional parameters
     * @param {Boolean} opts.deletePublicip check if to delete public ip
     * @param {Boolean} opts.deleteVolume check if to delete related volume
     * @param {module:api/ComputeApi~computeResourceDeleteCallback} callback The callback function, accepting three arguments: error, data, response
     * data is of type: {@link Object}
     */
    computeResourceDelete(crId, cloudProvider, opts, callback) {
      opts = opts || {};
      let postBody = null;
      // verify the required parameter 'crId' is set
      if (crId === undefined || crId === null) {
        throw new Error("Missing the required parameter 'crId' when calling computeResourceDelete");
      }
      // verify the required parameter 'cloudProvider' is set
      if (cloudProvider === undefined || cloudProvider === null) {
        throw new Error("Missing the required parameter 'cloudProvider' when calling computeResourceDelete");
      }

      let pathParams = {
        'cr_id': crId
      };
      let queryParams = {
        'cloud_provider': cloudProvider,
        'delete_publicip': opts['deletePublicip'],
        'delete_volume': opts['deleteVolume']
      };
      let headerParams = {
      };
      let formParams = {
      };

      let authNames = ['basicAuth'];
      let contentTypes = [];
      let accepts = ['application/json'];
      let returnType = Object;
      return this.apiClient.callApi(
        '/v1alpha/compute_resources/{cr_id}', 'DELETE',
        pathParams, queryParams, headerParams, formParams, postBody,
        authNames, contentTypes, accepts, returnType, null, callback
      );
    }

    /**
     * Callback function to receive the result of the computeResourceGet operation.
     * @callback module:api/ComputeApi~computeResourceGetCallback
     * @param {String} error Error message, if any.
     * @param {module:model/ComputeResourceSpec} data The data returned by the service call.
     * @param {String} response The complete HTTP response.
     */

    /**
     * get the information of compute resource.
     * @param {String} crId uuid of compute resource created.
     * @param {module:model/String} cloudProvider cloud provider name
     * @param {module:api/ComputeApi~computeResourceGetCallback} callback The callback function, accepting three arguments: error, data, response
     * data is of type: {@link module:model/ComputeResourceSpec}
     */
    computeResourceGet(crId, cloudProvider, callback) {
      let postBody = null;
      // verify the required parameter 'crId' is set
      if (crId === undefined || crId === null) {
        throw new Error("Missing the required parameter 'crId' when calling computeResourceGet");
      }
      // verify the required parameter 'cloudProvider' is set
      if (cloudProvider === undefined || cloudProvider === null) {
        throw new Error("Missing the required parameter 'cloudProvider' when calling computeResourceGet");
      }

      let pathParams = {
        'cr_id': crId
      };
      let queryParams = {
        'cloud_provider': cloudProvider
      };
      let headerParams = {
      };
      let formParams = {
      };

      let authNames = ['basicAuth'];
      let contentTypes = [];
      let accepts = ['application/json'];
      let returnType = ComputeResourceSpec;
      return this.apiClient.callApi(
        '/v1alpha/compute_resources/{cr_id}', 'GET',
        pathParams, queryParams, headerParams, formParams, postBody,
        authNames, contentTypes, accepts, returnType, null, callback
      );
    }


}
