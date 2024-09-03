TARGET = test-c-project-for-rust
DEBUG = 0
OPT = -Og
BUILD_DIR = build

C_LIB_SOURCES = \
lib/lib_example.c \

C_APP_SOURCES = \
source/app/main.c \
source/app/app_example.c \

C_UTIL_SOURCES = \
source/util/util_example.c \

C_SOURCES = $(C_LIB_SOURCES)
C_SOURCES += $(C_APP_SOURCES)
C_SOURCES += $(C_UTIL_SOURCES)

CC = gcc

HEX = $(CP) -O ihex
BIN = $(CP) -O binary -S

# C includes
C_INCLUDES =  \
-Ilib \
-Isource/app \
-Isource/util \
-isystem /usr/include \

CFLAGS = $(C_INCLUDES) -std=c99 -Wall -Wextra -Wconversion -fdata-sections -ffunction-sections -Wno-missing-field-initializers -Wno-missing-braces 

ifeq ($(DEBUG), 1)
CFLAGS += -g -gdwarf-2
endif

LDFLAGS = -lc -lm

all: $(BUILD_DIR)/$(TARGET)

OBJECTS = $(addprefix $(BUILD_DIR)/,$(notdir $(C_SOURCES:.c=.o)))
vpath %.c $(sort $(dir $(C_SOURCES)))

$(BUILD_DIR)/%.o: %.c Makefile | $(BUILD_DIR) 
	$(CC) -c $(CFLAGS) -Wa,-a,-ad,-alms=$(BUILD_DIR)/$(notdir $(<:.c=.lst)) $< -o $@

$(BUILD_DIR)/$(TARGET): $(OBJECTS) Makefile
	$(CC) $(OBJECTS) $(LDFLAGS) -o $@
	
$(BUILD_DIR):
	mkdir $@		

clean:
	rm -fR $(BUILD_DIR)

-include $(wildcard $(BUILD_DIR)/*.d)

