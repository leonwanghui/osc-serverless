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
 * The CloudServerResourceFragmentFlavor model module.
 * @module model/CloudServerResourceFragmentFlavor
 * @version 0.0.3
 */
class CloudServerResourceFragmentFlavor {
    /**
     * Constructs a new <code>CloudServerResourceFragmentFlavor</code>.
     * @alias module:model/CloudServerResourceFragmentFlavor
     */
    constructor() { 
        
        CloudServerResourceFragmentFlavor.initialize(this);
    }

    /**
     * Initializes the fields of this object.
     * This method is used by the constructors of any subclasses, in order to implement multiple inheritance (mix-ins).
     * Only for internal use.
     */
    static initialize(obj) { 
    }

    /**
     * Constructs a <code>CloudServerResourceFragmentFlavor</code> from a plain JavaScript object, optionally creating a new instance.
     * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
     * @param {Object} data The plain JavaScript object bearing properties of interest.
     * @param {module:model/CloudServerResourceFragmentFlavor} obj Optional instance to populate.
     * @return {module:model/CloudServerResourceFragmentFlavor} The populated <code>CloudServerResourceFragmentFlavor</code> instance.
     */
    static constructFromObject(data, obj) {
        if (data) {
            obj = obj || new CloudServerResourceFragmentFlavor();

            if (data.hasOwnProperty('id')) {
                obj['id'] = ApiClient.convertToType(data['id'], 'String');
            }
            if (data.hasOwnProperty('name')) {
                obj['name'] = ApiClient.convertToType(data['name'], 'String');
            }
            if (data.hasOwnProperty('disk')) {
                obj['disk'] = ApiClient.convertToType(data['disk'], 'String');
            }
            if (data.hasOwnProperty('vcpus')) {
                obj['vcpus'] = ApiClient.convertToType(data['vcpus'], 'String');
            }
            if (data.hasOwnProperty('ram')) {
                obj['ram'] = ApiClient.convertToType(data['ram'], 'String');
            }
        }
        return obj;
    }


}

/**
 * @member {String} id
 */
CloudServerResourceFragmentFlavor.prototype['id'] = undefined;

/**
 * @member {String} name
 */
CloudServerResourceFragmentFlavor.prototype['name'] = undefined;

/**
 * @member {String} disk
 */
CloudServerResourceFragmentFlavor.prototype['disk'] = undefined;

/**
 * @member {String} vcpus
 */
CloudServerResourceFragmentFlavor.prototype['vcpus'] = undefined;

/**
 * @member {String} ram
 */
CloudServerResourceFragmentFlavor.prototype['ram'] = undefined;






export default CloudServerResourceFragmentFlavor;
