SERVICE ?= api
ENV ?= dev

WORKSPACE_ROOT ?= $(PWD)/../..
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
API_MODEL_ARTIFACT_DIR ?= $(PWD)/.build/api-models
CARGO_TARGET_DIR ?= $(PWD)/target
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)

BASE_DOMAIN ?= dagit.club
BUCKET_EXPIRE ?= 3600
ASSET_DIR ?= assets

RUST_LOG ?= debug

BUILD_ENV ?= ENV=$(ENV) RUST_LOG=$(RUST_LOG) VERSION=$(VERSION) COMMIT=$(COMMIT) DATABASE_TYPE=$(DATABASE_TYPE) DATABASE_URL=$(DATABASE_URL) API_MODEL_ARTIFACT_DIR=$(API_MODEL_ARTIFACT_DIR) CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) BUCKET_NAME=$(BASE_DOMAIN) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) ASSET_DIR=$(ASSET_DIR) BUCKET_EXPIRE=$(BUCKET_EXPIRE)

run:
	$(BUILD_ENV) cargo watch -x run -w src

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(CARGO_TARGET_DIR)/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap