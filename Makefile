build:
	make update
	make git
	make publish_lib
	make publish
update:
	cargo update
publish:
	cargo publish
publish_lib:
	cd texcreate_lib && cargo publish
git:
	git add -A
	git commit -m "Update"
	git push