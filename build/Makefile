build := $(CARGO_MANIFEST_DIR)/build
output := $(OUT_DIR)
source := $(build)/source
patches := $(build)/patches
superlu := $(DEP_SUPERLU_ROOT)/include
cflags := -fPIC -MD -O3 -w
library := lib/libthreed-ice.a

all: $(output)/$(library)

$(output)/$(library): $(source)/$(library)
	mkdir -p $(dir $@)
	cp $< $@

$(source)/$(library):
	(cd $(source); cat $(sort $(wildcard $(patches)/*.patch)) | patch -p1)
	$(MAKE) -C $(source) lib 3DICE_LIB_A='$@' SLU_INCLUDE='$(superlu)' SLU_LIBS= CFLAGS='$(cflags)'
