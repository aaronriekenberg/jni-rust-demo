java_run: lib
	javac -h . HelloWorld.java && java -Djava.library.path=mylib/target/debug/ HelloWorld

.PHONY: lib

javah:
	javah HelloWorld

lib:
	cd mylib && cargo build
