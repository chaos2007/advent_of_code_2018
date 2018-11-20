FOLDERS := $(wildcard day_*)

all: $(FOLDERS)

$(FOLDERS): 
	cd $@; g++ test_main.cpp; ./a.out

.PHONY: all $(FOLDERS)

