RUSTC := rustc
RUSTDOC := rustdoc
BUILD := build
LIB := $(addprefix $(addsuffix /,$(BUILD)),$(shell $(RUSTC) --crate-file-name src/lib.rs))
TEST := $(BUILD)/curltest
LIB_DEP_INFO = $(BUILD)/curl.d
TEST_DEP_INFO = $(BUILD)/curltest.d

all: $(LIB)

-include $(LIB_DEP_INFO)
-include $(TEST_DEP_INFO)

lib: $(LIB)

$(LIB): src/lib.rs
	@mkdir -p $(@D)
	echo $(LIB)
	$(RUSTC) $< --out-dir $(@D) --dep-info $(LIB_DEP_INFO)

tests: $(TEST) doctest
	$(TEST)

$(TEST): src/test.rs $(LIB)
	$(RUSTC) $< --test -o $@ --dep-info $(TEST_DEP_INFO) -L $(BUILD)

doc: $(LIB)
	$(RUSTDOC) -L $(BUILD) src/lib.rs

doctest: $(LIB)
	$(RUSTDOC) --test -L $(BUILD) src/lib.rs

clean:
	rm -rf $(BUILD)
