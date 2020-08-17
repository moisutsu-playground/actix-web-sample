ifneq (,$(wildcard ./.env))
    include .env
endif

ifndef PORT
	PORT=3000
endif

.PHONY: start
start:
	systemfd --no-pid -s http::$(PORT) -- cargo watch -x run
