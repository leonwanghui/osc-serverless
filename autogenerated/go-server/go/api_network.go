/*
 * Open Service Cloud API
 *
 * Open Service Cloud API to manage different backend cloud services.
 *
 * API version: 0.0.1
 * Contact: wanghui71leon@gmail.com
 * Generated by: OpenAPI Generator (https://openapi-generator.tech)
 */

package openapi

import (
	"encoding/json"
	"net/http"

	"github.com/gorilla/mux"
	openapi "github.com/leonwanghui/osc-serverless/autogenerated/go-model"
)

// NetworkResourceCreate - create a new network resource.
func NetworkResourceCreate(w http.ResponseWriter, r *http.Request) {
	var req openapi.NetworkResourceCreateRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		handleError(w, http.StatusBadRequest, err.Error())
		return
	}

	network, err := ctr.CreateNetworkResource(&req)
	if err != nil {
		handleError(w, http.StatusInternalServerError, err.Error())
		return
	}

	handleResult(w, http.StatusCreated, network)
	return
}

// NetworkResourceDelete - remove specified network resource.
func NetworkResourceDelete(w http.ResponseWriter, r *http.Request) {
	nrID := mux.Vars(r)["nr_id"]
	cp := r.URL.Query().Get("cloud_provider")
	if nrID == "" || cp == "" {
		handleError(w, http.StatusBadRequest,
			"url params 'nr_id' or 'cloud_provider' is missing")
		return
	}

	if err := ctr.DeleteNetworkResource(nrID); err != nil {
		handleError(w, http.StatusInternalServerError, err.Error())
		return
	}

	handleResult(w, http.StatusOK, emptyBody)
	return
}

// NetworkResourceGet - get the information of network resource.
func NetworkResourceGet(w http.ResponseWriter, r *http.Request) {
	nrID := mux.Vars(r)["nr_id"]
	cp := r.URL.Query().Get("cloud_provider")
	if nrID == "" || cp == "" {
		handleError(w, http.StatusBadRequest,
			"url params 'nr_id' or 'cloud_provider' is missing")
		return
	}

	network, err := ctr.GetStorageResource(nrID)
	if err != nil {
		handleError(w, http.StatusNotFound, err.Error())
		return
	}

	handleResult(w, http.StatusOK, network)
	return
}