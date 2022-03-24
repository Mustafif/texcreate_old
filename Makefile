build:
	cd tex-rs && cargo publish
	cd texc-latex && cargo publish
	cd texc-config && cargo publish
	cd texc-utils && cargo publish
	cd texc-web && cargo publish