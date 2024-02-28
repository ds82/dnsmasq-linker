.PHONY: clean-build-image release 

IMAGE := dnsmasq-linker
BUILD_IMAGE_TAG := $(IMAGE):build
PLATFORM := linux/amd64

clean-build-image:
	@docker rmi -f $(BUILD_IMAGE_TAG) 2>/dev/null || true

release: clean-build-image
	@docker build \
		--build-arg "RUST_VERSION=1.75.0" \
		--build-arg "APP=dnsmasq-linker" \
		--platform=$(PLATFORM) \
		-t dnsmasq-linker:latest \
		. 

