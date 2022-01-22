build:
	make git
	make publish_lib
	make update
	make git
	make publish
update:
	cargo update
publish:
	cargo release --execute
publish_lib:
	cd texcreate_lib && cargo publish
git:
	git add -A
	git commit -m "Update"
	git push