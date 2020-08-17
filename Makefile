ifneq (,$(wildcard ./.env))
    include .env
endif

ifndef PORT
	PORT=3000
endif

export RUSTC_WRAPPER=$(which sccache)

.PHONY: start
start:
	systemfd --no-pid -s http::$(PORT) -- cargo watch -x run
