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
 * The CloudServerRequestFragmentNics model module.
 * @module model/CloudServerRequestFragmentNics
 * @version 0.0.3
 */
class CloudServerRequestFragmentNics {
    /**
     * Constructs a new <code>CloudServerRequestFragmentNics</code>.
     * @alias module:model/CloudServerRequestFragmentNics
     * @param subnetId {String} 
     */
    constructor(subnetId) { 
        
        CloudServerRequestFragmentNics.initialize(this, subnetId);
    }

    /**
     * Initializes the fields of this object.
     * This method is used by the constructors of any subclasses, in order to implement multiple inheritance (mix-ins).
     * Only for internal use.
     */
    static initialize(obj, subnetId) { 
        obj['subnet_id'] = subnetId;
    }

    /**
     * Constructs a <code>CloudServerRequestFragmentNics</code> from a plain JavaScript object, optionally creating a new instance.
     * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
     * @param {Object} data The plain JavaScript object bearing properties of interest.
     * @param {module:model/CloudServerRequestFragmentNics} obj Optional instance to populate.
     * @return {module:model/CloudServerRequestFragmentNics} The populated <code>CloudServerRequestFragmentNics</code> instance.
     */
    static constructFromObject(data, obj) {
        if (data) {
            obj = obj || new CloudServerRequestFragmentNics();

            if (data.hasOwnProperty('subnet_id')) {
                obj['subnet_id'] = ApiClient.convertToType(data['subnet_id'], 'String');
            }
        }
        return obj;
    }


}

/**
 * @member {String} subnet_id
 */
CloudServerRequestFragmentNics.prototype['subnet_id'] = undefined;






export default CloudServerRequestFragmentNics;

