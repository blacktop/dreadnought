build:
	LLVM_SYS_100_PREFIX=/usr/local/opt/llvm cargo build --bin=dreadnought --package=dreadnought

clean:
	rm -rf target