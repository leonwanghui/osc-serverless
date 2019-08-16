BASE_DIR := $(shell pwd)
BUILD_DIR := $(BASE_DIR)/build

all: build

build: prebuild osc-runtime osc-manager

prebuild:
	mkdir -p $(BUILD_DIR)

osc-runtime:
	cd $(BASE_DIR)/osc-runtime && \
		cargo build --release && \
		cp ./target/release/osc_runtime $(BUILD_DIR)/bin/osc-rt

osc-manager:
	go build -ldflags '-w -s' -o $(BUILD_DIR)/bin/osc-mgr github.com/leonwanghui/osc-poc/osc-manager

autogenerated: validate generate

validate:
	docker run --rm -v $(BASE_DIR)/autogenerated:/local openapitools/openapi-generator-cli validate \
		-i /local/openapi.yaml

generate:
	cp $(BASE_DIR)/autogenerated/go-server/go/api_*.go $(BASE_DIR)/autogenerated/go-server/

	docker run --rm -v $(BASE_DIR)/autogenerated:/local openapitools/openapi-generator-cli generate \
    	-i /local/openapi.yaml \
		-g go-server \
		-o /local/go-server \
		--additional-properties hideGenerationTimestamp=true
	docker run --rm -v $(BASE_DIR)/autogenerated:/local openapitools/openapi-generator-cli generate \
        -i /local/openapi.yaml \
		-g rust \
		-o /local/rust-client \
		--additional-properties packageName=rust_client \
		--library=reqwest
	docker run --rm -v $(BASE_DIR)/autogenerated:/local openapitools/openapi-generator-cli generate \
        -i /local/openapi.yaml \
		-g javascript \
		-o /local/js-client

	mv $(BASE_DIR)/autogenerated/go-server/api_*.go $(BASE_DIR)/autogenerated/go-server/go/

	rm -f $(BASE_DIR)/autogenerated/go-model/*.go && \
		cp $(BASE_DIR)/autogenerated/go-server/go/model_*.go $(BASE_DIR)/autogenerated/go-model/ && \
		rm -r $(BASE_DIR)/autogenerated/go-server/go/model_*.go

goimports:
	goimports -w $(shell go list -f {{.Dir}} ./... |grep -v /vendor/)

clean:
	rm -rf $(BUILD_DIR)

.PHONY: osc-runtime osc-manager autogenerated goimports clean
