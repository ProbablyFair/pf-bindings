PHP_ARG_ENABLE(pf-bindings, whether to enable pf-bindings support,
[  --enable-pf-bindings   Enable pf-bindings support])

if test "$PHP_PF_BINDINGS" = "yes"; then
  AC_DEFINE(HAVE_PF_BINDINGS, 1, [Whether you have pf-bindings])
  PHP_NEW_EXTENSION(pf-bindings, pf-bindings.c, $ext_shared)
  PHP_SUBST(PF_BINDINGS_SHARED_LIBADD)
  
  # Link against the Rust library
  PHP_ADD_LIBRARY(pf_bindings_php, 1, PF_BINDINGS_SHARED_LIBADD)
  
  # Include path for headers
  PHP_ADD_INCLUDE($ext_srcdir/../../crates/pf-bindings-c/bindings)
fi
