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

import ApiClient from '../ApiClient';

/**
 * The ErrorSpec model module.
 * @module model/ErrorSpec
 * @version 0.0.3
 */
class ErrorSpec {
    /**
     * Constructs a new <code>ErrorSpec</code>.
     * Detailed HTTP error response, which consists of a HTTP status code, and a custom error message unique for each failure case.
     * @alias module:model/ErrorSpec
     * @param code {Number} 
     * @param message {String} 
     */
    constructor(code, message) { 
        
        ErrorSpec.initialize(this, code, message);
    }

    /**
     * Initializes the fields of this object.
     * This method is used by the constructors of any subclasses, in order to implement multiple inheritance (mix-ins).
     * Only for internal use.
     */
    static initialize(obj, code, message) { 
        obj['code'] = code;
        obj['message'] = message;
    }

    /**
     * Constructs a <code>ErrorSpec</code> from a plain JavaScript object, optionally creating a new instance.
     * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
     * @param {Object} data The plain JavaScript object bearing properties of interest.
     * @param {module:model/ErrorSpec} obj Optional instance to populate.
     * @return {module:model/ErrorSpec} The populated <code>ErrorSpec</code> instance.
     */
    static constructFromObject(data, obj) {
        if (data) {
            obj = obj || new ErrorSpec();

            if (data.hasOwnProperty('code')) {
                obj['code'] = ApiClient.convertToType(data['code'], 'Number');
            }
            if (data.hasOwnProperty('message')) {
                obj['message'] = ApiClient.convertToType(data['message'], 'String');
            }
        }
        return obj;
    }


}

/**
 * @member {Number} code
 */
ErrorSpec.prototype['code'] = undefined;

/**
 * @member {String} message
 */
ErrorSpec.prototype['message'] = undefined;






export default ErrorSpec;
