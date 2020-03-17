build:
	LLVM_SYS_90_PREFIX=/usr/local/opt/llvm cargo build --bin=dreadnought --package=dreadnought

clean:
	rm -rf target