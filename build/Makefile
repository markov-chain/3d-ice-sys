build := $(CARGO_MANIFEST_DIR)/build
output := $(OUT_DIR)
source := $(build)/source
patches := $(build)/patches
superlu := $(DEP_SUPERLU_ROOT)/include
cflags := -fPIC -MD -O3 -w
library := libthreed-ice.a

all: $(output)/lib/$(library)

$(output)/lib/$(library):
	cp -R $(source)/* $(output)/
	(cd $(output); cat $(sort $(wildcard $(patches)/*.patch)) | patch -p1)
	$(MAKE) -C $(output) lib 3DICE_LIB_A='$@' SLU_INCLUDE='$(superlu)' SLU_LIBS= CFLAGS='$(cflags)'
