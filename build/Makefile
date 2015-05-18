build := $(CARGO_MANIFEST_DIR)/build
output := $(OUT_DIR)/lib
source := $(build)/3d-ice
patches := $(build)/patches
superlu := $(DEP_SUPERLU_ROOT)/include
cflags := -fPIC -MD -O3 -w

library := $(output)/libthreed-ice.a

all: $(library)

$(library): $(output)
	(cd $(source) && git apply $(patches)/*.patch)
	$(MAKE) -C $(source) lib 3DICE_LIB_A='$@' SLU_INCLUDE='$(superlu)' SLU_LIBS= CFLAGS='$(cflags)'
	(cd $(source) && git checkout .)

$(output):
	mkdir -p $(output)
