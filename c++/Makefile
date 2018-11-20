FOLDERS := $(wildcard day_*)

all: $(FOLDERS)

$(FOLDERS): 
	cd $@; g++ -std=c++1y test_main.cpp; ./a.out

.PHONY: all $(FOLDERS)

