RUSTC := rustc
RUSTDOC := rustdoc
BUILD := build
LIB := $(BUILD)/$(shell $(RUSTC) --crate-file-name src/lib.rs)
TEST := $(BUILD)/curltest

all: $(LIB)

-include $(BUILD)/curl.d
-include $(BUILD)/curltest.d

lib: $(LIB)

$(LIB): src/lib.rs
	@mkdir -p $(@D)
	$(RUSTC) $< --out-dir $(@D) --dep-info

tests: $(TEST) doctest
	$(TEST)

$(TEST): src/test.rs $(LIB)
	$(RUSTC) $< --test -o $@ --dep-info -L $(BUILD)

doc: $(LIB)
	$(RUSTDOC) -L $(BUILD) src/lib.rs

doctest: $(LIB)
	$(RUSTDOC) --test -L $(BUILD) src/lib.rs

clean:
	rm -rf $(BUILD)
