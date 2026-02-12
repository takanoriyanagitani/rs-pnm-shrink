iwasi=./rs-pnm-shrink.wasm

wasm-opt \
	-Oz \
	-o opt.wasm \
	--enable-simd \
	--enable-bulk-memory \
	--enable-nontrapping-float-to-int \
	"${iwasi}"
