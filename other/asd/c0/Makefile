### CONFIG ###

PROJECT = main
.DEFAULT_GOAL := main_

CC = g++
CFLAGS += --std=c++17
HEADERSUFFIX = .hpp
SOURCESUFFIX = .cpp

### MY SETUP ###
main_: $(PROJECT)

IGNORE_FILES = list.cpp main.cpp unit_list.cpp
DEPS += list.cpp main.cpp unit_list.cpp

DEPS += $(addsuffix $(HEADERSUFFIX), $(PROJECT))
FILES += $(filter-out $(IGNORE_FILES), $(wildcard $(addsuffix $(SOURCESUFFIX), *)))

### GTEST CONFIG ###

GTEST_DIR = /opt/gtest/googletest
CPPFLAGS += -isystem $(GTEST_DIR)/include
CXXFLAGS += -g -Wall -Wextra -pthread
USER_DIR = .

CFLAGS += -Wno-unknown-pragmas

### GOOGLETEST SETUP ###

GTEST_HEADERS = $(GTEST_DIR)/include/gtest/*.h \
				$(GTEST_DIR)/include/gtest/internal/*.h
GTEST_SRCS_ = $(GTEST_DIR)/src/*.cc $(GTEST_DIR)/src/*.h $(GTEST_HEADERS)

gtest-all.o : $(GTEST_SRCS_)
	$(CC) $(CPPFLAGS) -I$(GTEST_DIR) $(CXXFLAGS) -c \
			$(GTEST_DIR)/src/gtest-all.cc

gtest_main.o : $(GTEST_SRCS_)
	$(CC) $(CPPFLAGS) -I$(GTEST_DIR) $(CXXFLAGS) -c \
			$(GTEST_DIR)/src/gtest_main.cc

gtest.a : gtest-all.o
	$(AR) $(ARFLAGS) $@ $^

gtest_main.a : gtest-all.o gtest_main.o
	$(AR) $(ARFLAGS) $@ $^

### MAIN ###

$(PROJECT) : $(FILES) $(DEPS) gtest_main.a
	$(CC) $(CPPFLAGS) $(CXXFLAGS) $(CFLAGS) -lpthread $(FILES) gtest_main.a -o $@.out
	@./$@.out
	@printf "\nMemleaks:\n"
	@valgrind --leak-check=yes --log-fd=1 ./$@.out | sed -e '1,/LEAK SUMMARY/d' |\
	 head -n 3 | awk '{$$1= ""; print "    ", substr($$0,2);}'
