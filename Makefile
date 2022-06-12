java_run: lib
	javac -h . *.java && java -Djava.library.path=mylib/target/debug/ HelloWorld

.PHONY: lib

javah:
	javah HelloWorld

lib:
	cd mylib && cargo build
