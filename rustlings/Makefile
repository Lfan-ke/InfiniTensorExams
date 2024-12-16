name := #

rustlings := sudo /home/lfanke/rcore/rustlings/target/debug/rustlings

all: watch

watch:
	@${rustlings} watch

help:
	@${rustlings} hint ${NAME}

run:
	@${rustlings} run ${NAME}

list:
	@${rustlings} list

next:
	@${rustlings} run next

verify:
	@${rustlings} verify

init:
	@sudo cargo install --force --path .

commit:
	@git add .
	@git commit -m "update"
	@git push

code:
	@sudo code .

.PHONY: all watch help run list next verify init commit code
