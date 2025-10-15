package pfbindings

/*
#cgo CFLAGS: -I../../crates/pf-bindings-go/bindings
#cgo LDFLAGS: -L../../../target/release -lpf_bindings_go -ldl -lm
#include <stdlib.h>
#include "pf_bindings.h"
*/
import "C"
import (
	"errors"
	"unsafe"
)

// VerifyBet verifies a bet receipt against its transcript
func VerifyBet(receiptJSON, transcriptJSON string) error {
	cReceipt := C.CString(receiptJSON)
	defer C.free(unsafe.Pointer(cReceipt))
	
	cTranscript := C.CString(transcriptJSON)
	defer C.free(unsafe.Pointer(cTranscript))
	
	var err *C.char
	result := C.pf_verify_bet(cReceipt, cTranscript, &err)
	defer C.pf_free_error(err)
	
	if result != 0 {
		if err != nil {
			errMsg := C.GoString(err)
			return errors.New("verification failed: " + errMsg)
		}
		return errors.New("verification failed")
	}
	
	return nil
}

// RegisterGdpPackage registers a GDP package for use in betting operations
func RegisterGdpPackage(data []byte) error {
	var cData *C.uchar
	if len(data) > 0 {
		cData = (*C.uchar)(unsafe.Pointer(&data[0]))
	}
	
	var err *C.char
	result := C.pf_register_gdp_package(cData, C.size_t(len(data)), &err)
	defer C.pf_free_error(err)
	
	if result != 0 {
		if err != nil {
			errMsg := C.GoString(err)
			return errors.New("registration failed: " + errMsg)
		}
		return errors.New("registration failed")
	}
	
	return nil
}
