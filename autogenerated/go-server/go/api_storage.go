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

// StorageResourceCreate - create a new storage resource.
func StorageResourceCreate(w http.ResponseWriter, r *http.Request) {
	var req openapi.StorageResourceCreateRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		handleError(w, http.StatusBadRequest, err.Error())
		return
	}

	storage, err := ctr.CreateStorageResource(&req)
	if err != nil {
		handleError(w, http.StatusInternalServerError, err.Error())
		return
	}

	handleResult(w, http.StatusCreated, storage)
	return
}

// StorageResourceDelete - remove specified storage resource.
func StorageResourceDelete(w http.ResponseWriter, r *http.Request) {
	srID := mux.Vars(r)["sr_id"]
	cp := r.URL.Query().Get("cloud_provider")
	if srID == "" || cp == "" {
		handleError(w, http.StatusBadRequest,
			"url params 'sr_id' or 'cloud_provider' is missing")
		return
	}

	if err := ctr.DeleteStorageResource(srID); err != nil {
		handleError(w, http.StatusInternalServerError, err.Error())
		return
	}

	handleResult(w, http.StatusOK, emptyBody)
	return
}

// StorageResourceGet - get the information of storage resource.
func StorageResourceGet(w http.ResponseWriter, r *http.Request) {
	srID := mux.Vars(r)["sr_id"]
	cp := r.URL.Query().Get("cloud_provider")
	if srID == "" || cp == "" {
		handleError(w, http.StatusBadRequest,
			"url params 'sr_id' or 'cloud_provider' is missing")
		return
	}

	storage, err := ctr.GetStorageResource(srID)
	if err != nil {
		handleError(w, http.StatusNotFound, err.Error())
		return
	}

	handleResult(w, http.StatusOK, storage)
	return
}
