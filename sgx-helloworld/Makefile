SGX_SDK ?= /opt/sgxsdk
SGX_MODE ?= SIM
SGX_ARCH ?= x64
RUST_SGX_SDK ?= /opt/rust-sgx-sdk

include buildenv.mk

CUSTOM_LIBRARY_PATH := ./lib
CUSTOM_BIN_PATH := ./bin
CUSTOM_EDL_PATH := $(RUST_SGX_SDK)/edl
CUSTOM_COMMON_PATH := $(RUST_SGX_SDK)/common

ifeq ($(shell getconf LONG_BIT), 32)
	SGX_ARCH := x86
else ifeq ($(findstring -m32, $(CXXFLAGS)), -m32)
	SGX_ARCH := x86
endif

ifeq ($(SGX_ARCH), x86)
	SGX_COMMON_CFLAGS := -m32
	SGX_LIBRARY_PATH := $(SGX_SDK)/lib
	SGX_ENCLAVE_SIGNER := $(SGX_SDK)/bin/x86/sgx_sign
	SGX_EDGER8R := $(SGX_SDK)/bin/x86/sgx_edger8r
else
	SGX_COMMON_CFLAGS := -m64
	SGX_LIBRARY_PATH := $(SGX_SDK)/lib64
	SGX_ENCLAVE_SIGNER := $(SGX_SDK)/bin/x64/sgx_sign
	SGX_EDGER8R := $(SGX_SDK)/bin/x64/sgx_edger8r
endif

ifeq ($(SGX_DEBUG), 1)
	ifeq ($(SGX_PRERELEASE), 1)
		$(error Cannot set SGX_DEBUG and SGX_PRERELEASE at the same time!!)
	endif
endif

ifeq ($(SGX_DEBUG), 1)
	SGX_COMMON_CFLAGS += -O0 -g
else
	SGX_COMMON_CFLAGS += -O2
endif

SGX_COMMON_CFLAGS += -fstack-protector

######## Enclave Settings ########
Enclave_EDL_Files := enclave/Enclave_t.c enclave/Enclave_t.h
ifneq ($(SGX_MODE), HW)
	Trts_Library_Name := sgx_trts_sim
	Service_Library_Name := sgx_tservice_sim
else
	Trts_Library_Name := sgx_trts
	Service_Library_Name := sgx_tservice
endif
Crypto_Library_Name := sgx_tcrypto

RustEnclave_C_Files := $(wildcard ./enclave/*.c)
RustEnclave_C_Objects := $(RustEnclave_C_Files:.c=.o)
RustEnclave_Include_Paths := -I$(CUSTOM_COMMON_PATH)/inc -I$(CUSTOM_EDL_PATH) -I$(SGX_SDK)/include -I$(SGX_SDK)/include/tlibc -I$(SGX_SDK)/include/stlport -I$(SGX_SDK)/include/epid -I ./enclave -I./include

RustEnclave_Link_Libs := -L$(CUSTOM_LIBRARY_PATH) -lenclave
RustEnclave_Compile_Flags := $(SGX_COMMON_CFLAGS) $(ENCLAVE_CFLAGS) $(RustEnclave_Include_Paths)
RustEnclave_Link_Flags := -Wl,--no-undefined -nostdlib -nodefaultlibs -nostartfiles -L$(SGX_LIBRARY_PATH) \
-Wl,--whole-archive -l$(Trts_Library_Name) -Wl,--no-whole-archive \
-Wl,--start-group -lsgx_tstdc -l$(Service_Library_Name) -l$(Crypto_Library_Name) $(RustEnclave_Link_Libs) -Wl,--end-group \
-Wl,--version-script=enclave/Enclave.lds \
$(ENCLAVE_LDFLAGS)

RustEnclave_Name := enclave/enclave.so
Signed_RustEnclave_Name := bin/enclave.signed.so


.PHONY: all
all: $(Enclave_EDL_Files) app $(Signed_RustEnclave_Name)

$(Enclave_EDL_Files): $(SGX_EDGER8R) enclave/Enclave.edl
	$(SGX_EDGER8R) --trusted enclave/Enclave.edl --search-path $(SGX_SDK)/include --search-path $(CUSTOM_EDL_PATH) --trusted-dir enclave
	@echo "GEN  =>  $(Enclave_EDL_Files)"

enclave/Enclave_t.o: enclave/Enclave_t.c
	@$(CC) $(RustEnclave_Compile_Flags) -c $< -o $@
	@echo "CC   <=  $<"

$(RustEnclave_Name): enclave enclave/Enclave_t.o
	@$(CXX) enclave/Enclave_t.o -o $@ $(RustEnclave_Link_Flags)
	@echo "LINK =>  $@"

$(Signed_RustEnclave_Name): $(RustEnclave_Name)
	mkdir -p bin
	@$(SGX_ENCLAVE_SIGNER) sign -key enclave/Enclave_private.pem -enclave $(RustEnclave_Name) -out $@ -config enclave/Enclave.config.xml
	@echo "SIGN =>  $@"

.PHONY: app
app:
	$(MAKE) -C ./app

.PHONY: enclave
enclave:
	$(MAKE) -C ./enclave
