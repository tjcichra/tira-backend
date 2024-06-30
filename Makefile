# Check if podman-compose or docker-compose is available
PODMAN_COMPOSE := $(shell command -v podman-compose 2> /dev/null)
DOCKER_COMPOSE := $(shell command -v docker-compose 2> /dev/null)

# Set the compose command based on availability of podman-compose or docker-compose
ifdef PODMAN_COMPOSE
	COMPOSE := podman-compose
else
ifdef DOCKER_COMPOSE
	COMPOSE := docker-compose
else
$(error "Neither podman-compose nor docker-compose found. Please install either Podman or Docker.")
endif
endif

GIT_COMMIT:=$(shell git rev-parse --short=8 HEAD)

.PHONY: help
.ONESHELL:

help: ## This help.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help

build: ## Build tira
	$(COMPOSE) build

up: ## up tira
	@echo "Running dev copy of tira"
	$(COMPOSE) up -d

down: ## Down tira
	@echo "Stopping dev copy of tira"
	$(COMPOSE) down

logs: ## get logs
	$(COMPOSE) logs -f
