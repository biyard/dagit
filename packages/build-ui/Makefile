SERVICE ?= build-ui
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local
HOST ?= 0.0.0.0:8080
PROFILE ?= default
WORKSPACE_ROOT ?= ../..
ARTIFACT_DIR ?= $(WORKSPACE_ROOT)/.build/$(SERVICE)

ifneq ("$(PROFILE)","default")
	AWS_FLAG += --profile $(PROFILE)
endif

API_ENDPOINT ?= https://api.dev.dagit.club
ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
RUSTFLAGS ?= -D warnings
RUST_LOG ?= debug
BASE_URL ?= https://dev.dagit.club
CARGO_TARGET_DIR ?= $(PWD)/target

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) RUST_LOG=$(RUST_LOG) AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} TABLE_NAME=$(VOICE_KOREA_TABLE_NAME) BASE_URL=$(BASE_URL) API_URL=${API_URL} RUSTFLAGS="${RUSTFLAGS}"

setup.tool:
	cargo binstall dioxus-cli
	cargo binstall toml-cli

run: clean public
	@CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) $(BUILD_ENV) dx serve --fullstack $(DXFLAGS)

build: clean public
	@$(BUILD_ENV) dx build --release --fullstack --server-features lambda
	cp -r $(WORKSPACE_ROOT)/target/dx/$(SERVICE)/release/web $(ARTIFACT_DIR)
	mv $(ARTIFACT_DIR)/server $(ARTIFACT_DIR)/bootstrap

.PHONY: public
public: node_modules public/tailwind.css 

node_modules:
	npm i


public/tailwind.css:
	npx tailwindcss -i ./public/input.css -o ./public/tailwind.css

clean:
	rm -rf public/tailwind.css 

build-docker: clean public/tailwind.css
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) biyard/dioxus-docker bash -c 'source ~/.cargo/env && $(BUILD_ENV) dx build --release --fullstack --server-features lambda && cp -r /app/target/dx/$(SERVICE)/release/web /app/.build/$(SERVICE) && mv /app/.build/$(SERVICE)/server /app/.build/$(SERVICE)/bootstrap'
