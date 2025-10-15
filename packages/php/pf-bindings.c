#ifdef HAVE_CONFIG_H
#include "config.h"
#endif

#include "php.h"
#include "php_ini.h"
#include "ext/standard/info.h"
#include "ext/standard/basic_functions.h"
#include "zend_exceptions.h"
#include "pf_bindings.h"

#include <stdint.h>
#include <stdlib.h>

#include "../../crates/pf-bindings-c/bindings/pf_bindings.h"

#define PF_BINDINGS_NAME "pf-bindings"
#define PF_BINDINGS_VERSION "0.1.1"

/* True global resources - no need for thread safety here */
static int le_pf_bindings;

zend_class_entry *pf_bindings_exception_ce;

/* Define error constants */
#define PF_ERROR_INVALID_INPUT 1001
#define PF_ERROR_VERIFICATION_FAILED 1002
#define PF_ERROR_REGISTRATION_FAILED 1003
#define PF_ERROR_INTERNAL_ERROR 1004

static PHP_INI_MH(OnChangeCacheDir)
{
    return SUCCESS;
}

PHP_INI_BEGIN()
    PHP_INI_ENTRY("pf_bindings.cache_dir", "/tmp", PHP_INI_ALL, OnChangeCacheDir)
PHP_INI_END()

static void php_pf bindings_init_globals(zend_pf_bbindings_globals *pf bindings_globals)
{
}

/* {{{ proto bool pf_verify_bet(string receipt_json, string transcript_json)
   Verifies a bet receipt against its transcript */
PHP_FUNCTION(pf_verify_bet)
{
    char *receipt_json = NULL;
    size_t receipt_len;
    char *transcript_json = NULL;
    size_t transcript_len;
    
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "ss", &receipt_json, &receipt_len, &transcript_json, &transcript_len) == FAILURE) {
        RETURN_NULL();
    }
    
    struct PfError error;
    PfResult result = pf_verify_bet(receipt_json, transcript_json, &error);
    
    if (result == PfResult_Success) {
        RETURN_TRUE;
    } else {
        if (error.message != NULL) {
            zend_throw_exception(pf_bindings_exception_ce, error.message, (zend_long)result);
            pf_free_error(error);
        } else {
            zend_throw_exception(pf_bindings_exception_ce, "Verification failed", (zend_long)result);
        }
        RETURN_FALSE;
    }
}
/* }}} */

/* {{{ proto bool pf_register_gdp_package(string bytes)
   Registers a GDP package for use in betting operations */
PHP_FUNCTION(pf_register_gdp_package)
{
    zval *bytes_zval;
    char *bytes = NULL;
    size_t bytes_len;
    
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "s", &bytes, &bytes_len) == FAILURE) {
        RETURN_NULL();
    }
    
    struct PfError error;
    PfResult result = pf_register_gdp_package((const unsigned char*)bytes, bytes_len, &error);
    
    if (result == PfResult_Success) {
        RETURN_TRUE;
    } else {
        if (error.message != NULL) {
            zend_throw_exception(pf_bindings_exception_ce, error.message, (zend_long)result);
            pf_free_error(error);
        } else {
            zend_throw_exception(pf_bindings_exception_ce, "Registration failed", (zend_long)result);
        }
        RETURN_FALSE;
    }
}
/* }}} */

/* {{{ proto string pf_bindings_get_version()
   Get library version */
PHP_FUNCTION(pf_bindings_get_version)
{
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "") == FAILURE) {
        RETURN_NULL();
    }
    
    const char *version = pf_library_version();
    RETURN_STRING(version);
}
/* }}} */

/* {{{ pf_bindings_functions[]
 * Every user visible function must have an entry in pf_bindings_functions[].
 */
static const zend_function_entry pf_bindings_functions[] = {
    PHP_FE(pf_verify_bet,    NULL)
    PHP_FE(pf_register_gdp_package,  NULL)
    PHP_FE(pf_bindings_get_version,   NULL)
    PHP_FE_END    /* Must be the last line in pf_bindings_functions[] */
};
/* }}} */

/* {{{ PHP_MINIT_FUNCTION
 */
PHP_MINIT_FUNCTION(pf_bindings)
{
    /* Initialize exception class */
    zend_class_entry ce;
    INIT_CLASS_ENTRY(ce, "PfBindingsException", NULL);
    pf_bindings_exception_ce = zend_register_internal_class_ex(&ce, zend_exception_get_default());
    
    REGISTER_INI_ENTRIES();
    return SUCCESS;
}
/* }}} */

/* {{{ PHP_MSHUTDOWN_FUNCTION
 */
PHP_MSHUTDOWN_FUNCTION(pf_bindings)
{
    UNREGISTER_INI_ENTRIES();
    return SUCCESS;
}
/* }}} */

/* {{{ PHP_MINFO_FUNCTION
 */
PHP_MINFO_FUNCTION(pf_bindings)
{
    php_info_print_table_start();
    php_info_print_table_header(2, "pf-bindings support", "enabled");
    php_info_print_table_header(2, "pf-bindings version", PF_BINDINGS_VERSION);
    php_info_print_table_header(2, "Rust library version", pf_library_version());
    php_info_print_table_row(2, "pf-bindings author", "Probably Fair Foundation");
    php_info_print_table_end();

    DISPLAY_INI_ENTRIES();

    char *cache_dir = INI_STR("pf_bindings.cache_dir");
    if (cache_dir) {
        php_info_print_table_row(2, "Cache directory", cache_dir);
    }
}
/* }}} */

/* {{{ pf_bindings_module_entry
 */
zend_module_entry pf_bindings_module_entry = {
    STANDARD_MODULE_HEADER,
    "pf-bindings",
    pf_bindings_functions,
    PHP_MINIT(pf_bindings),
    PHP_MSHUTDOWN(pf_bindings),
    NULL,  // RINIT
    NULL,  // RSHUTDOWN
    PHP_MINFO(pf_bindings),
    PF_BINDINGS_VERSION,
    STANDARD_MODULE_PROPERTIES
};
/* }}} */

#ifdef COMPILE_DL_PF_BINDINGS
ZEND_GET_MODULE(pf_bindings)
#endif

/*
 * Local variables:
 * tab-width: 4
 * c-basic-offset: 4
 * End:
 * vim600: noet sw=4 ts=4 fdm=marker
 * vim<600: noet sw=4 ts=4
 */
