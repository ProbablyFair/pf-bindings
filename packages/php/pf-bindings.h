#ifndef PF_BINDINGS_H
#define PF_BINDINGS_H

extern zend_module_entry pf_bindings_module_entry;
#define phpext_pf_bindings_ptr &pf_bindings_module_entry

#define PHP_PF_BINDINGS_VERSION "0.1.1"

#ifdef PHP_WIN32
# define PHP_PF_BINDINGS_API __declspec(dllexport)
#else
# define PHP_PF_BINDINGS_API
#endif

#define PHP_PF_BINDINGS_MAX_STR_LEN 1024

#endif /* PF_BINDINGS_H */
