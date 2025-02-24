ENV ?= local

run:
	cd packages/$(SERVICE) && make run

clean:
	rm -rf .build/$(SERVICE)
