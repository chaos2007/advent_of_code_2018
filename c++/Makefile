FOLDERS := $(wildcard day_*)

all: $(FOLDERS)

$(FOLDERS): 
	cd $@; g++ -std=c++14 test_main.cpp; ./a.out

.PHONY: all $(FOLDERS)

